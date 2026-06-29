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
    // is_npc at offset 144 (distinct from NPC type at 83)
    if data.npc_type != 0 {
        write_u8(&mut buf, 144, 1);
    }
    // guildID at offset 238
    write_u32_le(&mut buf, 238, data.guild_id);
    // set_to_0xFF at offset 276 (8 bytes)
    for i in 0..8 {
        buf[276 + i] = 0xFF;
    }
    // showhelm at offset 139
    write_u8(&mut buf, 139, 1);

    buf
}

pub struct ZoneData {
    pub short_name: String,
    pub long_name: String,
    pub zone_id: u16,
    pub safe_x: f32,
    pub safe_y: f32,
    pub safe_z: f32,
    pub minclip: f32,
    pub maxclip: f32,
    pub fog_minclip: [f32; 4],
    pub fog_maxclip: [f32; 4],
    pub fog_red: [u8; 4],
    pub fog_green: [u8; 4],
    pub fog_blue: [u8; 4],
    pub fog_density: f32,
    pub sky: u8,
    pub ztype: u8,
    pub zone_exp_multiplier: f32,
    pub gravity: f32,
    pub time_type: u8,
    pub rain_chance: [u8; 4],
    pub rain_duration: [u8; 4],
    pub snow_chance: [u8; 4],
    pub snow_duration: [u8; 4],
    pub underworld: f32,
    pub max_z: f32,
}

pub fn build_new_zone_struct(char_name: &str, zd: &ZoneData) -> Vec<u8> {
    let mut buf = vec![0u8; NEW_ZONE_STRUCT_SIZE];

    write_str(&mut buf, 0, char_name, 64);
    write_str(&mut buf, 64, &zd.short_name, 32);
    write_str(&mut buf, 96, &zd.long_name, 278);

    // ztype at 374
    write_u8(&mut buf, 374, zd.ztype);
    // fog_red[4] at 375, fog_green[4] at 379, fog_blue[4] at 383
    for i in 0..4 {
        buf[375 + i] = zd.fog_red[i];
        buf[379 + i] = zd.fog_green[i];
        buf[383 + i] = zd.fog_blue[i];
    }
    // fog_minclip[4] at 388
    for i in 0..4 {
        write_f32_le(&mut buf, 388 + i * 4, zd.fog_minclip[i]);
    }
    // fog_maxclip[4] at 404
    for i in 0..4 {
        write_f32_le(&mut buf, 404 + i * 4, zd.fog_maxclip[i]);
    }
    // gravity at 420
    write_f32_le(&mut buf, 420, zd.gravity);
    // time_type at 424
    write_u8(&mut buf, 424, zd.time_type);
    // rain_chance[4] at 425, rain_duration[4] at 429
    for i in 0..4 {
        buf[425 + i] = zd.rain_chance[i];
        buf[429 + i] = zd.rain_duration[i];
    }
    // snow_chance[4] at 433, snow_duration[4] at 437
    for i in 0..4 {
        buf[433 + i] = zd.snow_chance[i];
        buf[437 + i] = zd.snow_duration[i];
    }
    // sky at 474
    write_u8(&mut buf, 474, zd.sky);
    // zone_exp_multiplier at 488
    write_f32_le(&mut buf, 488, zd.zone_exp_multiplier);
    // safe coords at 492/496/500
    write_f32_le(&mut buf, 492, zd.safe_y);
    write_f32_le(&mut buf, 496, zd.safe_x);
    write_f32_le(&mut buf, 500, zd.safe_z);
    // max_z at 504
    write_f32_le(&mut buf, 504, zd.max_z);
    // underworld at 508
    write_f32_le(&mut buf, 508, zd.underworld);
    // minclip at 512
    write_f32_le(&mut buf, 512, zd.minclip);
    // maxclip at 516
    write_f32_le(&mut buf, 516, zd.maxclip);
    // zone_short_name2 at 604
    write_str(&mut buf, 604, &zd.short_name, 68);
    // zone_id at 684
    write_u16_le(&mut buf, 684, zd.zone_id);

    buf
}

pub struct PlayerProfileData {
    pub name: String,
    pub last_name: String,
    pub race: u32,
    pub class_id: u32,
    pub level: u8,
    pub gender: u32,
    pub deity: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub heading: f32,
    pub zone_id: u16,
    pub face: u8,
    pub hair_color: u8,
    pub beard_color: u8,
    pub eye_color_1: u8,
    pub eye_color_2: u8,
    pub hair_style: u8,
    pub beard: u8,
    pub entity_id: u32,
}

pub fn build_player_profile(
    _name: &str,
    race: u32,
    class_id: u32,
    level: u8,
    gender: u32,
    x: f32,
    y: f32,
    z: f32,
    zone_id: u32,
) -> Vec<u8> {
    build_player_profile_full(&PlayerProfileData {
        name: _name.to_string(),
        last_name: String::new(),
        race, class_id, level, gender,
        deity: 0, x, y, z, heading: 0.0,
        zone_id: zone_id as u16,
        face: 0, hair_color: 0, beard_color: 0,
        eye_color_1: 0, eye_color_2: 0,
        hair_style: 0, beard: 0,
        entity_id: 0,
    })
}

pub fn build_player_profile_full(pp: &PlayerProfileData) -> Vec<u8> {
    let mut buf = vec![0u8; PLAYER_PROFILE_SIZE];

    // Titanium PlayerProfile_Struct offsets (from titanium_structs.h)
    write_u32_le(&mut buf, 4, pp.gender);
    write_u32_le(&mut buf, 8, pp.race);
    write_u32_le(&mut buf, 12, pp.class_id);
    write_u8(&mut buf, 20, pp.level);
    write_u8(&mut buf, 21, pp.level);

    // binds[0] (primary bind) at offset 24: zone_id(u32) + x(f32) + y(f32) + z(f32) + heading(f32) = 20 bytes
    write_u32_le(&mut buf, 24, pp.zone_id as u32);
    write_f32_le(&mut buf, 28, pp.x);
    write_f32_le(&mut buf, 32, pp.y);
    write_f32_le(&mut buf, 36, pp.z);
    write_f32_le(&mut buf, 40, pp.heading);
    // binds[4] (home) — same as primary for new characters
    write_u32_le(&mut buf, 104, pp.zone_id as u32);
    write_f32_le(&mut buf, 108, pp.x);
    write_f32_le(&mut buf, 112, pp.y);
    write_f32_le(&mut buf, 116, pp.z);
    write_f32_le(&mut buf, 120, pp.heading);

    // deity at 124
    write_u32_le(&mut buf, 124, pp.deity);

    // Appearance at 172-177
    write_u8(&mut buf, 172, pp.hair_color);
    write_u8(&mut buf, 173, pp.beard_color);
    write_u8(&mut buf, 174, pp.eye_color_1);
    write_u8(&mut buf, 175, pp.eye_color_2);
    write_u8(&mut buf, 176, pp.hair_style);
    write_u8(&mut buf, 177, pp.beard);

    // cur_hp at 2232
    write_u32_le(&mut buf, 2232, 1000);
    // STR through WIS at 2236 (7 stats x u32)
    for i in 0..7 {
        write_u32_le(&mut buf, 2236 + i * 4, 75);
    }
    // face at 2264
    write_u8(&mut buf, 2264, pp.face);

    // spellbook — fill unused slots with 0xFFFFFFFF (offset 2312, 400 u32s)
    for i in 0..400 {
        write_u32_le(&mut buf, 2312 + i * 4, 0xFFFFFFFF);
    }
    // unknown4184 fill with 0xFF (offset 3912, 448 bytes)
    for i in 0..448 {
        buf[3912 + i] = 0xFF;
    }
    // mem_spells — fill with 0xFFFFFFFF (offset 4360, 9 u32s)
    for i in 0..9 {
        write_u32_le(&mut buf, 4360 + i * 4, 0xFFFFFFFF);
    }

    // hunger/thirst (offset 5000/5004) — set to full
    write_u32_le(&mut buf, 5000, 6000);
    write_u32_le(&mut buf, 5004, 6000);

    // name at 12940 (64 chars)
    write_str(&mut buf, 12940, &pp.name, 64);
    // last_name at 13004 (32 chars)
    write_str(&mut buf, 13004, &pp.last_name, 32);
    // guild_id at 13036 (0xFFFFFFFF = no guild)
    write_u32_le(&mut buf, 13036, 0xFFFFFFFF);

    // x/y/z/heading at 13116-13128
    write_f32_le(&mut buf, 13116, pp.x);
    write_f32_le(&mut buf, 13120, pp.y);
    write_f32_le(&mut buf, 13124, pp.z);
    write_f32_le(&mut buf, 13128, pp.heading);

    // expansions at 13240 (all expansions)
    write_u32_le(&mut buf, 13240, 0x7FFF);

    // zone_id at 13276 (u16)
    write_u16_le(&mut buf, 13276, pp.zone_id);

    // entityid at 14384 — must match player's spawn_id
    write_u32_le(&mut buf, 14384, pp.entity_id);

    // air_remaining at 14900
    write_u32_le(&mut buf, 14900, 60);

    // available_slots at 12860 (must be 0xFFFFFFFF)
    write_u32_le(&mut buf, 12860, 0xFFFFFFFF);

    // unknown12864 — required magic bytes from EQEmu titanium.cpp lines 1600-1606
    let magic: [u8; 57] = [
        0x78, 0x03, 0x00, 0x00, 0x1A, 0x04, 0x00, 0x00, 0x1A, 0x04, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00,
        0x19, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x0F, 0x00, 0x00, 0x00, 0x0F, 0x00, 0x00, 0x00,
        0x0F, 0x00, 0x00, 0x00, 0x1F, 0x85, 0xEB, 0x3E, 0x33, 0x33, 0x33, 0x3F, 0x09, 0x00, 0x00, 0x00,
        0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x14,
    ];
    buf[12864..12864 + 57].copy_from_slice(&magic);

    // level3 at 19576 (for leadership AA max)
    write_u32_le(&mut buf, 19576, pp.level as u32);
    // showhelm at 19580
    write_u32_le(&mut buf, 19580, 1);

    // unknown04396 at offset 4396 — 32 bytes must be 0xFF (EQEmu titanium.cpp line 1339)
    for i in 0..32 {
        buf[4396 + i] = 0xFF;
    }

    // EQ checksum: CRC32 over bytes 4..len-4 (EQEmu skips last 4 bytes of struct)
    let crc_end = buf.len() - 4;
    let mut check: u32 = 0xFFFFFFFF;
    for &byte in &buf[4..crc_end] {
        let index = ((byte as u32) ^ check) & 0xFF;
        check = (check >> 8) ^ crate::eq_protocol::codec::CRC32_TABLE[index as usize];
    }
    buf[0..4].copy_from_slice(&check.to_le_bytes());

    buf
}

pub fn recompute_pp_checksum(buf: &mut [u8]) {
    let crc_end = buf.len() - 4;
    let mut check: u32 = 0xFFFFFFFF;
    for &byte in &buf[4..crc_end] {
        let index = ((byte as u32) ^ check) & 0xFF;
        check = (check >> 8) ^ crate::eq_protocol::codec::CRC32_TABLE[index as usize];
    }
    buf[0..4].copy_from_slice(&check.to_le_bytes());
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
        let zd = ZoneData {
            short_name: "grobb".to_string(), long_name: "Grobb".to_string(),
            zone_id: 52, safe_x: -99.0, safe_y: -585.0, safe_z: 27.0,
            minclip: 450.0, maxclip: 450.0,
            fog_minclip: [10.0; 4], fog_maxclip: [500.0; 4],
            fog_red: [0; 4], fog_green: [0; 4], fog_blue: [0; 4], fog_density: 0.0,
            sky: 1, ztype: 255, zone_exp_multiplier: 1.0, gravity: 0.4, time_type: 2,
            rain_chance: [0; 4], rain_duration: [0; 4],
            snow_chance: [0; 4], snow_duration: [0; 4],
            underworld: -1000.0, max_z: 10000.0,
        };
        let buf = build_new_zone_struct("Ghouldan", &zd);
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
