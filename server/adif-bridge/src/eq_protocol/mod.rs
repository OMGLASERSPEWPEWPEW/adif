pub mod codec;
pub mod fragment;
pub mod packet;
pub mod session;

pub const OP_SESSION_REQUEST: u8 = 0x01;
pub const OP_SESSION_RESPONSE: u8 = 0x02;
pub const OP_COMBINED: u8 = 0x03;
pub const OP_SESSION_DISCONNECT: u8 = 0x05;
pub const OP_KEEP_ALIVE: u8 = 0x06;
pub const OP_SESSION_STAT_REQUEST: u8 = 0x07;
pub const OP_SESSION_STAT_RESPONSE: u8 = 0x08;
pub const OP_PACKET: u8 = 0x09;
pub const OP_FRAGMENT: u8 = 0x0d;
pub const OP_OUT_OF_ORDER_ACK: u8 = 0x11;
pub const OP_ACK: u8 = 0x15;
pub const OP_APP_COMBINED: u8 = 0x19;
pub const OP_OUTBOUND_PING: u8 = 0x1c;
pub const OP_OUT_OF_SESSION: u8 = 0x1d;
