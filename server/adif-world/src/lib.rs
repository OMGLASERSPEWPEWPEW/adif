pub mod account;
pub mod character;
pub mod zone_registry;
pub mod zone_routing;

use sqlx::PgPool;
use tokio::sync::RwLock;

pub struct WorldState {
    pub pool: PgPool,
    pub zone_registry: RwLock<zone_registry::ZoneRegistry>,
    pub motd: String,
    pub server_name: String,
}

impl WorldState {
    pub fn new(pool: PgPool, server_name: String, motd: String) -> Self {
        Self {
            pool,
            zone_registry: RwLock::new(zone_registry::ZoneRegistry::new()),
            motd,
            server_name,
        }
    }
}
