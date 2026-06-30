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

fn server_to_titanium_slot(server_slot: i32) -> i32 {
    match server_slot {
        0..=20 => server_slot,
        22 => server_slot - 1,
        23..=30 => server_slot - 1,
        33 => server_slot - 3,
        _ => server_slot,
    }
}

fn swap_bits_21_and_22(mask: u32) -> u32 {
    let bit21 = 1u32 << 21;
    let bit22 = 1u32 << 22;
    let mut m = mask;
    if ((m & bit21) != 0) != ((m & bit22) != 0) {
        m ^= bit21 | bit22;
    }
    m
}

fn catch22(mask: u32) -> u32 {
    mask & !(1u32 << 22)
}

#[derive(Debug, sqlx::FromRow)]
pub struct InventoryItemRow {
    pub slot_id: i32,
    pub item_id: i32,
    pub charges: i16,
    pub itemclass: i32,
    pub name: String,
    pub lore: String,
    pub idfile: String,
    pub item_db_id: i32,
    pub weight: i32,
    pub norent: i32,
    pub nodrop: i32,
    pub size: i32,
    pub slots: i32,
    pub price: i32,
    pub icon: i32,
    pub benefitflag: i32,
    pub tradeskills: i32,
    pub cr: i32,
    pub dr: i32,
    pub pr: i32,
    pub mr: i32,
    pub fr: i32,
    pub astr: i32,
    pub asta: i32,
    pub aagi: i32,
    pub adex: i32,
    pub acha: i32,
    pub aint: i32,
    pub awis: i32,
    pub hp: i32,
    pub mana: i32,
    pub ac: i32,
    pub deity: i32,
    pub skillmodvalue: i32,
    pub skillmodmax: i32,
    pub skillmodtype: i32,
    pub banedmgrace: i32,
    pub banedmgamt: i32,
    pub banedmgbody: i32,
    pub magic: i32,
    pub casttime_: i32,
    pub reqlevel: i32,
    pub bardtype: i32,
    pub bardvalue: i32,
    pub light: i32,
    pub delay: i32,
    pub reclevel: i32,
    pub recskill: i32,
    pub elemdmgtype: i32,
    pub elemdmgamt: i32,
    pub range: i32,
    pub damage: i32,
    pub color: i64,
    pub classes: i32,
    pub races: i32,
    pub maxcharges: i32,
    pub itemtype: i32,
    pub material: i32,
    pub sellrate: f32,
    pub procrate: i32,
    pub combateffects: String,
    pub shielding: i32,
    pub stunresist: i32,
    pub strikethrough: i32,
    pub extradmgskill: i32,
    pub extradmgamt: i32,
    pub spellshield: i32,
    pub avoidance: i32,
    pub accuracy: i32,
    pub charmfileid: String,
    pub factionmod1: i32,
    pub factionmod2: i32,
    pub factionmod3: i32,
    pub factionmod4: i32,
    pub factionamt1: i32,
    pub factionamt2: i32,
    pub factionamt3: i32,
    pub factionamt4: i32,
    pub charmfile: String,
    pub augtype: i32,
    pub augslot1type: i16,
    pub augslot1visible: i16,
    pub augslot2type: i16,
    pub augslot2visible: i16,
    pub augslot3type: i16,
    pub augslot3visible: i16,
    pub augslot4type: i16,
    pub augslot4visible: i16,
    pub augslot5type: i16,
    pub augslot5visible: i16,
    pub ldontheme: i32,
    pub ldonprice: i32,
    pub ldonsold: i32,
    pub bagtype: i32,
    pub bagslots: i32,
    pub bagsize: i32,
    pub bagwr: i32,
    pub book: i32,
    pub booktype: i32,
    pub filename: String,
    pub banedmgraceamt: i32,
    pub augrestrict: i32,
    pub loregroup: i32,
    pub pendingloreflag: i16,
    pub artifactflag: i16,
    pub summonedflag: i16,
    pub favor: i32,
    pub fvnodrop: i32,
    pub endur: i32,
    pub dotshielding: i32,
    pub attack: i32,
    pub regen: i32,
    pub manaregen: i32,
    pub enduranceregen: i32,
    pub haste: i32,
    pub damageshield: i32,
    pub recastdelay: i32,
    pub recasttype: i32,
    pub guildfavor: i32,
    pub augdistiller: i32,
    pub attuneable: i32,
    pub nopet: i32,
    pub pointtype: i32,
    pub potionbelt: i32,
    pub potionbeltslots: i32,
    pub stacksize: i32,
    pub notransfer: i32,
    pub stackable: i32,
    pub clickeffect: i32,
    pub clicktype: i32,
    pub clicklevel2: i32,
    pub clicklevel: i32,
    pub proceffect: i32,
    pub proctype: i32,
    pub proclevel2: i32,
    pub proclevel: i32,
    pub worneffect: i32,
    pub worntype: i32,
    pub wornlevel2: i32,
    pub wornlevel: i32,
    pub focuseffect: i32,
    pub focustype: i32,
    pub focuslevel2: i32,
    pub focuslevel: i32,
    pub scrolleffect: i32,
    pub scrolltype: i32,
    pub scrolllevel2: i32,
    pub scrolllevel: i32,
}

pub fn serialize_titanium_item(row: &InventoryItemRow, serial: i32) -> Vec<u8> {
    use std::fmt::Write;

    let titanium_slot = server_to_titanium_slot(row.slot_id);
    let is_stackable = row.stackable != 0;

    let stack_count = if is_stackable { row.charges as i32 } else { 0 };

    let wire_charges = if is_stackable {
        if row.itemtype == 21 { 1 } else { 0 }
    } else {
        row.charges as i32
    };

    let weight = row.weight.min(255);
    let bane_dmg_amt = row.banedmgamt.min(255);
    let slots_xformed = catch22(swap_bits_21_and_22(row.slots as u32));
    let combat_effects = row.combateffects.parse::<i32>().unwrap_or(0);
    let charm_file_id = row.charmfileid.parse::<i32>().unwrap_or(0);
    let color = row.color as u32;

    let mut s = String::with_capacity(512);

    // Instance data (11 fields)
    write!(s, "{}|0|{}|{}|1|0|{}|0|{}|0|0|",
        stack_count, titanium_slot, row.price, serial, wire_charges).unwrap();

    // Opening quote + item data (~120 fields)
    write!(s, "\"{}|{}|{}|{}|{}|{}",
        row.itemclass, row.name, row.lore, row.idfile, row.item_db_id, weight).unwrap();

    write!(s, "|{}|{}|{}|{}|{}|{}",
        row.norent, row.nodrop, row.size, slots_xformed, row.price, row.icon).unwrap();
    write!(s, "|0|0|{}|{}", row.benefitflag, row.tradeskills).unwrap();

    write!(s, "|{}|{}|{}|{}|{}", row.cr, row.dr, row.pr, row.mr, row.fr).unwrap();
    write!(s, "|{}|{}|{}|{}|{}|{}|{}",
        row.astr, row.asta, row.aagi, row.adex, row.acha, row.aint, row.awis).unwrap();
    write!(s, "|{}|{}|{}|{}", row.hp, row.mana, row.ac, row.deity).unwrap();
    write!(s, "|{}|{}|{}", row.skillmodvalue, row.skillmodmax, row.skillmodtype).unwrap();
    write!(s, "|{}|{}|{}", row.banedmgrace, bane_dmg_amt, row.banedmgbody).unwrap();
    write!(s, "|{}|{}|{}|{}|{}", row.magic, row.casttime_, row.reqlevel, row.bardtype, row.bardvalue).unwrap();
    write!(s, "|{}|{}", row.light, row.delay).unwrap();
    write!(s, "|{}|{}", row.reclevel, row.recskill).unwrap();
    write!(s, "|{}|{}", row.elemdmgtype, row.elemdmgamt).unwrap();
    write!(s, "|{}|{}", row.range, row.damage).unwrap();
    write!(s, "|{}|{}|{}|0", color, row.classes, row.races).unwrap();
    write!(s, "|{}|{}|{}|{:.6}", row.maxcharges, row.itemtype, row.material, row.sellrate).unwrap();
    write!(s, "|0|{}|0", row.casttime_).unwrap();
    write!(s, "|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        row.procrate, combat_effects, row.shielding, row.stunresist,
        row.strikethrough, row.extradmgskill, row.extradmgamt,
        row.spellshield, row.avoidance, row.accuracy).unwrap();
    write!(s, "|{}", charm_file_id).unwrap();
    write!(s, "|{}|{}|{}|{}", row.factionmod1, row.factionmod2, row.factionmod3, row.factionmod4).unwrap();
    write!(s, "|{}|{}|{}|{}", row.factionamt1, row.factionamt2, row.factionamt3, row.factionamt4).unwrap();
    write!(s, "|{}", row.charmfile).unwrap();
    write!(s, "|{}", row.augtype).unwrap();
    write!(s, "|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        row.augslot1type, row.augslot1visible,
        row.augslot2type, row.augslot2visible,
        row.augslot3type, row.augslot3visible,
        row.augslot4type, row.augslot4visible,
        row.augslot5type, row.augslot5visible).unwrap();
    write!(s, "|{}|{}|{}", row.ldontheme, row.ldonprice, row.ldonsold).unwrap();
    write!(s, "|{}|{}|{}|{}", row.bagtype, row.bagslots, row.bagsize, row.bagwr).unwrap();
    write!(s, "|{}|{}", row.book, row.booktype).unwrap();
    write!(s, "|{}", row.filename).unwrap();
    write!(s, "|{}|{}|{}|{}|{}|{}",
        row.banedmgraceamt, row.augrestrict, row.loregroup,
        row.pendingloreflag, row.artifactflag, row.summonedflag).unwrap();
    write!(s, "|{}|{}|{}|{}", row.favor, row.fvnodrop, row.endur, row.dotshielding).unwrap();
    write!(s, "|{}|{}|{}|{}", row.attack, row.regen, row.manaregen, row.enduranceregen).unwrap();
    write!(s, "|{}|{}|{}|{}|{}", row.haste, row.damageshield, row.recastdelay, row.recasttype, row.guildfavor).unwrap();
    write!(s, "|{}", row.augdistiller).unwrap();
    write!(s, "|0|0|{}|{}|0|{}", row.attuneable, row.nopet, row.pointtype).unwrap();
    write!(s, "|{}|{}|{}|{}|{}",
        row.potionbelt, row.potionbeltslots, row.stacksize, row.notransfer, row.stackable).unwrap();
    // click effect
    write!(s, "|{}|{}|{}|{}|0", row.clickeffect, row.clicktype, row.clicklevel2, row.clicklevel).unwrap();
    // proc effect
    write!(s, "|{}|{}|{}|{}|0", row.proceffect, row.proctype, row.proclevel2, row.proclevel).unwrap();
    // worn effect
    write!(s, "|{}|{}|{}|{}|0", row.worneffect, row.worntype, row.wornlevel2, row.wornlevel).unwrap();
    // focus effect
    write!(s, "|{}|{}|{}|{}|0", row.focuseffect, row.focustype, row.focuslevel2, row.focuslevel).unwrap();
    // scroll effect
    write!(s, "|{}|{}|{}|{}|0", row.scrolleffect, row.scrolltype, row.scrolllevel2, row.scrolllevel).unwrap();

    // Closing quote
    s.push('"');

    // 10 trailing pipes for empty bag sub-item slots
    s.push_str("||||||||||");

    let mut out = s.into_bytes();
    out.push(0); // NUL terminator
    out
}

pub const DEATH_STRUCT_SIZE: usize = 32;
pub const COMBAT_DAMAGE_SIZE: usize = 23;
pub const ANIMATION_SIZE: usize = 4;
pub const MOB_HEALTH_SIZE: usize = 3;

pub fn build_death_struct(
    spawn_id: u32, killer_id: u32, corpse_id: u32,
    damage: u32, spell_id: u32, attack_skill: u32,
) -> Vec<u8> {
    let mut buf = vec![0u8; DEATH_STRUCT_SIZE];
    write_u32_le(&mut buf, 0, spawn_id);
    write_u32_le(&mut buf, 4, killer_id);
    write_u32_le(&mut buf, 8, corpse_id);
    write_u32_le(&mut buf, 12, 0); // bindzoneid (0 for NPCs)
    write_u32_le(&mut buf, 16, spell_id);
    write_u32_le(&mut buf, 20, attack_skill);
    write_u32_le(&mut buf, 24, damage);
    // offset 28: unknown028 = 0 (already zeroed)
    buf
}

pub fn build_combat_damage(
    target: u16, source: u16, dmg_type: u8, spell_id: u16,
    damage: u32, force: f32, hit_heading: f32, hit_pitch: f32,
) -> Vec<u8> {
    let mut buf = vec![0u8; COMBAT_DAMAGE_SIZE];
    write_u16_le(&mut buf, 0, target);
    write_u16_le(&mut buf, 2, source);
    buf[4] = dmg_type;
    write_u16_le(&mut buf, 5, spell_id);
    write_u32_le(&mut buf, 7, damage);
    write_f32_le(&mut buf, 11, force);
    write_f32_le(&mut buf, 15, hit_heading);
    write_f32_le(&mut buf, 19, hit_pitch);
    buf
}

pub fn build_animation(spawn_id: u16, speed: u8, action: u8) -> Vec<u8> {
    let mut buf = vec![0u8; ANIMATION_SIZE];
    write_u16_le(&mut buf, 0, spawn_id);
    buf[2] = speed;
    buf[3] = action;
    buf
}

pub fn build_mob_health(spawn_id: i16, hp_percent: u8) -> Vec<u8> {
    let mut buf = vec![0u8; MOB_HEALTH_SIZE];
    write_u16_le(&mut buf, 0, spawn_id as u16);
    buf[2] = hp_percent;
    buf
}

pub fn build_hp_update(cur_hp: i32, max_hp: i32, spawn_id: i16) -> Vec<u8> {
    let mut buf = vec![0u8; 10];
    write_u32_le(&mut buf, 0, cur_hp as u32);
    write_u32_le(&mut buf, 4, max_hp as u32);
    write_u16_le(&mut buf, 8, spawn_id as u16);
    buf
}

pub const MONEY_ON_CORPSE_SIZE: usize = 20;
pub const LOOT_ITEM_SIZE: usize = 16;

pub fn build_money_on_corpse(
    response: u8, platinum: u32, gold: u32, silver: u32, copper: u32,
) -> Vec<u8> {
    let mut buf = vec![0u8; MONEY_ON_CORPSE_SIZE];
    buf[0] = response;
    if response == 1 {
        buf[1] = 0x42;
        buf[2] = 0xef;
    } else {
        buf[1] = 0x5a;
        buf[2] = 0x40;
    }
    write_u32_le(&mut buf, 4, platinum);
    write_u32_le(&mut buf, 8, gold);
    write_u32_le(&mut buf, 12, silver);
    write_u32_le(&mut buf, 16, copper);
    buf
}

pub fn build_loot_item(
    lootee: u32, looter: u32, slot_id: u16, auto_loot: i32,
) -> Vec<u8> {
    let mut buf = vec![0u8; LOOT_ITEM_SIZE];
    write_u32_le(&mut buf, 0, lootee);
    write_u32_le(&mut buf, 4, looter);
    write_u16_le(&mut buf, 8, slot_id);
    write_u32_le(&mut buf, 12, auto_loot as u32);
    buf
}

pub fn get_con_color_titanium(player_level: u8, target_level: u8) -> u32 {
    const GREEN: u32 = 2;
    const DARK_BLUE: u32 = 4;
    const RED: u32 = 13;
    const YELLOW: u32 = 15;
    const LIGHT_BLUE: u32 = 18;
    const WHITE_TITANIUM: u32 = 20;

    let diff = target_level as i16 - player_level as i16;

    if diff == 0 {
        return WHITE_TITANIUM;
    } else if diff >= 1 && diff <= 3 {
        return YELLOW;
    } else if diff >= 4 {
        return RED;
    }

    let my = player_level as i32;
    let other = target_level as i32;
    let con_gray_lvl = my - (my + 5) / 3;
    let con_green_lvl = my - (my + 7) / 4;

    if my <= 15 {
        if diff <= -6 {
            GREEN // Titanium: Gray remapped to Green
        } else {
            DARK_BLUE
        }
    } else if my <= 20 {
        if other <= con_gray_lvl {
            GREEN // Titanium: Gray remapped to Green
        } else if other <= con_green_lvl {
            GREEN
        } else {
            DARK_BLUE
        }
    } else {
        if other <= con_gray_lvl {
            GREEN // Titanium: Gray remapped to Green
        } else if other <= con_green_lvl {
            GREEN
        } else if diff <= -6 {
            LIGHT_BLUE
        } else {
            DARK_BLUE
        }
    }
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

    #[test]
    fn con_color_same_level() {
        assert_eq!(get_con_color_titanium(10, 10), 20); // WhiteTitanium
        assert_eq!(get_con_color_titanium(50, 50), 20);
        assert_eq!(get_con_color_titanium(1, 1), 20);
    }

    #[test]
    fn con_color_yellow() {
        assert_eq!(get_con_color_titanium(10, 11), 15); // diff +1
        assert_eq!(get_con_color_titanium(10, 12), 15); // diff +2
        assert_eq!(get_con_color_titanium(10, 13), 15); // diff +3
    }

    #[test]
    fn con_color_red() {
        assert_eq!(get_con_color_titanium(10, 14), 13); // diff +4
        assert_eq!(get_con_color_titanium(10, 20), 13); // diff +10
    }

    #[test]
    fn con_color_dark_blue() {
        assert_eq!(get_con_color_titanium(10, 8), 4); // diff -2
        assert_eq!(get_con_color_titanium(10, 6), 4); // diff -4
    }

    #[test]
    fn con_color_green_low_level() {
        assert_eq!(get_con_color_titanium(10, 3), 2); // diff -7, level<=15 → Green
        assert_eq!(get_con_color_titanium(10, 1), 2); // diff -9 → Green
    }

    #[test]
    fn con_color_light_blue_high_level() {
        // level 50: gray_lvl = 50 - 55/3 = 50 - 18 = 32, green_lvl = 50 - 57/4 = 50 - 14 = 36
        assert_eq!(get_con_color_titanium(50, 44), 18); // diff -6, above green_lvl → LightBlue
        assert_eq!(get_con_color_titanium(50, 40), 18); // diff -10, above green_lvl → LightBlue
    }

    #[test]
    fn con_color_green_high_level() {
        // level 50: green_lvl = 36, gray_lvl = 32
        assert_eq!(get_con_color_titanium(50, 35), 2); // below green_lvl, above gray → Green
        assert_eq!(get_con_color_titanium(50, 33), 2); // below green_lvl, above gray → Green
    }

    #[test]
    fn con_color_gray_remapped_to_green() {
        // level 50: gray_lvl = 32
        assert_eq!(get_con_color_titanium(50, 30), 2); // below gray → Green (Titanium remap)
        assert_eq!(get_con_color_titanium(50, 1), 2);  // way below → Green
    }

    #[test]
    fn death_struct_correct_size() {
        let buf = build_death_struct(10, 1, 10, 5, 0xFFFFFFFF, 0);
        assert_eq!(buf.len(), DEATH_STRUCT_SIZE);
        assert_eq!(u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]), 10); // spawn_id
        assert_eq!(u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]), 1);  // killer_id
        assert_eq!(u32::from_le_bytes([buf[8], buf[9], buf[10], buf[11]]), 10); // corpseid
        assert_eq!(u32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]), 0); // bindzoneid
        assert_eq!(u32::from_le_bytes([buf[16], buf[17], buf[18], buf[19]]), 0xFFFFFFFF); // spell_id
        assert_eq!(u32::from_le_bytes([buf[24], buf[25], buf[26], buf[27]]), 5); // damage
    }

    #[test]
    fn combat_damage_correct_size() {
        let buf = build_combat_damage(10, 1, 0, 0xFFFF, 42, 0.0, 1.5, 0.0);
        assert_eq!(buf.len(), COMBAT_DAMAGE_SIZE);
        assert_eq!(u16::from_le_bytes([buf[0], buf[1]]), 10); // target
        assert_eq!(u16::from_le_bytes([buf[2], buf[3]]), 1);  // source
        assert_eq!(buf[4], 0); // type
        assert_eq!(u16::from_le_bytes([buf[5], buf[6]]), 0xFFFF); // spellid
        assert_eq!(u32::from_le_bytes([buf[7], buf[8], buf[9], buf[10]]), 42); // damage
    }

    #[test]
    fn animation_correct_size() {
        let buf = build_animation(5, 10, 8);
        assert_eq!(buf.len(), ANIMATION_SIZE);
        assert_eq!(u16::from_le_bytes([buf[0], buf[1]]), 5);
        assert_eq!(buf[2], 10); // speed
        assert_eq!(buf[3], 8);  // action (h2h)
    }

    #[test]
    fn mob_health_correct_size() {
        let buf = build_mob_health(10, 75);
        assert_eq!(buf.len(), MOB_HEALTH_SIZE);
        assert_eq!(buf[2], 75); // hp percent
    }
}
