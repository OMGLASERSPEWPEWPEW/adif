// Titanium client opcode values from patch_Titanium.conf
// These are the application-layer opcodes (inside OP_Packet payloads)

// Login server opcodes
pub const OP_LOGIN_SESSION_READY: u16 = 0x0001;
pub const OP_LOGIN_LOGIN: u16 = 0x0002;
pub const OP_LOGIN_SERVER_LIST_REQUEST: u16 = 0x0004;
pub const OP_LOGIN_PLAY_REQUEST: u16 = 0x000d;
pub const OP_LOGIN_CHAT_MESSAGE: u16 = 0x0016;
pub const OP_LOGIN_SERVER_LIST_RESPONSE: u16 = 0x0018;
pub const OP_LOGIN_ACCEPTED: u16 = 0x0017;
pub const OP_LOGIN_PLAY_RESPONSE: u16 = 0x0021;

// World server opcodes
pub const OP_GUILDS_LIST: u16 = 0x6957;
pub const OP_APPROVE_WORLD: u16 = 0x3c25;
pub const OP_LOG_SERVER: u16 = 0x0fa6;
pub const OP_MOTD: u16 = 0x024d;
pub const OP_EXPANSION_INFO: u16 = 0x04ec;
pub const OP_POST_ENTER_WORLD: u16 = 0x52A4;
pub const OP_SEND_CHAR_INFO: u16 = 0x4513;
pub const OP_ZONE_SERVER_INFO: u16 = 0x61b6;

pub const OP_ZONE_ENTRY: u16 = 0x7213;
pub const OP_PLAYER_PROFILE: u16 = 0x75df;
pub const OP_NEW_ZONE: u16 = 0x0920;
pub const OP_REQ_CLIENT_SPAWN: u16 = 0x0322;
pub const OP_ZONE_SPAWNS: u16 = 0x2e78;
pub const OP_SET_SERVER_FILTER: u16 = 0x6563;
pub const OP_SEND_ZONE_POINTS: u16 = 0x3eba;
pub const OP_REQ_NEW_ZONE: u16 = 0x7ac5;
pub const OP_TIME_OF_DAY: u16 = 0x1580;
pub const OP_SEND_EXP_ZONEIN: u16 = 0x0587;
pub const OP_CONSIDER: u16 = 0x65ca;
pub const OP_SPAWN_APPEARANCE: u16 = 0x7c32;
pub const OP_DELETE_SPAWN: u16 = 0x55bc;
pub const OP_CLIENT_READY: u16 = 0x5e20;
pub const OP_NEW_SPAWN: u16 = 0x1860;
pub const OP_WEATHER: u16 = 0x254d;
pub const OP_TARGET_MOUSE: u16 = 0x6c47;
pub const OP_CLIENT_UPDATE: u16 = 0x14cb;
pub const OP_CHANNEL_MESSAGE: u16 = 0x1004;
pub const OP_HP_UPDATE: u16 = 0x3bcf;
pub const OP_ACK_PACKET: u16 = 0x7752;
pub const OP_SEND_LOGIN_INFO: u16 = 0x4dd0;
pub const OP_ENTER_WORLD: u16 = 0x7cba;
pub const OP_WORLD_COMPLETE: u16 = 0x509d;
pub const OP_CRASH_DUMP: u16 = 0x7825;
pub const OP_WORLD_OBJECTS_SENT: u16 = 0x1fa1;
pub const OP_SPAWN_DOOR: u16 = 0x4c24;
pub const OP_CHAR_INVENTORY: u16 = 0x5394;
pub const OP_SEND_AA_STATS: u16 = 0x5918;
pub const OP_SEND_AA_TABLE: u16 = 0x367D;
pub const OP_UPDATE_AA: u16 = 0x5966;
pub const OP_SEND_TRIBUTES: u16 = 0x067A;
pub const OP_GUILD_TRIBUTES: u16 = 0x5E3A;
pub const OP_APP_COMBINED: u16 = 0x1900;
pub const OP_GROUND_SPAWN: u16 = 0x0f47;
pub const OP_EXP_UPDATE: u16 = 0x5ecd;
pub const OP_RAID_UPDATE: u16 = 0x1f21;

pub fn opcode_name(opcode: u16) -> &'static str {
    match opcode {
        OP_ZONE_ENTRY => "OP_ZoneEntry",
        OP_PLAYER_PROFILE => "OP_PlayerProfile",
        OP_NEW_ZONE => "OP_NewZone",
        OP_REQ_CLIENT_SPAWN => "OP_ReqClientSpawn",
        OP_ZONE_SPAWNS => "OP_ZoneSpawns",
        OP_SET_SERVER_FILTER => "OP_SetServerFilter",
        OP_SEND_ZONE_POINTS => "OP_SendZonepoints",
        OP_REQ_NEW_ZONE => "OP_ReqNewZone",
        OP_TIME_OF_DAY => "OP_TimeOfDay",
        OP_SEND_EXP_ZONEIN => "OP_SendExpZonein",
        OP_CONSIDER => "OP_Consider",
        OP_SPAWN_APPEARANCE => "OP_SpawnAppearance",
        OP_DELETE_SPAWN => "OP_DeleteSpawn",
        OP_CLIENT_READY => "OP_ClientReady",
        OP_NEW_SPAWN => "OP_NewSpawn",
        OP_WEATHER => "OP_Weather",
        OP_TARGET_MOUSE => "OP_TargetMouse",
        OP_CLIENT_UPDATE => "OP_ClientUpdate",
        OP_CHANNEL_MESSAGE => "OP_ChannelMessage",
        OP_HP_UPDATE => "OP_HPUpdate",
        OP_ACK_PACKET => "OP_AckPacket",
        OP_WORLD_COMPLETE => "OP_WorldComplete",
        _ => "Unknown",
    }
}
