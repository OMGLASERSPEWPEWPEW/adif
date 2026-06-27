use std::net::SocketAddr;

use tracing::{debug, info};

use super::codec;
use super::fragment::FragmentAssembler;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Connecting,
    Connected,
    Disconnecting,
}

pub struct EqSession {
    pub addr: SocketAddr,
    pub state: SessionState,
    pub connect_code: u32,
    pub encode_key: u32,
    pub crc_bytes: u8,
    pub max_packet_size: u32,
    pub sequence_in: u16,
    pub sequence_out: u16,
    pub last_ack_sent: u16,
    pub fragment_assembler: FragmentAssembler,
}

impl EqSession {
    pub fn new(addr: SocketAddr, connect_code: u32, max_packet_size: u32) -> Self {
        let encode_key = connect_code ^ 0x5A3C_96D7;
        Self {
            addr,
            state: SessionState::Connected,
            connect_code,
            encode_key,
            crc_bytes: 2,
            max_packet_size: max_packet_size.min(512),
            sequence_in: 0,
            sequence_out: 0,
            last_ack_sent: 0,
            fragment_assembler: FragmentAssembler::new(),
        }
    }

    pub fn decode_packet(&self, raw: &mut Vec<u8>) -> bool {
        if raw.len() < 2 || raw[1] == super::OP_SESSION_REQUEST {
            return true;
        }

        if !codec::verify_and_strip_crc(raw, self.encode_key, self.crc_bytes) {
            debug!(addr = %self.addr, "CRC verification failed");
            return false;
        }

        if raw.len() > 2 && raw[1] != super::OP_SESSION_DISCONNECT {
            let body = &raw[2..];
            if let Ok(decompressed) = codec::decompress(body) {
                raw.truncate(2);
                raw.extend_from_slice(&decompressed);
            }
        }

        true
    }

    pub fn next_sequence_out(&mut self) -> u16 {
        let seq = self.sequence_out;
        self.sequence_out = self.sequence_out.wrapping_add(1);
        seq
    }

    pub fn process_incoming_sequence(&mut self, sequence: u16) -> bool {
        if sequence == self.sequence_in {
            self.sequence_in = self.sequence_in.wrapping_add(1);
            true
        } else {
            debug!(
                expected = self.sequence_in,
                got = sequence,
                "Out-of-order packet"
            );
            false
        }
    }

    pub fn build_app_packet(&mut self, app_opcode: u16, app_data: &[u8]) -> Vec<u8> {
        let seq = self.next_sequence_out();

        let mut app_payload = Vec::new();
        app_payload.extend_from_slice(&app_opcode.to_le_bytes());
        app_payload.extend_from_slice(app_data);

        let compressed = codec::compress(&app_payload);

        let mut buf = Vec::new();
        buf.push(0x00);

        if compressed.len() + 6 > self.max_packet_size as usize {
            self.build_fragmented_packet(seq, &app_payload)
        } else {
            buf.push(super::OP_PACKET);
            buf.extend_from_slice(&seq.to_be_bytes());
            buf.extend_from_slice(&compressed);
            codec::append_crc(&mut buf, self.encode_key, self.crc_bytes);
            buf
        }
    }

    fn build_fragmented_packet(&mut self, _first_seq: u16, app_payload: &[u8]) -> Vec<u8> {
        // For MVP, send as single oversized packet — proper fragmentation in Phase 2
        let compressed = codec::compress(app_payload);
        let seq = self.sequence_out.wrapping_sub(1);

        let mut buf = Vec::new();
        buf.push(0x00);
        buf.push(super::OP_PACKET);
        buf.extend_from_slice(&seq.to_be_bytes());
        buf.extend_from_slice(&compressed);
        codec::append_crc(&mut buf, self.encode_key, self.crc_bytes);
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_creation() {
        let addr: SocketAddr = "127.0.0.1:5998".parse().unwrap();
        let session = EqSession::new(addr, 0xDEADBEEF, 512);
        assert_eq!(session.state, SessionState::Connected);
        assert_eq!(session.crc_bytes, 2);
        assert_eq!(session.max_packet_size, 512);
        assert_eq!(session.sequence_in, 0);
        assert_eq!(session.sequence_out, 0);
    }

    #[test]
    fn sequence_increments() {
        let addr: SocketAddr = "127.0.0.1:5998".parse().unwrap();
        let mut session = EqSession::new(addr, 0x1234, 512);
        assert_eq!(session.next_sequence_out(), 0);
        assert_eq!(session.next_sequence_out(), 1);
        assert_eq!(session.next_sequence_out(), 2);
    }

    #[test]
    fn incoming_sequence_tracking() {
        let addr: SocketAddr = "127.0.0.1:5998".parse().unwrap();
        let mut session = EqSession::new(addr, 0x1234, 512);
        assert!(session.process_incoming_sequence(0));
        assert!(session.process_incoming_sequence(1));
        assert!(!session.process_incoming_sequence(5)); // out of order
    }
}
