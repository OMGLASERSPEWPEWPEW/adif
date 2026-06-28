use crate::WorldState;

#[derive(Debug, Clone)]
pub struct ZoneRouteInfo {
    pub ip: String,
    pub port: u16,
    pub zone_id: i32,
    pub zone_short_name: String,
}

pub async fn resolve_zone(state: &WorldState, zone_id: i32) -> Option<ZoneRouteInfo> {
    let registry = state.zone_registry.read().await;
    let instance = registry.find_by_zone_id(zone_id)?;

    Some(ZoneRouteInfo {
        ip: instance.addr.ip().to_string(),
        port: instance.addr.port(),
        zone_id: instance.zone_id,
        zone_short_name: instance.zone_short_name.clone(),
    })
}

pub fn build_zone_server_info_bytes(route: &ZoneRouteInfo) -> [u8; 130] {
    let mut buf = [0u8; 130];
    let ip_bytes = route.ip.as_bytes();
    let len = ip_bytes.len().min(127);
    buf[..len].copy_from_slice(&ip_bytes[..len]);
    buf[128] = (route.port & 0xFF) as u8;
    buf[129] = (route.port >> 8) as u8;
    buf
}
