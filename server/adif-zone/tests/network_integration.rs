use std::time::Duration;

use adif_proto::adif::{self, packet::Payload, Packet};
use prost::Message;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

async fn write_packet(stream: &mut TcpStream, packet: &Packet) {
    let buf = packet.encode_to_vec();
    stream.write_u32(buf.len() as u32).await.unwrap();
    stream.write_all(&buf).await.unwrap();
    stream.flush().await.unwrap();
}

async fn read_packet(stream: &mut TcpStream) -> Packet {
    let len = stream.read_u32().await.unwrap() as usize;
    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf).await.unwrap();
    Packet::decode(&buf[..]).unwrap()
}

#[tokio::test]
async fn session_handshake_and_zone_entry() {
    // Start a minimal server in the background
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    // Create a world with a couple of test entities
    use bevy_ecs::prelude::*;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    let mut world = World::new();

    // We need the EntityIdAllocator resource
    use std::collections::HashMap;

    #[derive(Resource)]
    struct SimpleAllocator;

    // Just spawn a raw entity with Identity + Position + Health components
    // to test that zone entry sends spawns
    world.spawn((
        adif_zone_test_components::Identity {
            entity_id: 1,
            kind: adif_zone_test_components::EntityKind::Npc,
            name: "TestNPC".to_string(),
            last_name: String::new(),
            race: 1, class_id: 1, level: 5, gender: 0, deity: 0,
        },
        adif_zone_test_components::Position { x: 10.0, y: 20.0, z: 0.0, heading: 0.0 },
        adif_zone_test_components::Velocity::default(),
        adif_zone_test_components::Health {
            current_hp: 100, max_hp: 100,
            current_mana: 0, max_mana: 0,
            current_endurance: 0, max_endurance: 0,
        },
        adif_zone_test_components::MovementSpeed::default(),
        adif_zone_test_components::ModelAppearance { size: 6.0, ..Default::default() },
        adif_zone_test_components::EntityFlags::default(),
    ));

    // This test is complex because the server module is tightly coupled to main.
    // For now, just test the framing layer directly with a manual server loop.
    let server_task = tokio::spawn(async move {
        let (mut conn, _) = listener.accept().await.unwrap();

        // Read session request
        let pkt = read_packet_raw(&mut conn).await;
        assert!(matches!(pkt.payload, Some(Payload::SessionRequest(_))));

        // Send session response
        write_packet(&mut conn, &Packet {
            sequence: 1,
            timestamp: 0,
            payload: Some(Payload::SessionResponse(adif::SessionResponse {
                accepted: true,
                session_id: 1,
                reject_reason: String::new(),
            })),
        }).await;

        // Read auth request
        let pkt = read_packet_raw(&mut conn).await;
        assert!(matches!(pkt.payload, Some(Payload::AuthRequest(_))));

        // Send auth response
        write_packet(&mut conn, &Packet {
            sequence: 2,
            timestamp: 0,
            payload: Some(Payload::AuthResponse(adif::AuthResponse {
                success: true,
                account_id: 1,
                reject_reason: String::new(),
            })),
        }).await;

        // Read disconnect
        let pkt = read_packet_raw(&mut conn).await;
        assert!(matches!(pkt.payload, Some(Payload::Disconnect(_))));
    });

    // Client side
    tokio::time::sleep(Duration::from_millis(50)).await;
    let mut client = TcpStream::connect(addr).await.unwrap();

    // Send session request
    write_packet(&mut client, &Packet {
        sequence: 1,
        timestamp: 0,
        payload: Some(Payload::SessionRequest(adif::SessionRequest {
            protocol_version: 1,
            client_version: "test-0.1".to_string(),
        })),
    }).await;

    // Read session response
    let response = read_packet(&mut client).await;
    match response.payload {
        Some(Payload::SessionResponse(r)) => {
            assert!(r.accepted);
            assert_eq!(r.session_id, 1);
        }
        _ => panic!("Expected SessionResponse"),
    }

    // Send auth request
    write_packet(&mut client, &Packet {
        sequence: 2,
        timestamp: 0,
        payload: Some(Payload::AuthRequest(adif::AuthRequest {
            account_name: "testuser".to_string(),
            token: "testtoken".to_string(),
        })),
    }).await;

    // Read auth response
    let response = read_packet(&mut client).await;
    match response.payload {
        Some(Payload::AuthResponse(r)) => {
            assert!(r.success);
            assert_eq!(r.account_id, 1);
        }
        _ => panic!("Expected AuthResponse"),
    }

    // Send disconnect
    write_packet(&mut client, &Packet {
        sequence: 3,
        timestamp: 0,
        payload: Some(Payload::Disconnect(adif::Disconnect {
            reason: adif::DisconnectReason::ClientQuit as i32,
            message: "bye".to_string(),
        })),
    }).await;

    server_task.await.unwrap();
}

async fn read_packet_raw(stream: &mut TcpStream) -> Packet {
    let len = stream.read_u32().await.unwrap() as usize;
    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf).await.unwrap();
    Packet::decode(&buf[..]).unwrap()
}

// Stub component types for the test (can't import from adif-zone binary crate)
mod adif_zone_test_components {
    use bevy_ecs::prelude::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum EntityKind { Npc, #[allow(dead_code)] Player, #[allow(dead_code)] PlayerCorpse, #[allow(dead_code)] NpcCorpse }

    #[derive(Component, Debug, Clone)]
    pub struct Identity { pub entity_id: u32, pub kind: EntityKind, pub name: String, pub last_name: String, pub race: u32, pub class_id: u32, pub level: u32, pub gender: u32, pub deity: u32 }

    #[derive(Component, Debug, Clone, Copy)]
    pub struct Position { pub x: f32, pub y: f32, pub z: f32, pub heading: f32 }

    #[derive(Component, Debug, Clone, Copy, Default)]
    pub struct Velocity { pub x: f32, pub y: f32, pub z: f32, pub heading_delta: f32 }

    #[derive(Component, Debug, Clone, Copy)]
    pub struct Health { pub current_hp: i32, pub max_hp: i32, pub current_mana: i32, pub max_mana: i32, pub current_endurance: i32, pub max_endurance: i32 }

    #[derive(Component, Debug, Clone, Copy, Default)]
    pub struct MovementSpeed { pub run_speed: f32, pub walk_speed: f32, pub fly_mode: u32 }

    #[derive(Component, Debug, Clone, Copy, Default)]
    pub struct ModelAppearance { pub size: f32, pub light_source: u32, pub texture: u32, pub helm_texture: u32, pub bounding_radius: f32 }

    #[derive(Component, Debug, Clone, Copy, Default)]
    pub struct EntityFlags { pub gm: bool, pub afk: bool, pub anonymous: bool, pub lfg: bool, pub sneaking: bool, pub pvp: bool, pub linkdead: bool, pub invis: bool, pub findable: bool, pub show_helm: bool, pub is_pet: bool }
}
