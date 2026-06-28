use sqlx::PgPool;

#[derive(Debug, sqlx::FromRow)]
pub struct CharSelectEntry {
    pub id: i32,
    pub name: String,
    pub level: i32,
    pub race: i16,
    #[sqlx(rename = "class")]
    pub class_id: i16,
    pub gender: i16,
    pub zone_id: i32,
    pub deity: i32,
    pub face: i32,
}

pub async fn load_character_list(
    pool: &PgPool,
    account_id: i32,
) -> anyhow::Result<Vec<CharSelectEntry>> {
    let chars = sqlx::query_as::<_, CharSelectEntry>(
        "SELECT id, name, level, race, class, gender, zone_id, deity, face \
         FROM character_data \
         WHERE account_id = $1 AND deleted_at IS NULL \
         ORDER BY name \
         LIMIT 8",
    )
    .bind(account_id)
    .fetch_all(pool)
    .await?;

    Ok(chars)
}

#[derive(Debug, sqlx::FromRow)]
pub struct CharacterRecord {
    pub id: i32,
    pub account_id: i32,
    pub name: String,
    pub last_name: String,
    pub race: i16,
    #[sqlx(rename = "class")]
    pub class_id: i16,
    pub level: i32,
    pub gender: i16,
    pub deity: i32,
    pub zone_id: i32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub heading: f32,
    pub face: i32,
    pub hair_color: i16,
    pub hair_style: i16,
    pub beard: i16,
    pub beard_color: i16,
    pub eye_color_1: i16,
    pub eye_color_2: i16,
    pub gm: i16,
}

pub async fn load_character(
    pool: &PgPool,
    account_id: i32,
    char_name: &str,
) -> anyhow::Result<Option<CharacterRecord>> {
    let record = sqlx::query_as::<_, CharacterRecord>(
        "SELECT id, account_id, name, last_name, \
         race, class, level, gender, deity, \
         zone_id, x, y, z, heading, \
         face, hair_color, hair_style, beard, beard_color, \
         eye_color_1, eye_color_2, gm \
         FROM character_data \
         WHERE name = $1 AND account_id = $2 AND deleted_at IS NULL",
    )
    .bind(char_name)
    .bind(account_id)
    .fetch_optional(pool)
    .await?;

    Ok(record)
}
