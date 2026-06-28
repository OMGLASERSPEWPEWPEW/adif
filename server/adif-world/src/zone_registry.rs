use std::collections::HashMap;
use std::net::SocketAddr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZoneState {
    Booting,
    Running,
    ShuttingDown,
}

#[derive(Debug, Clone)]
pub struct ZoneInstance {
    pub zone_id: i32,
    pub zone_short_name: String,
    pub addr: SocketAddr,
    pub player_count: u32,
    pub state: ZoneState,
}

pub struct ZoneRegistry {
    zones: HashMap<i32, ZoneInstance>,
}

impl ZoneRegistry {
    pub fn new() -> Self {
        Self {
            zones: HashMap::new(),
        }
    }

    pub fn register(&mut self, zone_id: i32, zone_short_name: String, addr: SocketAddr) {
        tracing::info!(zone_id, zone = %zone_short_name, addr = %addr, "Zone registered");
        self.zones.insert(
            zone_id,
            ZoneInstance {
                zone_id,
                zone_short_name,
                addr,
                player_count: 0,
                state: ZoneState::Running,
            },
        );
    }

    pub fn unregister(&mut self, zone_id: i32) {
        if let Some(z) = self.zones.remove(&zone_id) {
            tracing::info!(zone_id, zone = %z.zone_short_name, "Zone unregistered");
        }
    }

    pub fn find_by_zone_id(&self, zone_id: i32) -> Option<&ZoneInstance> {
        self.zones.get(&zone_id)
    }

    pub fn find_by_name(&self, short_name: &str) -> Option<&ZoneInstance> {
        self.zones.values().find(|z| z.zone_short_name == short_name)
    }

    pub fn all_zones(&self) -> Vec<&ZoneInstance> {
        self.zones.values().collect()
    }

    pub fn zone_count(&self) -> usize {
        self.zones.len()
    }
}
