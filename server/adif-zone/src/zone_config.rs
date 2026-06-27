use sqlx::PgPool;

#[derive(Debug, sqlx::FromRow)]
pub struct ZoneConfig {
    pub id: i32,
    pub zoneidnumber: i32,
    pub short_name: String,
    pub long_name: String,
    pub safe_x: f32,
    pub safe_y: f32,
    pub safe_z: f32,
    pub safe_heading: f32,
    pub min_level: i16,
    pub max_level: i16,
    pub min_clip: f32,
    pub max_clip: f32,
    pub underworld: f32,
    pub walkspeed: f32,
    pub zone_exp_multiplier: f32,
    pub can_bind: i16,
    pub can_combat: i16,
    pub can_levitate: i16,
}

impl ZoneConfig {
    pub fn bindable(&self) -> bool {
        self.can_bind != 0
    }

    pub fn combatable(&self) -> bool {
        self.can_combat != 0
    }

    pub fn levitable(&self) -> bool {
        self.can_levitate != 0
    }

    pub async fn load_all(pool: &PgPool) -> anyhow::Result<Vec<Self>> {
        let zones = sqlx::query_as::<_, ZoneConfig>(
            "SELECT id, zoneidnumber, short_name, long_name, \
             safe_x, safe_y, safe_z, safe_heading, \
             min_level, max_level, \
             minclip AS min_clip, maxclip AS max_clip, \
             underworld, walkspeed, \
             zone_exp_multiplier::real AS zone_exp_multiplier, \
             canbind AS can_bind, \
             cancombat AS can_combat, \
             canlevitate AS can_levitate \
             FROM zone \
             WHERE short_name IS NOT NULL \
             ORDER BY zoneidnumber",
        )
        .fetch_all(pool)
        .await?;

        Ok(zones)
    }
}
