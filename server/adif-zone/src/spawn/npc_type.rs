use sqlx::PgPool;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct NpcType {
    pub id: i32,
    pub name: String,
    pub lastname: Option<String>,
    pub level: i16,
    pub race: i16,
    pub class: i16,
    pub bodytype: i32,
    pub hp: i64,
    pub mana: i64,
    pub gender: i16,
    pub texture: i16,
    pub helmtexture: i16,
    pub size: f32,
    pub walkspeed: f32,
    pub runspeed: f32,
    pub mindmg: i32,
    pub maxdmg: i32,
    pub aggroradius: i32,
    pub assistradius: i32,
    pub face: i32,
    pub luclin_hairstyle: i32,
    pub luclin_haircolor: i64,
    pub luclin_eyecolor: i64,
    pub luclin_eyecolor2: i64,
    pub luclin_beardcolor: i64,
    pub luclin_beard: i32,
    pub findable: i16,
    pub flymode: i16,
    pub light: i16,
    pub show_name: i16,
}

impl NpcType {
    pub async fn load_for_zone(pool: &PgPool, zone_short_name: &str) -> anyhow::Result<Vec<Self>> {
        let npcs = sqlx::query_as::<_, NpcType>(
            "SELECT DISTINCT n.id, n.name, n.lastname, n.level, n.race, n.class, \
             n.bodytype, n.hp, n.mana, n.gender, n.texture, n.helmtexture, \
             n.size, n.walkspeed, n.runspeed, n.mindmg, n.maxdmg, \
             n.aggroradius, n.assistradius, n.face, \
             n.luclin_hairstyle, n.luclin_haircolor, n.luclin_eyecolor, \
             n.luclin_eyecolor2, n.luclin_beardcolor, n.luclin_beard, \
             n.findable, n.flymode, n.light, n.show_name \
             FROM npc_types n \
             INNER JOIN spawnentry se ON se.npcid = n.id \
             INNER JOIN spawn2 s ON s.spawngroupid = se.spawngroupid \
             WHERE s.zone = $1",
        )
        .bind(zone_short_name)
        .fetch_all(pool)
        .await?;

        Ok(npcs)
    }
}
