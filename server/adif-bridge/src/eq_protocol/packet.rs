use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;

use super::*;

#[derive(Debug)]
pub enum ProtocolPacket {
    SessionRequest {
        protocol_version: u32,
        connect_code: u32,
        max_packet_size: u32,
    },
    SessionDisconnect {
        connect_code: u32,
    },
    KeepAlive,
    SessionStatRequest {
        data: Vec<u8>,
    },
    Ack {
        sequence: u16,
    },
    OutOfOrderAck {
        sequence: u16,
    },
    AppPacket {
        sequence: u16,
        data: Vec<u8>,
    },
    Fragment {
        sequence: u16,
        data: Vec<u8>,
    },
    Combined {
        sub_packets: Vec<Vec<u8>>,
    },
    OutboundPing,
    Unknown {
        opcode: u8,
        data: Vec<u8>,
    },
}

pub fn parse_protocol_packet(raw: &[u8]) -> anyhow::Result<ProtocolPacket> {
    if raw.len() < 2 {
        anyhow::bail!("Packet too short: {} bytes", raw.len());
    }

    let zero = raw[0];
    let opcode = raw[1];

    if zero != 0x00 {
        anyhow::bail!("Expected zero byte, got 0x{zero:02x}");
    }

    let body = &raw[2..];

    match opcode {
        OP_SESSION_REQUEST => {
            if body.len() < 12 {
                anyhow::bail!("SessionRequest too short");
            }
            let mut cur = Cursor::new(body);
            let protocol_version = cur.read_u32::<BigEndian>()?;
            let connect_code = cur.read_u32::<BigEndian>()?;
            let max_packet_size = cur.read_u32::<BigEndian>()?;
            Ok(ProtocolPacket::SessionRequest {
                protocol_version,
                connect_code,
                max_packet_size,
            })
        }

        OP_SESSION_DISCONNECT => {
            let mut cur = Cursor::new(body);
            let connect_code = if body.len() >= 4 {
                cur.read_u32::<BigEndian>()?
            } else {
                0
            };
            Ok(ProtocolPacket::SessionDisconnect { connect_code })
        }

        OP_KEEP_ALIVE => Ok(ProtocolPacket::KeepAlive),

        OP_SESSION_STAT_REQUEST => Ok(ProtocolPacket::SessionStatRequest {
            data: body.to_vec(),
        }),

        0x15..=0x18 => {
            if body.len() < 2 {
                anyhow::bail!("Ack too short");
            }
            let sequence = u16::from_be_bytes([body[0], body[1]]);
            Ok(ProtocolPacket::Ack { sequence })
        }

        0x11..=0x14 => {
            if body.len() < 2 {
                anyhow::bail!("OutOfOrderAck too short");
            }
            let sequence = u16::from_be_bytes([body[0], body[1]]);
            Ok(ProtocolPacket::OutOfOrderAck { sequence })
        }

        0x09..=0x0c => {
            if body.len() < 2 {
                anyhow::bail!("AppPacket too short");
            }
            let sequence = u16::from_be_bytes([body[0], body[1]]);
            Ok(ProtocolPacket::AppPacket {
                sequence,
                data: body[2..].to_vec(),
            })
        }

        0x0d..=0x10 => {
            if body.len() < 2 {
                anyhow::bail!("Fragment too short");
            }
            let sequence = u16::from_be_bytes([body[0], body[1]]);
            Ok(ProtocolPacket::Fragment {
                sequence,
                data: body[2..].to_vec(),
            })
        }

        OP_COMBINED => {
            let mut sub_packets = Vec::new();
            let mut offset = 0;
            while offset < body.len() {
                let len = body[offset] as usize;
                offset += 1;
                if offset + len > body.len() {
                    break;
                }
                sub_packets.push(body[offset..offset + len].to_vec());
                offset += len;
            }
            Ok(ProtocolPacket::Combined { sub_packets })
        }

        OP_OUTBOUND_PING => Ok(ProtocolPacket::OutboundPing),

        _ => Ok(ProtocolPacket::Unknown {
            opcode,
            data: body.to_vec(),
        }),
    }
}

pub fn build_session_response(
    connect_code: u32,
    encode_key: u32,
    crc_bytes: u8,
    max_packet_size: u32,
) -> Vec<u8> {
    let mut buf = Vec::with_capacity(17);
    buf.push(0x00);
    buf.push(OP_SESSION_RESPONSE);
    buf.extend_from_slice(&connect_code.to_be_bytes());
    buf.extend_from_slice(&encode_key.to_be_bytes());
    buf.push(crc_bytes);
    buf.push(0x01); // encode_pass1 = Compression
    buf.push(0x00); // encode_pass2 = None
    buf.extend_from_slice(&max_packet_size.to_be_bytes());
    buf
}

pub fn build_keep_alive() -> Vec<u8> {
    vec![0x00, OP_KEEP_ALIVE]
}

pub fn build_ack(sequence: u16) -> Vec<u8> {
    let mut buf = vec![0x00, OP_ACK];
    buf.extend_from_slice(&sequence.to_be_bytes());
    buf
}

pub fn build_app_packet(sequence: u16, app_data: &[u8], encode_key: u32, crc_bytes: u8) -> Vec<u8> {
    let mut inner = Vec::new();
    inner.extend_from_slice(app_data);

    let compressed = super::codec::compress(&inner);

    let mut buf = Vec::new();
    buf.push(0x00);
    buf.push(OP_PACKET);
    buf.extend_from_slice(&sequence.to_be_bytes());
    buf.extend_from_slice(&compressed);

    super::codec::append_crc(&mut buf, encode_key, crc_bytes);
    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_session_request() {
        let mut pkt = vec![0x00, 0x01];
        pkt.extend_from_slice(&2u32.to_be_bytes()); // protocol_version
        pkt.extend_from_slice(&0xDEADBEEFu32.to_be_bytes()); // connect_code
        pkt.extend_from_slice(&512u32.to_be_bytes()); // max_packet_size

        match parse_protocol_packet(&pkt).unwrap() {
            ProtocolPacket::SessionRequest { protocol_version, connect_code, max_packet_size } => {
                assert_eq!(protocol_version, 2);
                assert_eq!(connect_code, 0xDEADBEEF);
                assert_eq!(max_packet_size, 512);
            }
            _ => panic!("Expected SessionRequest"),
        }
    }

    #[test]
    fn build_and_verify_session_response() {
        let resp = build_session_response(0xDEADBEEF, 0x12345678, 2, 512);
        assert_eq!(resp.len(), 17);
        assert_eq!(resp[0], 0x00);
        assert_eq!(resp[1], 0x02); // OP_SessionResponse
        assert_eq!(resp[6], 0x12); // encode_key high byte
        assert_eq!(resp[10], 2); // crc_bytes
        assert_eq!(resp[11], 1); // encode_pass1 = Compression
        assert_eq!(resp[12], 0); // encode_pass2 = None
    }

    #[test]
    fn parse_combined_packet() {
        let mut pkt = vec![0x00, 0x03]; // OP_Combined
        // sub-packet 1: 3 bytes
        pkt.push(3);
        pkt.extend_from_slice(&[0xAA, 0xBB, 0xCC]);
        // sub-packet 2: 2 bytes
        pkt.push(2);
        pkt.extend_from_slice(&[0xDD, 0xEE]);

        match parse_protocol_packet(&pkt).unwrap() {
            ProtocolPacket::Combined { sub_packets } => {
                assert_eq!(sub_packets.len(), 2);
                assert_eq!(sub_packets[0], vec![0xAA, 0xBB, 0xCC]);
                assert_eq!(sub_packets[1], vec![0xDD, 0xEE]);
            }
            _ => panic!("Expected Combined"),
        }
    }

    #[test]
    fn parse_ack() {
        let pkt = vec![0x00, 0x15, 0x00, 0x05]; // ACK seq=5
        match parse_protocol_packet(&pkt).unwrap() {
            ProtocolPacket::Ack { sequence } => assert_eq!(sequence, 5),
            _ => panic!("Expected Ack"),
        }
    }
}
