use std::path::PathBuf;

use anyhow::Context;
use bevy_ecs::prelude::*;
use tracing::{info, warn};

mod ecs;
mod zone_config;

use ecs::EntityIdAllocator;
use zone_config::ZoneConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config_path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("server.toml"));

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
    info!(count = zones.len(), "Loaded zone configs from database");

    if zones.is_empty() {
        warn!("No zones found in database — is the zones table populated?");
    } else {
        for zone in zones.iter().take(5) {
            info!(
                id = zone.zoneidnumber,
                short_name = %zone.short_name,
                long_name = %zone.long_name,
                "  zone"
            );
        }
        if zones.len() > 5 {
            info!("  ... and {} more", zones.len() - 5);
        }
    }

    let mut world = World::new();
    world.insert_resource(EntityIdAllocator::new());
    info!("ECS world initialized");

    info!(
        zones = zones.len(),
        "Zone server boot complete"
    );

    Ok(())
}
