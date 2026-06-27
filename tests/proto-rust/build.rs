use std::path::PathBuf;

fn main() {
    let proto_dir = PathBuf::from("../../proto");
    let proto_files: Vec<PathBuf> = [
        "adif/common.proto",
        "adif/connection.proto",
        "adif/zone.proto",
        "adif/entity.proto",
        "adif/character.proto",
        "adif/combat.proto",
        "adif/chat.proto",
        "adif/inventory.proto",
        "adif/world_objects.proto",
        "adif/trade.proto",
        "adif/group.proto",
        "adif/guild.proto",
        "adif/skills.proto",
        "adif/social.proto",
        "adif/admin.proto",
        "adif/packet.proto",
        "adif/ipc.proto",
    ]
    .iter()
    .map(|f| proto_dir.join(f))
    .collect();

    prost_build::Config::new()
        .out_dir("src/generated")
        .compile_protos(&proto_files, &[&proto_dir])
        .expect("Failed to compile proto files");
}
