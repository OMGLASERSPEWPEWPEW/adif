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

/// Full scenario: connect → session → auth → zone entry (receive spawns) →
/// send chat → send heartbeat → disconnect
#[tokio::test]
async fn full_client_lifecycle() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let server_task = tokio::spawn(async move {
        let (mut conn, _) = listener.accept().await.unwrap();

        // Session handshake
        let pkt = read_pkt(&mut conn).await;
        assert!(matches!(pkt.payload, Some(Payload::SessionRequest(_))));

        write_packet(&mut conn, &Packet {
            sequence: 1, timestamp: 0,
            payload: Some(Payload::SessionResponse(adif::SessionResponse {
                accepted: true, session_id: 1, reject_reason: String::new(),
            })),
        }).await;

        // Auth
        let pkt = read_pkt(&mut conn).await;
        match &pkt.payload {
            Some(Payload::AuthRequest(a)) => assert_eq!(a.account_name, "TestUser"),
            _ => panic!("Expected AuthRequest"),
        }

        write_packet(&mut conn, &Packet {
            sequence: 2, timestamp: 0,
            payload: Some(Payload::AuthResponse(adif::AuthResponse {
                success: true, account_id: 42, reject_reason: String::new(),
            })),
        }).await;

        // Zone entry — send 2 NPC spawns + ZoneReady
        let pkt = read_pkt(&mut conn).await;
        assert!(matches!(pkt.payload, Some(Payload::ZoneEntryRequest(_))));

        for i in 1..=2 {
            write_packet(&mut conn, &Packet {
                sequence: 2 + i, timestamp: 0,
                payload: Some(Payload::Spawn(adif::Spawn {
                    entity_id: i as u32,
                    entity_type: adif::EntityType::Npc as i32,
                    name: format!("npc_{i}"),
                    level: 5,
                    current_hp: 100,
                    max_hp: 100,
                    position: Some(adif::Vec3 { x: i as f32 * 10.0, y: 0.0, z: 0.0 }),
                    ..Default::default()
                })),
            }).await;
        }

        write_packet(&mut conn, &Packet {
            sequence: 5, timestamp: 0,
            payload: Some(Payload::ZoneReady(adif::ZoneReady { zone_id: 52 })),
        }).await;

        // Read chat
        let pkt = read_pkt(&mut conn).await;
        match &pkt.payload {
            Some(Payload::ChatMessage(m)) => {
                assert_eq!(m.sender_name, "TestUser");
                assert_eq!(m.message, "Hello Norrath!");
                assert_eq!(m.channel, adif::ChatChannel::Shout as i32);
            }
            _ => panic!("Expected ChatMessage, got {:?}", pkt.payload),
        }

        // Read heartbeat
        let pkt = read_pkt(&mut conn).await;
        assert!(matches!(pkt.payload, Some(Payload::Heartbeat(_))));

        // Read position update
        let pkt = read_pkt(&mut conn).await;
        match &pkt.payload {
            Some(Payload::PositionUpdate(u)) => {
                assert_eq!(u.entity_id, 100);
                let pos = u.position.as_ref().unwrap();
                assert!((pos.x - 50.0).abs() < f32::EPSILON);
            }
            _ => panic!("Expected PositionUpdate"),
        }

        // Read disconnect
        let pkt = read_pkt(&mut conn).await;
        match &pkt.payload {
            Some(Payload::Disconnect(d)) => {
                assert_eq!(d.reason, adif::DisconnectReason::ClientQuit as i32);
            }
            _ => panic!("Expected Disconnect"),
        }
    });

    // --- CLIENT SIDE ---
    tokio::time::sleep(Duration::from_millis(50)).await;
    let mut client = TcpStream::connect(addr).await.unwrap();

    // 1. Session request
    write_packet(&mut client, &Packet {
        sequence: 1, timestamp: 0,
        payload: Some(Payload::SessionRequest(adif::SessionRequest {
            protocol_version: 1, client_version: "test-0.1".to_string(),
        })),
    }).await;

    let resp = read_packet(&mut client).await;
    match resp.payload {
        Some(Payload::SessionResponse(r)) => {
            assert!(r.accepted);
            assert_eq!(r.session_id, 1);
        }
        _ => panic!("Expected SessionResponse"),
    }

    // 2. Auth
    write_packet(&mut client, &Packet {
        sequence: 2, timestamp: 0,
        payload: Some(Payload::AuthRequest(adif::AuthRequest {
            account_name: "TestUser".to_string(), token: "abc".to_string(),
        })),
    }).await;

    let resp = read_packet(&mut client).await;
    match resp.payload {
        Some(Payload::AuthResponse(r)) => {
            assert!(r.success);
            assert_eq!(r.account_id, 42);
        }
        _ => panic!("Expected AuthResponse"),
    }

    // 3. Zone entry
    write_packet(&mut client, &Packet {
        sequence: 3, timestamp: 0,
        payload: Some(Payload::ZoneEntryRequest(adif::ZoneEntryRequest {
            character_name: "TestUser".to_string(), zone_id: 52,
        })),
    }).await;

    // Receive 2 spawns + ZoneReady
    let mut spawn_count = 0;
    loop {
        let pkt = read_packet(&mut client).await;
        match pkt.payload {
            Some(Payload::Spawn(s)) => {
                spawn_count += 1;
                assert!(s.entity_id > 0);
                assert!(s.name.starts_with("npc_"));
            }
            Some(Payload::ZoneReady(r)) => {
                assert_eq!(r.zone_id, 52);
                break;
            }
            other => panic!("Unexpected packet during zone entry: {:?}", other),
        }
    }
    assert_eq!(spawn_count, 2);

    // 4. Send chat (shout)
    write_packet(&mut client, &Packet {
        sequence: 4, timestamp: 0,
        payload: Some(Payload::ChatMessage(adif::ChatMessage {
            sender_name: "TestUser".to_string(),
            target_name: String::new(),
            channel: adif::ChatChannel::Shout as i32,
            language: 0,
            message: "Hello Norrath!".to_string(),
        })),
    }).await;

    // 5. Heartbeat
    write_packet(&mut client, &Packet {
        sequence: 5, timestamp: 0,
        payload: Some(Payload::Heartbeat(adif::Heartbeat { session_id: 1 })),
    }).await;

    // 6. Position update
    write_packet(&mut client, &Packet {
        sequence: 6, timestamp: 0,
        payload: Some(Payload::PositionUpdate(adif::PositionUpdate {
            entity_id: 100,
            position: Some(adif::Vec3 { x: 50.0, y: 25.0, z: 0.0 }),
            velocity: Some(adif::Vec3 { x: 0.0, y: 0.0, z: 0.0 }),
            heading: 90.0,
            heading_delta: 0.0,
            animation: 0,
        })),
    }).await;

    // 7. Disconnect
    write_packet(&mut client, &Packet {
        sequence: 7, timestamp: 0,
        payload: Some(Payload::Disconnect(adif::Disconnect {
            reason: adif::DisconnectReason::ClientQuit as i32,
            message: "goodbye".to_string(),
        })),
    }).await;

    server_task.await.unwrap();
}

async fn read_pkt(stream: &mut TcpStream) -> Packet {
    let len = stream.read_u32().await.unwrap() as usize;
    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf).await.unwrap();
    Packet::decode(&buf[..]).unwrap()
}
