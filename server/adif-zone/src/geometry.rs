use bevy_ecs::prelude::*;

pub trait ZoneMap: Send + Sync {
    fn best_z(&self, x: f32, y: f32, z: f32) -> Option<f32>;
    fn line_of_sight(&self, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> bool;
    fn check_collision(&self, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> Option<(f32, f32, f32)>;
}

pub trait WaterMap: Send + Sync {
    fn in_water(&self, x: f32, y: f32, z: f32) -> bool;
    fn in_lava(&self, x: f32, y: f32, z: f32) -> bool;
    fn water_surface(&self, x: f32, y: f32) -> Option<f32>;
}

pub trait Pathfinder: Send + Sync {
    fn find_path(&self, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> Option<Vec<(f32, f32, f32)>>;
    fn has_path(&self, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> bool {
        self.find_path(x1, y1, z1, x2, y2, z2).is_some()
    }
}

pub struct FlatPlaneMap;

impl ZoneMap for FlatPlaneMap {
    fn best_z(&self, _x: f32, _y: f32, _z: f32) -> Option<f32> {
        Some(0.0)
    }

    fn line_of_sight(&self, _x1: f32, _y1: f32, _z1: f32, _x2: f32, _y2: f32, _z2: f32) -> bool {
        true
    }

    fn check_collision(&self, _x1: f32, _y1: f32, _z1: f32, _x2: f32, _y2: f32, _z2: f32) -> Option<(f32, f32, f32)> {
        None
    }
}

pub struct NullWaterMap;

impl WaterMap for NullWaterMap {
    fn in_water(&self, _x: f32, _y: f32, _z: f32) -> bool { false }
    fn in_lava(&self, _x: f32, _y: f32, _z: f32) -> bool { false }
    fn water_surface(&self, _x: f32, _y: f32) -> Option<f32> { None }
}

pub struct DirectPathfinder;

impl Pathfinder for DirectPathfinder {
    fn find_path(&self, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> Option<Vec<(f32, f32, f32)>> {
        Some(vec![(x1, y1, z1), (x2, y2, z2)])
    }
}

#[derive(Resource)]
pub struct ZoneGeometry {
    pub map: Box<dyn ZoneMap>,
    pub water: Box<dyn WaterMap>,
    pub pathfinder: Box<dyn Pathfinder>,
}

impl ZoneGeometry {
    pub fn flat_plane() -> Self {
        Self {
            map: Box::new(FlatPlaneMap),
            water: Box::new(NullWaterMap),
            pathfinder: Box::new(DirectPathfinder),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flat_plane_always_zero() {
        let map = FlatPlaneMap;
        assert_eq!(map.best_z(100.0, 200.0, 50.0), Some(0.0));
        assert!(map.line_of_sight(0.0, 0.0, 0.0, 100.0, 100.0, 0.0));
        assert!(map.check_collision(0.0, 0.0, 0.0, 100.0, 100.0, 0.0).is_none());
    }

    #[test]
    fn null_water_map_dry() {
        let water = NullWaterMap;
        assert!(!water.in_water(0.0, 0.0, 0.0));
        assert!(!water.in_lava(0.0, 0.0, 0.0));
        assert_eq!(water.water_surface(0.0, 0.0), None);
    }

    #[test]
    fn direct_pathfinder_returns_line() {
        let pf = DirectPathfinder;
        let path = pf.find_path(0.0, 0.0, 0.0, 100.0, 100.0, 0.0).unwrap();
        assert_eq!(path.len(), 2);
        assert!(pf.has_path(0.0, 0.0, 0.0, 100.0, 100.0, 0.0));
    }

    #[test]
    fn zone_geometry_resource() {
        let geo = ZoneGeometry::flat_plane();
        assert_eq!(geo.map.best_z(0.0, 0.0, 0.0), Some(0.0));
        assert!(!geo.water.in_water(0.0, 0.0, 0.0));
        assert!(geo.pathfinder.has_path(0.0, 0.0, 0.0, 1.0, 1.0, 0.0));
    }
}
