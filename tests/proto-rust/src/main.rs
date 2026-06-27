mod generated {
    include!("generated/adif.rs");
}

use generated::*;
use prost::Message;

fn main() {
    let mut passed = 0u32;
    let mut failed = 0u32;

    macro_rules! assert_test {
        ($cond:expr, $name:expr) => {
            if $cond {
                println!("  PASS: {}", $name);
                passed += 1;
            } else {
                println!("  FAIL: {}", $name);
                failed += 1;
            }
        };
    }

    // Test 1: PositionUpdate round-trip
    println!("\n[Test 1] PositionUpdate round-trip");
    {
        let original = PositionUpdate {
            entity_id: 42,
            position: Some(Vec3 { x: 150.5, y: -320.75, z: 12.0 }),
            velocity: Some(Vec3 { x: 1.0, y: 0.0, z: 0.0 }),
            heading: 180.0,
            heading_delta: -2.5,
            animation: 1,
        };
        let bytes = original.encode_to_vec();
        let decoded = PositionUpdate::decode(bytes.as_slice()).unwrap();
        assert_test!(decoded.entity_id == 42, "entity_id preserved");
        assert_test!(decoded.position.as_ref().unwrap().x == 150.5, "position.x preserved");
        assert_test!(decoded.heading == 180.0, "heading preserved");
        println!("  Serialized size: {} bytes", bytes.len());
    }

    // Test 2: Spawn with expanded fields
    println!("\n[Test 2] Spawn with Phase 4 fields");
    {
        let original = Spawn {
            entity_id: 1001,
            entity_type: EntityType::Npc as i32,
            name: "a_froglok_warrior".into(),
            level: 15,
            race: 74,
            class_id: 1,
            current_hp: 750,
            max_hp: 750,
            run_speed: 1.25,
            walk_speed: 0.46,
            size: 6.0,
            findable: true,
            show_helm: true,
            title: "Guard".into(),
            bounding_radius: 5.5,
            ..Default::default()
        };
        let bytes = original.encode_to_vec();
        let decoded = Spawn::decode(bytes.as_slice()).unwrap();
        assert_test!(decoded.name == "a_froglok_warrior", "name preserved");
        assert_test!(decoded.run_speed == 1.25, "run_speed preserved");
        assert_test!(decoded.size == 6.0, "size preserved");
        assert_test!(decoded.findable, "findable preserved");
        assert_test!(decoded.title == "Guard", "title preserved");
        assert_test!(decoded.bounding_radius == 5.5, "bounding_radius preserved");
        println!("  Serialized size: {} bytes", bytes.len());
    }

    // Test 3: Packet envelope dispatch
    println!("\n[Test 3] Packet envelope with oneof");
    {
        let packet = Packet {
            sequence: 1,
            timestamp: 50000,
            payload: Some(packet::Payload::PositionUpdate(PositionUpdate {
                entity_id: 42,
                position: Some(Vec3 { x: 100.0, y: 200.0, z: 10.0 }),
                heading: 45.0,
                ..Default::default()
            })),
        };
        let bytes = packet.encode_to_vec();
        let decoded = Packet::decode(bytes.as_slice()).unwrap();
        assert_test!(decoded.sequence == 1, "sequence preserved");
        match &decoded.payload {
            Some(packet::Payload::PositionUpdate(pos)) => {
                assert_test!(pos.entity_id == 42, "nested entity_id preserved");
                assert_test!(pos.position.as_ref().unwrap().x == 100.0, "nested position.x preserved");
            }
            _ => { assert_test!(false, "oneof dispatch to PositionUpdate"); }
        }
        println!("  Serialized size: {} bytes", bytes.len());
    }

    // Test 4: IPC message
    println!("\n[Test 4] IPC ZoneBootRequest");
    {
        let msg = IpcMessage {
            source_zone_id: 0,
            target_zone_id: 1,
            timestamp: 90000,
            payload: Some(ipc_message::Payload::ZoneBootRequest(ZoneBootRequest {
                zone_id: 52,
                instance_id: 0,
                zone_short_name: "gukbottom".into(),
            })),
        };
        let bytes = msg.encode_to_vec();
        let decoded = IpcMessage::decode(bytes.as_slice()).unwrap();
        match &decoded.payload {
            Some(ipc_message::Payload::ZoneBootRequest(req)) => {
                assert_test!(req.zone_id == 52, "zone_id preserved");
                assert_test!(req.zone_short_name == "gukbottom", "zone_short_name preserved");
            }
            _ => { assert_test!(false, "IPC oneof dispatch"); }
        }
        println!("  Serialized size: {} bytes", bytes.len());
    }

    // Test 5: ItemDefinition with stats and effects
    println!("\n[Test 5] ItemDefinition (weapon with stats + proc)");
    {
        let item = ItemDefinition {
            item_id: 5023,
            name: "Sword of the Morning".into(),
            magic: true,
            lore: true,
            item_type: ItemType::Weapon1hSlash as i32,
            damage: 15,
            delay: 24,
            stats: Some(ItemStats {
                str: 10,
                hp: 50,
                haste: 21,
                fire_resist: 10,
                ..Default::default()
            }),
            effects: vec![ItemEffect {
                r#type: ItemEffectType::Proc as i32,
                spell_id: 1234,
                charges: -1,
                level: 50,
                recast_delay: 0,
            }],
            required_level: 20,
            price: 50000,
            ..Default::default()
        };
        let bytes = item.encode_to_vec();
        let decoded = ItemDefinition::decode(bytes.as_slice()).unwrap();
        assert_test!(decoded.name == "Sword of the Morning", "item name preserved");
        assert_test!(decoded.damage == 15, "damage preserved");
        assert_test!(decoded.stats.as_ref().unwrap().str == 10, "str stat preserved");
        assert_test!(decoded.stats.as_ref().unwrap().haste == 21, "haste preserved");
        assert_test!(decoded.effects.len() == 1, "1 effect");
        assert_test!(decoded.effects[0].spell_id == 1234, "proc spell_id preserved");
        println!("  Serialized size: {} bytes", bytes.len());
    }

    // Summary
    println!("\n{:=<40}", "");
    println!("Results: {} passed, {} failed", passed, failed);
    if failed > 0 {
        println!("SOME TESTS FAILED");
        std::process::exit(1);
    } else {
        println!("ALL TESTS PASSED");
    }
}
