using Google.Protobuf;
using Adif.Protocol;

int passed = 0;
int failed = 0;

void Assert(bool condition, string test)
{
    if (condition)
    {
        Console.WriteLine($"  PASS: {test}");
        passed++;
    }
    else
    {
        Console.WriteLine($"  FAIL: {test}");
        failed++;
    }
}

// ── Test 1: PositionUpdate round-trip ──────────────────────────
Console.WriteLine("\n[Test 1] PositionUpdate round-trip");
{
    var original = new PositionUpdate
    {
        EntityId = 42,
        Position = new Vec3 { X = 150.5f, Y = -320.75f, Z = 12.0f },
        Velocity = new Vec3 { X = 1.0f, Y = 0.0f, Z = 0.0f },
        Heading = 180.0f,
        HeadingDelta = -2.5f,
        Animation = 1
    };

    byte[] bytes = original.ToByteArray();
    var decoded = PositionUpdate.Parser.ParseFrom(bytes);

    Assert(decoded.EntityId == 42, "entity_id preserved");
    Assert(decoded.Position.X == 150.5f, "position.x preserved");
    Assert(decoded.Position.Y == -320.75f, "position.y preserved (negative)");
    Assert(decoded.Position.Z == 12.0f, "position.z preserved");
    Assert(decoded.Heading == 180.0f, "heading preserved");
    Assert(decoded.HeadingDelta == -2.5f, "heading_delta preserved (negative float)");
    Assert(decoded.Animation == 1, "animation preserved");

    Console.WriteLine($"  Serialized size: {bytes.Length} bytes");
}

// ── Test 2: Spawn with nested structs ──────────────────────────
Console.WriteLine("\n[Test 2] Spawn with nested structs");
{
    var original = new Spawn
    {
        EntityId = 1001,
        EntityType = EntityType.Npc,
        Name = "a_froglok_warrior",
        Level = 15,
        Race = 74,
        ClassId = 1,
        Gender = 0,
        Position = new Vec3 { X = -500.0f, Y = 200.0f, Z = -25.5f },
        Heading = 90.0f,
        CurrentHp = 750,
        MaxHp = 750,
        BodyType = 1,
        Appearance = new Appearance
        {
            HairColor = 0,
            Face = 3,
            SkinTint = new Color { R = 50, G = 120, B = 50, A = 255 }
        }
    };
    original.Equipment.Add(new EquipSlot
    {
        Slot = 0,
        ItemId = 5023,
        Tint = new Color { R = 180, G = 180, B = 180, A = 255 }
    });

    byte[] bytes = original.ToByteArray();
    var decoded = Spawn.Parser.ParseFrom(bytes);

    Assert(decoded.EntityId == 1001, "entity_id preserved");
    Assert(decoded.EntityType == EntityType.Npc, "entity_type is NPC");
    Assert(decoded.Name == "a_froglok_warrior", "name preserved");
    Assert(decoded.Level == 15, "level preserved");
    Assert(decoded.CurrentHp == 750, "hp preserved");
    Assert(decoded.Appearance.Face == 3, "nested appearance.face preserved");
    Assert(decoded.Appearance.SkinTint.G == 120, "deep nested skin_tint.g preserved");
    Assert(decoded.Equipment.Count == 1, "equipment list has 1 item");
    Assert(decoded.Equipment[0].ItemId == 5023, "equipment item_id preserved");

    Console.WriteLine($"  Serialized size: {bytes.Length} bytes (EQ equivalent: ~383 bytes)");
}

// ── Test 3: Packet envelope with oneof dispatch ────────────────
Console.WriteLine("\n[Test 3] Packet envelope with oneof dispatch");
{
    var packet = new Packet
    {
        Sequence = 1,
        Timestamp = 50000,
        PositionUpdate = new PositionUpdate
        {
            EntityId = 42,
            Position = new Vec3 { X = 100.0f, Y = 200.0f, Z = 10.0f },
            Heading = 45.0f
        }
    };

    byte[] bytes = packet.ToByteArray();
    var decoded = Packet.Parser.ParseFrom(bytes);

    Assert(decoded.Sequence == 1, "sequence preserved");
    Assert(decoded.Timestamp == 50000, "timestamp preserved");
    Assert(decoded.PayloadCase == Packet.PayloadOneofCase.PositionUpdate,
        "oneof dispatches to PositionUpdate");
    Assert(decoded.PositionUpdate.EntityId == 42,
        "nested position_update.entity_id preserved");
    Assert(decoded.PositionUpdate.Position.X == 100.0f,
        "deep nested position.x preserved");

    Console.WriteLine($"  Serialized size: {bytes.Length} bytes");
}

// ── Test 4: Zone transition flow ───────────────────────────────
Console.WriteLine("\n[Test 4] Zone transition packet flow");
{
    var request = new Packet
    {
        Sequence = 100,
        Timestamp = 60000,
        ZoneChangeRequest = new ZoneChangeRequest
        {
            TargetZoneId = 15,
            Trigger = ZoneChangeTrigger.Zoneline,
            Position = new Vec3 { X = 1200.0f, Y = -400.0f, Z = 0.0f }
        }
    };

    var response = new Packet
    {
        Sequence = 101,
        Timestamp = 60001,
        ZoneChangeResponse = new ZoneChangeResponse
        {
            Approved = true,
            TargetZoneId = 15,
            TargetPosition = new Vec3 { X = -50.0f, Y = 100.0f, Z = 5.0f },
            TargetHeading = 270.0f
        }
    };

    var reqBytes = request.ToByteArray();
    var respBytes = response.ToByteArray();
    var decodedReq = Packet.Parser.ParseFrom(reqBytes);
    var decodedResp = Packet.Parser.ParseFrom(respBytes);

    Assert(decodedReq.PayloadCase == Packet.PayloadOneofCase.ZoneChangeRequest,
        "request dispatches to ZoneChangeRequest");
    Assert(decodedReq.ZoneChangeRequest.Trigger == ZoneChangeTrigger.Zoneline,
        "trigger is ZONELINE");
    Assert(decodedResp.ZoneChangeResponse.Approved, "response approved");
    Assert(decodedResp.ZoneChangeResponse.TargetHeading == 270.0f,
        "target heading preserved");

    Console.WriteLine($"  Request: {reqBytes.Length} bytes, Response: {respBytes.Length} bytes");
}

// ── Test 5: Combat damage round-trip ───────────────────────────
Console.WriteLine("\n[Test 5] Combat damage round-trip");
{
    var packet = new Packet
    {
        Sequence = 200,
        Timestamp = 70000,
        MeleeDamage = new MeleeDamage
        {
            AttackerId = 42,
            TargetId = 1001,
            Damage = 127,
            DamageType = DamageType.Melee1HSlash,
            SpellId = 0,
            Force = new Vec3 { X = 0.5f, Y = 0.0f, Z = 0.1f }
        }
    };

    byte[] bytes = packet.ToByteArray();
    var decoded = Packet.Parser.ParseFrom(bytes);

    Assert(decoded.PayloadCase == Packet.PayloadOneofCase.MeleeDamage,
        "oneof dispatches to MeleeDamage");
    Assert(decoded.MeleeDamage.Damage == 127, "damage value preserved");
    Assert(decoded.MeleeDamage.DamageType == DamageType.Melee1HSlash,
        "damage type is 1H_SLASH");
    Assert(decoded.MeleeDamage.Force.X == 0.5f, "knockback force preserved");

    Console.WriteLine($"  Serialized size: {bytes.Length} bytes (EQ equivalent: ~24 bytes)");
}

// ── Test 6: Chat message ───────────────────────────────────────
Console.WriteLine("\n[Test 6] Chat message round-trip");
{
    var packet = new Packet
    {
        Sequence = 300,
        Timestamp = 80000,
        ChatMessage = new ChatMessage
        {
            SenderName = "Soandso",
            TargetName = "",
            Channel = ChatChannel.Say,
            Language = 0,
            Message = "Hail, a_froglok_warrior!"
        }
    };

    byte[] bytes = packet.ToByteArray();
    var decoded = Packet.Parser.ParseFrom(bytes);

    Assert(decoded.PayloadCase == Packet.PayloadOneofCase.ChatMessage,
        "oneof dispatches to ChatMessage");
    Assert(decoded.ChatMessage.SenderName == "Soandso", "sender preserved");
    Assert(decoded.ChatMessage.Message == "Hail, a_froglok_warrior!",
        "message text preserved");
    Assert(decoded.ChatMessage.Channel == ChatChannel.Say, "channel is SAY");

    Console.WriteLine($"  Serialized size: {bytes.Length} bytes");
}

// ── Test 7: IPC ZoneBootRequest round-trip ────────────────────
Console.WriteLine("\n[Test 7] IPC ZoneBootRequest round-trip");
{
    var original = new IpcMessage
    {
        SourceZoneId = 0,
        TargetZoneId = 1,
        Timestamp = 90000,
        ZoneBootRequest = new ZoneBootRequest
        {
            ZoneId = 52,
            InstanceId = 0,
            ZoneShortName = "gukbottom"
        }
    };

    byte[] bytes = original.ToByteArray();
    var decoded = IpcMessage.Parser.ParseFrom(bytes);

    Assert(decoded.SourceZoneId == 0, "source_zone_id preserved (world=0)");
    Assert(decoded.TargetZoneId == 1, "target_zone_id preserved");
    Assert(decoded.PayloadCase == IpcMessage.PayloadOneofCase.ZoneBootRequest,
        "oneof dispatches to ZoneBootRequest");
    Assert(decoded.ZoneBootRequest.ZoneId == 52, "zone_id preserved");
    Assert(decoded.ZoneBootRequest.ZoneShortName == "gukbottom", "zone_short_name preserved");

    Console.WriteLine($"  Serialized size: {bytes.Length} bytes");
}

// ── Test 8: IPC ChannelMessage round-trip ─────────────────────
Console.WriteLine("\n[Test 8] IPC ChannelMessage (cross-zone tell)");
{
    var original = new IpcMessage
    {
        SourceZoneId = 15,
        TargetZoneId = 0,
        Timestamp = 91000,
        IpcChannelMessage = new IpcChannelMessage
        {
            SenderName = "Ghouldan",
            TargetName = "Soandso",
            Channel = 2,
            Language = 0,
            Message = "Hey, are you in Grobb?",
            GuildId = 0,
            MinStatus = 0
        }
    };

    byte[] bytes = original.ToByteArray();
    var decoded = IpcMessage.Parser.ParseFrom(bytes);

    Assert(decoded.PayloadCase == IpcMessage.PayloadOneofCase.IpcChannelMessage,
        "oneof dispatches to IpcChannelMessage");
    Assert(decoded.IpcChannelMessage.SenderName == "Ghouldan", "sender preserved");
    Assert(decoded.IpcChannelMessage.TargetName == "Soandso", "target preserved");
    Assert(decoded.IpcChannelMessage.Message == "Hey, are you in Grobb?", "message preserved");

    Console.WriteLine($"  Serialized size: {bytes.Length} bytes");
}

// ── Test 9: IPC ZoneToZoneTransfer round-trip ─────────────────
Console.WriteLine("\n[Test 9] IPC ZoneToZoneTransfer");
{
    var original = new IpcMessage
    {
        SourceZoneId = 65,
        TargetZoneId = 0,
        Timestamp = 92000,
        ZoneToZoneTransfer = new ZoneToZoneTransfer
        {
            CharacterId = 1,
            CharacterName = "Ghouldan",
            CurrentZoneId = 65,
            TargetZoneId = 46,
            TargetInstanceId = 0,
            TargetPosition = new Vec3 { X = -500.0f, Y = 200.0f, Z = -10.0f },
            TargetHeading = 128.0f,
            Approved = true
        }
    };

    byte[] bytes = original.ToByteArray();
    var decoded = IpcMessage.Parser.ParseFrom(bytes);

    Assert(decoded.PayloadCase == IpcMessage.PayloadOneofCase.ZoneToZoneTransfer,
        "oneof dispatches to ZoneToZoneTransfer");
    Assert(decoded.ZoneToZoneTransfer.CharacterName == "Ghouldan", "character name preserved");
    Assert(decoded.ZoneToZoneTransfer.CurrentZoneId == 65, "current zone (grobb) preserved");
    Assert(decoded.ZoneToZoneTransfer.TargetZoneId == 46, "target zone (innothule) preserved");
    Assert(decoded.ZoneToZoneTransfer.Approved, "approved flag preserved");
    Assert(decoded.ZoneToZoneTransfer.TargetPosition.X == -500.0f, "target position preserved");

    Console.WriteLine($"  Serialized size: {bytes.Length} bytes");
}

// ── Test 10: IPC IncomingClient round-trip ─────────────────────
Console.WriteLine("\n[Test 10] IPC IncomingClient");
{
    var original = new IpcMessage
    {
        SourceZoneId = 0,
        TargetZoneId = 65,
        Timestamp = 93000,
        IncomingClient = new IncomingClient
        {
            AccountId = 1,
            AccountName = "darklight",
            CharacterId = 1,
            CharacterName = "Ghouldan",
            WorldClientId = 42,
            IpAddress = "127.0.0.1",
            GmStatus = 250,
            IsLocal = true
        }
    };

    byte[] bytes = original.ToByteArray();
    var decoded = IpcMessage.Parser.ParseFrom(bytes);

    Assert(decoded.PayloadCase == IpcMessage.PayloadOneofCase.IncomingClient,
        "oneof dispatches to IncomingClient");
    Assert(decoded.IncomingClient.CharacterName == "Ghouldan", "character name preserved");
    Assert(decoded.IncomingClient.GmStatus == 250, "GM status 250 preserved");
    Assert(decoded.IncomingClient.IsLocal, "is_local flag preserved");
    Assert(decoded.IncomingClient.WorldClientId == 42, "world client id preserved");

    Console.WriteLine($"  Serialized size: {bytes.Length} bytes");
}

// ── Summary ────────────────────────────────────────────────────
Console.WriteLine($"\n{'=',-40}");
Console.WriteLine($"Results: {passed} passed, {failed} failed");
if (failed > 0)
{
    Console.WriteLine("SOME TESTS FAILED");
    Environment.Exit(1);
}
else
{
    Console.WriteLine("ALL TESTS PASSED");
}
