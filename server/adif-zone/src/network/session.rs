use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;
use tracing::{info, warn};

use adif_proto::adif::Packet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Connected,
    Authenticated,
    InZone,
}

#[derive(Debug)]
pub struct ClientInfo {
    pub session_id: u32,
    pub state: SessionState,
    pub account_id: u32,
    pub account_name: String,
    pub entity_id: Option<u32>,
    pub addr: std::net::SocketAddr,
}

pub type OutboundQueue = Arc<Mutex<Vec<Packet>>>;

pub struct SessionManager {
    next_session_id: u32,
    sessions: HashMap<u32, ClientInfo>,
    outbound: HashMap<u32, OutboundQueue>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            next_session_id: 1,
            sessions: HashMap::new(),
            outbound: HashMap::new(),
        }
    }

    pub fn create_session(&mut self, addr: std::net::SocketAddr) -> (u32, OutboundQueue) {
        let id = self.next_session_id;
        self.next_session_id += 1;

        let queue = Arc::new(Mutex::new(Vec::new()));
        self.sessions.insert(id, ClientInfo {
            session_id: id,
            state: SessionState::Connected,
            account_id: 0,
            account_name: String::new(),
            entity_id: None,
            addr,
        });
        self.outbound.insert(id, Arc::clone(&queue));

        info!(session = id, addr = %addr, "Session created");
        (id, queue)
    }

    pub fn authenticate(&mut self, session_id: u32, account_id: u32, account_name: String) {
        if let Some(info) = self.sessions.get_mut(&session_id) {
            info.state = SessionState::Authenticated;
            info.account_id = account_id;
            info.account_name = account_name;
        }
    }

    pub fn enter_zone(&mut self, session_id: u32, entity_id: u32) {
        if let Some(info) = self.sessions.get_mut(&session_id) {
            info.state = SessionState::InZone;
            info.entity_id = Some(entity_id);
        }
    }

    pub fn remove_session(&mut self, session_id: u32) {
        if let Some(info) = self.sessions.remove(&session_id) {
            self.outbound.remove(&session_id);
            info!(session = session_id, addr = %info.addr, "Session removed");
        }
    }

    pub fn get(&self, session_id: u32) -> Option<&ClientInfo> {
        self.sessions.get(&session_id)
    }

    pub fn session_count(&self) -> usize {
        self.sessions.len()
    }

    pub async fn queue_packet(&self, session_id: u32, packet: Packet) {
        if let Some(queue) = self.outbound.get(&session_id) {
            queue.lock().await.push(packet);
        } else {
            warn!(session = session_id, "No outbound queue for session");
        }
    }

    pub async fn broadcast(&self, packet: Packet) {
        for (_, queue) in &self.outbound {
            queue.lock().await.push(packet.clone());
        }
    }

    pub fn all_session_ids(&self) -> Vec<u32> {
        self.sessions.keys().copied().collect()
    }
}
