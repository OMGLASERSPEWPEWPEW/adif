use sqlx::PgPool;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SpawnPoint {
    pub id: i32,
    pub spawngroupid: i32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub heading: f32,
    pub respawntime: i32,
    pub variance: i32,
    pub pathgrid: i32,
    pub animation: i32,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SpawnEntry {
    pub spawngroupid: i32,
    pub npcid: i32,
    pub chance: i32,
}

impl SpawnPoint {
    pub async fn load_for_zone(pool: &PgPool, zone_short_name: &str) -> anyhow::Result<Vec<Self>> {
        let points = sqlx::query_as::<_, SpawnPoint>(
            "SELECT id, spawngroupid, x, y, z, heading, respawntime, \
             variance, pathgrid, animation \
             FROM spawn2 \
             WHERE zone = $1 AND version = 0",
        )
        .bind(zone_short_name)
        .fetch_all(pool)
        .await?;

        Ok(points)
    }
}

impl SpawnEntry {
    pub async fn load_for_zone(pool: &PgPool, zone_short_name: &str) -> anyhow::Result<Vec<Self>> {
        let entries = sqlx::query_as::<_, SpawnEntry>(
            "SELECT se.spawngroupid, se.npcid, se.chance \
             FROM spawnentry se \
             INNER JOIN spawn2 s ON s.spawngroupid = se.spawngroupid \
             WHERE s.zone = $1",
        )
        .bind(zone_short_name)
        .fetch_all(pool)
        .await?;

        Ok(entries)
    }
}
