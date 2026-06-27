// Titanium wire format struct sizes and builders.
// These produce raw byte buffers matching the EQ Titanium client's expected formats.
// Field offsets derived from studying the EQEmu open-source server.

pub const SPAWN_STRUCT_SIZE: usize = 385;
pub const NEW_ZONE_STRUCT_SIZE: usize = 700;
pub const PLAYER_PROFILE_SIZE: usize = 19592;
pub const CLIENT_ZONE_ENTRY_SIZE: usize = 68;
pub const TIME_OF_DAY_SIZE: usize = 8;
pub const WEATHER_SIZE: usize = 12;
pub const SPAWN_APPEARANCE_SIZE: usize = 8;

fn write_str(buf: &mut [u8], offset: usize, s: &str, max_len: usize) {
    let bytes = s.as_bytes();
    let len = bytes.len().min(max_len - 1);
    buf[offset..offset + len].copy_from_slice(&bytes[..len]);
}

fn write_u8(buf: &mut [u8], offset: usize, val: u8) {
    buf[offset] = val;
}

fn write_u16_le(buf: &mut [u8], offset: usize, val: u16) {
    buf[offset..offset + 2].copy_from_slice(&val.to_le_bytes());
}

fn write_u32_le(buf: &mut [u8], offset: usize, val: u32) {
    buf[offset..offset + 4].copy_from_slice(&val.to_le_bytes());
}

fn write_f32_le(buf: &mut [u8], offset: usize, val: f32) {
    buf[offset..offset + 4].copy_from_slice(&val.to_le_bytes());
}

pub struct SpawnData {
    pub spawn_id: u32,
    pub name: String,
    pub last_name: String,
    pub level: u8,
    pub race: u32,
    pub class_id: u8,
    pub gender: u8,
    pub deity: u16,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub heading: f32,
    pub size: f32,
    pub npc_type: u8,
    pub cur_hp: u8,
    pub max_hp: u8,
    pub body_type: u8,
    pub run_speed: f32,
    pub walk_speed: f32,
    pub findable: u8,
    pub light: u8,
    pub texture: u8,
    pub helm_texture: u8,
    pub guild_id: u32,
}

pub fn build_spawn_struct(data: &SpawnData) -> Vec<u8> {
    let mut buf = vec![0u8; SPAWN_STRUCT_SIZE];

    // NPC type at offset 83
    write_u8(&mut buf, 83, data.npc_type);
    // name at offset 7 (64 bytes)
    write_str(&mut buf, 7, &data.name, 64);
    // deity at offset 71
    write_u16_le(&mut buf, 71, data.deity);
    // size at offset 75
    write_f32_le(&mut buf, 75, data.size);
    // cur_hp at offset 86
    write_u8(&mut buf, 86, data.cur_hp);
    // max_hp at offset 87
    write_u8(&mut buf, 87, data.max_hp);
    // findable at offset 88
    write_u8(&mut buf, 88, data.findable);

    // Position bitfields at offsets 94-113 (5 u32s = 20 bytes)
    // These use packed bitfields: x/y/z as 19-bit signed (value * 8), heading as 12-bit unsigned
    let ix = (data.x * 8.0) as i32;
    let iy = (data.y * 8.0) as i32;
    let iz = (data.z * 8.0) as i32;
    let iheading = ((data.heading * 4096.0 / 360.0) as u32) & 0xFFF;

    // Word 0 (offset 94): deltaHeading:10, x:19, padding:3
    let word0: u32 = ((ix as u32) & 0x7FFFF) << 10;
    write_u32_le(&mut buf, 94, word0);

    // Word 1 (offset 98): y:19, animation:10, padding:3
    let word1: u32 = (iy as u32) & 0x7FFFF;
    write_u32_le(&mut buf, 98, word1);

    // Word 2 (offset 102): z:19, deltaY:13
    let word2: u32 = (iz as u32) & 0x7FFFF;
    write_u32_le(&mut buf, 102, word2);

    // Word 3 (offset 106): deltaX:13, heading:12, padding:7
    let word3: u32 = (iheading & 0xFFF) << 13;
    write_u32_le(&mut buf, 106, word3);

    // Word 4 (offset 110): deltaZ:13, padding:19
    write_u32_le(&mut buf, 110, 0);

    // level at offset 151
    write_u8(&mut buf, 151, data.level);
    // lastName at offset 292 (32 bytes)
    write_str(&mut buf, 292, &data.last_name, 32);
    // runspeed at offset 233
    write_f32_le(&mut buf, 233, data.run_speed);
    // walkspeed at offset 324
    write_f32_le(&mut buf, 324, data.walk_speed);
    // race at offset 284
    write_u32_le(&mut buf, 284, data.race);
    // class at offset 331
    write_u8(&mut buf, 331, data.class_id);
    // gender at offset 334
    write_u8(&mut buf, 334, data.gender);
    // bodytype at offset 335
    write_u8(&mut buf, 335, data.body_type);
    // light at offset 330
    write_u8(&mut buf, 330, data.light);
    // helm at offset 275
    write_u8(&mut buf, 275, data.helm_texture);
    // spawnId at offset 340
    write_u32_le(&mut buf, 340, data.spawn_id);
    // guildID at offset 238
    write_u32_le(&mut buf, 238, 0xFFFFFFFF);
    // set_to_0xFF at offset 276 (8 bytes)
    for i in 0..8 {
        buf[276 + i] = 0xFF;
    }
    // showhelm at offset 139
    write_u8(&mut buf, 139, 1);

    buf
}

pub fn build_new_zone_struct(
    char_name: &str,
    zone_short: &str,
    zone_long: &str,
    safe_x: f32,
    safe_y: f32,
    safe_z: f32,
    min_clip: f32,
    max_clip: f32,
    zone_id: u16,
) -> Vec<u8> {
    let mut buf = vec![0u8; NEW_ZONE_STRUCT_SIZE];

    write_str(&mut buf, 0, char_name, 64);
    write_str(&mut buf, 64, zone_short, 32);
    write_str(&mut buf, 96, zone_long, 278);

    // ztype at 374
    write_u8(&mut buf, 374, 0xFF);
    // gravity at 420
    write_f32_le(&mut buf, 420, 0.4);
    // time_type at 424
    write_u8(&mut buf, 424, 2);
    // sky at 474
    write_u8(&mut buf, 474, 1);
    // zone_exp_multiplier at 488
    write_f32_le(&mut buf, 488, 1.0);
    // safe coords at 492/496/500
    write_f32_le(&mut buf, 492, safe_y);
    write_f32_le(&mut buf, 496, safe_x);
    write_f32_le(&mut buf, 500, safe_z);
    // underworld at 508
    write_f32_le(&mut buf, 508, -1000.0);
    // minclip at 512
    write_f32_le(&mut buf, 512, min_clip);
    // maxclip at 516
    write_f32_le(&mut buf, 516, max_clip);
    // zone_short_name2 at 604
    write_str(&mut buf, 604, zone_short, 68);
    // zone_id at 684
    write_u16_le(&mut buf, 684, zone_id);

    buf
}

pub fn build_player_profile(
    name: &str,
    race: u32,
    class_id: u32,
    level: u8,
    gender: u32,
    x: f32,
    y: f32,
    z: f32,
    zone_id: u32,
) -> Vec<u8> {
    let mut buf = vec![0u8; PLAYER_PROFILE_SIZE];

    // Skip checksum at 0 (will be computed by client or ignored for now)
    write_u32_le(&mut buf, 4, gender);
    write_u32_le(&mut buf, 8, race);
    write_u32_le(&mut buf, 12, class_id);
    write_u8(&mut buf, 20, level);
    write_u8(&mut buf, 21, level);

    // Primary bind point at offset 24: zone_id(u32), x(f32), y(f32), z(f32), heading(f32) = 20 bytes
    write_u32_le(&mut buf, 24, zone_id);
    write_f32_le(&mut buf, 28, x);
    write_f32_le(&mut buf, 32, y);
    write_f32_le(&mut buf, 36, z);

    // cur_hp at 2232
    write_u32_le(&mut buf, 2232, 1000);
    // STR-CHA base stats at 2236+ (set to reasonable defaults)
    for i in 0..7 {
        write_u32_le(&mut buf, 2236 + i * 4, 75);
    }

    // showhelm at 19580
    write_u32_le(&mut buf, 19580, 1);

    buf
}

pub fn build_time_of_day(hour: u8, minute: u8, day: u16, year: u16) -> Vec<u8> {
    let mut buf = vec![0u8; TIME_OF_DAY_SIZE];
    write_u8(&mut buf, 0, hour);
    write_u8(&mut buf, 1, minute);
    write_u16_le(&mut buf, 2, day);
    write_u16_le(&mut buf, 4, year);
    buf
}

pub fn build_weather(weather_type: u32, intensity: u32) -> Vec<u8> {
    let mut buf = vec![0u8; WEATHER_SIZE];
    write_u32_le(&mut buf, 0, weather_type);
    write_u32_le(&mut buf, 4, intensity);
    buf
}

pub fn build_spawn_appearance(entity_id: u32, appearance_type: u32, value: u32) -> Vec<u8> {
    let mut buf = vec![0u8; 12];
    write_u32_le(&mut buf, 0, entity_id);
    write_u16_le(&mut buf, 4, appearance_type as u16);
    write_u32_le(&mut buf, 8, value);
    buf
}

pub fn extract_zone_entry_name(data: &[u8]) -> String {
    if data.len() < 5 {
        return String::from("Unknown");
    }
    let name_bytes = &data[4..];
    let end = name_bytes.iter().position(|&b| b == 0).unwrap_or(name_bytes.len().min(64));
    String::from_utf8_lossy(&name_bytes[..end]).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn_struct_correct_size() {
        let data = SpawnData {
            spawn_id: 1, name: "TestNPC".to_string(), last_name: String::new(),
            level: 10, race: 1, class_id: 1, gender: 0, deity: 0,
            x: 100.0, y: 200.0, z: 5.0, heading: 90.0, size: 6.0,
            npc_type: 1, cur_hp: 100, max_hp: 100, body_type: 1,
            run_speed: 0.7, walk_speed: 0.46, findable: 1, light: 0,
            texture: 0, helm_texture: 0, guild_id: 0xFFFFFFFF,
        };
        let buf = build_spawn_struct(&data);
        assert_eq!(buf.len(), SPAWN_STRUCT_SIZE);
        // Check name at offset 7
        assert_eq!(&buf[7..14], b"TestNPC");
        // Check level at offset 151
        assert_eq!(buf[151], 10);
        // Check spawnId at offset 340
        assert_eq!(u32::from_le_bytes([buf[340], buf[341], buf[342], buf[343]]), 1);
    }

    #[test]
    fn new_zone_struct_correct_size() {
        let buf = build_new_zone_struct("Ghouldan", "grobb", "Grobb", -99.0, -585.0, 27.0, 450.0, 450.0, 52);
        assert_eq!(buf.len(), NEW_ZONE_STRUCT_SIZE);
        assert_eq!(&buf[0..8], b"Ghouldan");
        assert_eq!(&buf[64..69], b"grobb");
    }

    #[test]
    fn player_profile_correct_size() {
        let buf = build_player_profile("Ghouldan", 5, 11, 50, 0, -99.0, -585.0, 27.0, 52);
        assert_eq!(buf.len(), PLAYER_PROFILE_SIZE);
        assert_eq!(buf[20], 50); // level
    }

    #[test]
    fn time_of_day_correct_size() {
        let buf = build_time_of_day(14, 30, 1, 3100);
        assert_eq!(buf.len(), TIME_OF_DAY_SIZE);
        assert_eq!(buf[0], 14);
        assert_eq!(buf[1], 30);
    }
}
