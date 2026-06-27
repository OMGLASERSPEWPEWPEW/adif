use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Context;
use bevy_ecs::prelude::*;
use tokio::sync::Mutex;
use tracing::info;

mod ai;
mod chat;
mod combat;
mod ecs;
mod game_loop;
mod geometry;
mod movement;
mod network;
mod spawn;
mod zone_config;
mod zone_transition;

use ecs::EntityIdAllocator;
use network::session::SessionManager;
use zone_config::ZoneConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config_path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("server.toml"));

    let zone_short_name = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "grobb".to_string());

    let config = adif_common::ServerConfig::load(&config_path)
        .with_context(|| format!("Failed to load config from {}", config_path.display()))?;

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| config.server.log_level.parse().unwrap_or_default()),
        )
        .init();

    info!(name = %config.server.name, "ADIF Zone Server starting");

    let pool = adif_common::create_pool(&config.database)
        .await
        .context("Failed to connect to PostgreSQL")?;

    info!("Connected to PostgreSQL at {}:{}", config.database.host, config.database.port);

    let zones = ZoneConfig::load_all(&pool).await?;
    let zone = zones
        .iter()
        .find(|z| z.short_name == zone_short_name)
        .ok_or_else(|| anyhow::anyhow!("Zone '{}' not found in database", zone_short_name))?;

    info!(
        zone = %zone.short_name,
        long_name = %zone.long_name,
        id = zone.zoneidnumber,
        "Booting zone"
    );

    let mut world = World::new();
    world.insert_resource(EntityIdAllocator::new());
    world.insert_resource(geometry::ZoneGeometry::flat_plane());

    let result = spawn::resolver::load_and_spawn(&pool, &zone_short_name, &mut world).await?;

    let zone_points = zone_transition::ZonePoint::load_for_zone(&pool, &zone_short_name).await?;
    let zone_lines = zone_transition::ZoneLines::new(zone_points);
    info!(zone_points = zone_lines.point_count(), "Loaded zone lines");
    world.insert_resource(zone_lines);

    info!(
        npcs = result.npcs_spawned,
        spawn_points = result.spawn_points_total,
        "Zone populated"
    );

    let listen_port = config.server.listen_port;
    let sessions = Arc::new(Mutex::new(SessionManager::new()));
    let shared_world = Arc::new(Mutex::new(world));

    // Start TCP listener in background
    let net_sessions = Arc::clone(&sessions);
    let net_world = Arc::clone(&shared_world);
    tokio::spawn(async move {
        if let Err(e) = network::server::start_listener(listen_port, net_sessions, net_world).await {
            tracing::error!(error = %e, "TCP listener failed");
        }
    });

    // Run the game loop (pass --duration N to stop after N seconds, else runs forever)
    let duration = std::env::args()
        .position(|a| a == "--duration")
        .and_then(|i| std::env::args().nth(i + 1))
        .and_then(|s| s.parse::<u64>().ok())
        .map(Duration::from_secs);

    let mut world = shared_world.lock().await;
    game_loop::run_loop(&mut world, duration).await;

    Ok(())
}
