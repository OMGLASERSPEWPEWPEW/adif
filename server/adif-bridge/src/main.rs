use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::Context;
use rand::Rng;
use sqlx;
use tokio::net::UdpSocket;
use tracing::{debug, info, warn};

fn hex_preview(data: &[u8]) -> String {
    let len = data.len().min(48);
    data[..len]
        .iter()
        .map(|b| format!("{b:02X}"))
        .collect::<Vec<_>>()
        .join(" ")
}

mod eq_protocol;
mod login_handler;
mod titanium;
mod world_handler;

use eq_protocol::packet::{self, ProtocolPacket};
use eq_protocol::session::EqSession;
use titanium::opcodes;
use titanium::structs::{self, InventoryItemRow, PlayerProfileData, SpawnData, ZoneData};

use adif_world::WorldState;

const INVENTORY_QUERY: &str = r#"SELECT
    i.slot_id, i.item_id, i.charges,
    it.itemclass, it.name, it.lore, it.idfile, it.id AS item_db_id,
    it.weight, it.norent, it.nodrop, it.size, it.slots, it.price, it.icon,
    it.benefitflag, it.tradeskills,
    it.cr, it.dr, it.pr, it.mr, it.fr,
    it.astr, it.asta, it.aagi, it.adex, it.acha, it.aint, it.awis,
    it.hp, it.mana, it.ac, it.deity,
    it.skillmodvalue, it.skillmodmax, it.skillmodtype,
    it.banedmgrace, it.banedmgamt, it.banedmgbody,
    it.magic, it.casttime_, it.reqlevel, it.bardtype, it.bardvalue,
    it.light, it.delay, it.reclevel, it.recskill,
    it.elemdmgtype, it.elemdmgamt, it.range, it.damage,
    it.color, it.classes, it.races,
    it.maxcharges, it.itemtype, it.material, it.sellrate,
    it.procrate, it.combateffects, it.shielding, it.stunresist,
    it.strikethrough, it.extradmgskill, it.extradmgamt,
    it.spellshield, it.avoidance, it.accuracy,
    it.charmfileid,
    it.factionmod1, it.factionmod2, it.factionmod3, it.factionmod4,
    it.factionamt1, it.factionamt2, it.factionamt3, it.factionamt4,
    it.charmfile,
    it.augtype,
    it.augslot1type, it.augslot1visible, it.augslot2type, it.augslot2visible,
    it.augslot3type, it.augslot3visible, it.augslot4type, it.augslot4visible,
    it.augslot5type, it.augslot5visible,
    it.ldontheme, it.ldonprice, it.ldonsold,
    it.bagtype, it.bagslots, it.bagsize, it.bagwr,
    it.book, it.booktype, it.filename,
    it.banedmgraceamt, it.augrestrict, it.loregroup, it.pendingloreflag,
    it.artifactflag, it.summonedflag,
    it.favor, it.fvnodrop, it.endur, it.dotshielding,
    it.attack, it.regen, it.manaregen, it.enduranceregen,
    it.haste, it.damageshield, it.recastdelay, it.recasttype, it.guildfavor,
    it.augdistiller, it.attuneable, it.nopet, it.pointtype,
    it.potionbelt, it.potionbeltslots, it.stacksize, it.notransfer, it.stackable,
    it.clickeffect, it.clicktype, it.clicklevel2, it.clicklevel,
    it.proceffect, it.proctype, it.proclevel2, it.proclevel,
    it.worneffect, it.worntype, it.wornlevel2, it.wornlevel,
    it.focuseffect, it.focustype, it.focuslevel2, it.focuslevel,
    it.scrolleffect, it.scrolltype, it.scrolllevel2, it.scrolllevel
FROM inventory i
JOIN items it ON i.item_id = it.id
WHERE i.character_id = $1
  AND i.slot_id >= 0 AND i.slot_id <= 33
ORDER BY i.slot_id"#;

#[derive(Debug, sqlx::FromRow)]
struct ZoneDbRow {
    short_name: String,
    long_name: String,
    safe_x: f32,
    safe_y: f32,
    safe_z: f32,
    minclip: f32,
    maxclip: f32,
    fog_minclip: f32,
    fog_maxclip: f32,
    fog_minclip2: f32,
    fog_maxclip2: f32,
    fog_minclip3: f32,
    fog_maxclip3: f32,
    fog_minclip4: f32,
    fog_maxclip4: f32,
    fog_red: i16,
    fog_green: i16,
    fog_blue: i16,
    fog_red2: i16,
    fog_green2: i16,
    fog_blue2: i16,
    fog_red3: i16,
    fog_green3: i16,
    fog_blue3: i16,
    fog_red4: i16,
    fog_green4: i16,
    fog_blue4: i16,
    fog_density: f32,
    sky: i16,
    ztype: i16,
    zone_exp_multiplier: f32,
    gravity: f32,
    time_type: i16,
    rain_chance1: i32,
    rain_chance2: i32,
    rain_chance3: i32,
    rain_chance4: i32,
    rain_duration1: i32,
    rain_duration2: i32,
    rain_duration3: i32,
    rain_duration4: i32,
    snow_chance1: i32,
    snow_chance2: i32,
    snow_chance3: i32,
    snow_chance4: i32,
    snow_duration1: i32,
    snow_duration2: i32,
    snow_duration3: i32,
    snow_duration4: i32,
    underworld: f32,
    max_z: i32,
}

#[derive(Debug, sqlx::FromRow)]
struct ObjectRow {
    id: i32,
    xpos: f32,
    ypos: f32,
    zpos: f32,
    heading: f32,
    objectname: String,
    #[sqlx(rename = "type")]
    object_type: i32,
    size: f32,
    incline: i32,
    tilt_x: f32,
    tilt_y: f32,
}

#[derive(Debug, sqlx::FromRow)]
struct DoorRow {
    name: String,
    pos_y: f32,
    pos_x: f32,
    pos_z: f32,
    heading: f32,
    incline: i32,
    size: i16,
    doorid: i16,
    opentype: i16,
    invert_state: i32,
    door_param: i32,
}

#[derive(Debug, sqlx::FromRow)]
struct ZonePointRow {
    number: i32,
    target_y: f32,
    target_x: f32,
    target_z: f32,
    target_heading: f32,
    target_zone_id: i32,
    target_instance: i32,
}

struct SpawnedNpcInfo {
    level: u8,
    name: String,
    cur_hp: i64,
    max_hp: i64,
    min_dmg: i32,
    max_dmg: i32,
    attack_delay: i16,
    loottable_id: i32,
    is_corpse: bool,
    loot_items: Vec<structs::InventoryItemRow>,
    loot_platinum: u32,
    loot_gold: u32,
    loot_silver: u32,
    loot_copper: u32,
}

#[derive(Debug, sqlx::FromRow)]
struct ZoneSpawnRow {
    npc_name: String,
    lastname: Option<String>,
    level: i16,
    race: i16,
    class: i16,
    gender: i16,
    bodytype: i32,
    hp: i64,
    size: f32,
    runspeed: f32,
    walkspeed: f32,
    texture: i16,
    helmtexture: i16,
    light: i16,
    findable: i16,
    flymode: i16,
    mindmg: i32,
    maxdmg: i32,
    attack_delay: i16,
    loottable_id: i32,
    x: f32,
    y: f32,
    z: f32,
    heading: f32,
}

const LOGIN_PORT: u16 = 5998;
const WORLD_PORT: u16 = 9000;
const ZONE_PORT: u16 = 7778;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConnectionPhase {
    Login,
    World,
    Zone,
}

struct ClientState {
    phase: ConnectionPhase,
    account_id: Option<i32>,
    account_name: String,
    char_name: String,
    char_zone_id: Option<i32>,
    char_zone_short: String,
    player_spawn_id: u32,
    next_spawn_id: u32,
    last_x: f32,
    last_y: f32,
    last_z: f32,
    last_heading: f32,
    player_level: u8,
    target_id: Option<u32>,
    spawned_npcs: HashMap<u32, SpawnedNpcInfo>,
    auto_attack: bool,
    last_attack_time: Option<Instant>,
    attack_delay_ms: u32,
    zone_transition_pending: bool,
    exp_ratio: u32,
}

impl ClientState {
    fn new(phase: ConnectionPhase) -> Self {
        Self {
            phase,
            account_id: None,
            account_name: String::new(),
            char_name: String::new(),
            char_zone_id: None,
            char_zone_short: String::new(),
            player_spawn_id: 0,
            next_spawn_id: 1,
            last_x: 0.0,
            last_y: 0.0,
            last_z: 0.0,
            last_heading: 0.0,
            player_level: 0,
            target_id: None,
            spawned_npcs: HashMap::new(),
            auto_attack: false,
            last_attack_time: None,
            attack_delay_ms: 4000,
            zone_transition_pending: false,
            exp_ratio: 0,
        }
    }

    fn alloc_spawn_id(&mut self) -> u32 {
        let id = self.next_spawn_id;
        self.next_spawn_id += 1;
        id
    }
}

struct PhaseState {
    sessions: HashMap<SocketAddr, EqSession>,
    client_states: HashMap<SocketAddr, ClientState>,
}

impl PhaseState {
    fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            client_states: HashMap::new(),
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config_path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("server.toml"));

    let config = adif_common::ServerConfig::load(&config_path)
        .with_context(|| format!("Failed to load config from {}", config_path.display()))?;

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| config.server.log_level.parse().unwrap_or_default()),
        )
        .init();

    info!(name = %config.server.name, "ADIF Protocol Bridge starting");

    let pool = adif_common::create_pool(&config.database)
        .await
        .context("Failed to connect to PostgreSQL")?;
    info!("Connected to PostgreSQL");

    let world_state = Arc::new(WorldState::new(
        pool,
        config.server.name.clone(),
        "Welcome to ADIF - Another Day In Forever".to_string(),
    ));

    {
        let mut registry = world_state.zone_registry.write().await;
        let zone_addr = SocketAddr::new(
            std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST),
            ZONE_PORT,
        );
        registry.register(52, "grobb".to_string(), zone_addr);
        registry.register(46, "innothule".to_string(), zone_addr);
    }

    let login_socket = UdpSocket::bind(format!("0.0.0.0:{LOGIN_PORT}")).await?;
    let world_socket = UdpSocket::bind(format!("0.0.0.0:{WORLD_PORT}")).await?;
    let zone_socket = UdpSocket::bind(format!("0.0.0.0:{ZONE_PORT}")).await?;

    info!(login = LOGIN_PORT, world = WORLD_PORT, zone = ZONE_PORT, "UDP listeners bound");

    let mut login_state = PhaseState::new();
    let mut world_state_phase = PhaseState::new();
    let mut zone_state = PhaseState::new();

    let mut login_buf = vec![0u8; 8192];
    let mut world_buf = vec![0u8; 8192];
    let mut zone_buf = vec![0u8; 8192];
    let mut combat_tick = tokio::time::interval(Duration::from_millis(250));

    loop {
        tokio::select! {
            result = login_socket.recv_from(&mut login_buf) => {
                let (len, addr) = result?;
                handle_udp_packet(
                    &login_buf[..len], addr, ConnectionPhase::Login,
                    &mut login_state, &login_socket, &world_state,
                ).await?;
            }
            result = world_socket.recv_from(&mut world_buf) => {
                let (len, addr) = result?;
                handle_udp_packet(
                    &world_buf[..len], addr, ConnectionPhase::World,
                    &mut world_state_phase, &world_socket, &world_state,
                ).await?;
            }
            result = zone_socket.recv_from(&mut zone_buf) => {
                let (len, addr) = result?;
                handle_udp_packet(
                    &zone_buf[..len], addr, ConnectionPhase::Zone,
                    &mut zone_state, &zone_socket, &world_state,
                ).await?;
            }
            _ = combat_tick.tick() => {
                if let Err(e) = process_combat_tick(&mut zone_state, &zone_socket).await {
                    warn!(error = %e, "Combat tick error");
                }
            }
        }
    }
}

async fn process_combat_tick(
    state: &mut PhaseState,
    socket: &UdpSocket,
) -> anyhow::Result<()> {
    let now = Instant::now();
    let PhaseState { sessions, client_states } = state;

    let mut packets_to_send: Vec<(SocketAddr, Vec<(u16, Vec<u8>)>)> = Vec::new();

    for (addr, cs) in client_states.iter_mut() {
        if !cs.auto_attack {
            continue;
        }
        let target_id = match cs.target_id {
            Some(id) if id != cs.player_spawn_id => id,
            _ => { cs.auto_attack = false; continue; }
        };
        if let Some(last) = cs.last_attack_time {
            if now.duration_since(last) < Duration::from_millis(cs.attack_delay_ms as u64) {
                continue;
            }
        }
        let player_spawn_id = cs.player_spawn_id;
        let player_level = cs.player_level;
        let heading = cs.last_heading;

        let (dead, npc_level, mut pkts) = {
            let npc = match cs.spawned_npcs.get_mut(&target_id) {
                Some(n) if n.cur_hp > 0 && !n.is_corpse => n,
                _ => { cs.auto_attack = false; continue; }
            };

            let damage = rand::thread_rng().gen_range(
                (player_level.max(1) as i32)..=((player_level as i32) * 3).max(1)
            );
            npc.cur_hp = (npc.cur_hp - damage as i64).max(0);
            let hp_pct = ((npc.cur_hp * 100) / npc.max_hp.max(1)).clamp(0, 100) as u8;
            let dead = npc.cur_hp <= 0;

            info!(
                target = %npc.name, damage, hp_remaining = npc.cur_hp,
                hp_pct, "Combat: hit"
            );

            let mut pkts: Vec<(u16, Vec<u8>)> = Vec::new();
            pkts.push((opcodes::OP_ANIMATION, structs::build_animation(
                player_spawn_id as u16, 10, 8,
            )));
            pkts.push((opcodes::OP_DAMAGE, structs::build_combat_damage(
                target_id as u16, player_spawn_id as u16,
                0, 0xFFFF, damage as u32, 0.0, heading, 0.0,
            )));
            pkts.push((opcodes::OP_MOB_HEALTH, structs::build_mob_health(
                target_id as i16, hp_pct,
            )));

            if dead {
                pkts.push((opcodes::OP_DEATH, structs::build_death_struct(
                    target_id, player_spawn_id, target_id,
                    damage as u32, 0xFFFFFFFF, 0,
                )));
            }

            (dead, npc.level, pkts)
        };

        cs.last_attack_time = Some(now);
        if dead {
            cs.auto_attack = false;
            if let Some(npc) = cs.spawned_npcs.get_mut(&target_id) {
                npc.is_corpse = true;
                npc.cur_hp = 0;
            }
            let xp_gain = (npc_level as u32) * 15;
            cs.exp_ratio = (cs.exp_ratio + xp_gain).min(330);
            let mut exp_buf = vec![0u8; 8];
            exp_buf[0..4].copy_from_slice(&cs.exp_ratio.to_le_bytes());
            pkts.push((opcodes::OP_EXP_UPDATE, exp_buf));
            info!(target_id, npc_level, xp_gain, exp_ratio = cs.exp_ratio, "Combat: target killed");
        }

        packets_to_send.push((*addr, pkts));
    }

    for (addr, pkts) in packets_to_send {
        if let Some(session) = sessions.get_mut(&addr) {
            for (opcode, data) in pkts {
                send_app_packet(session, socket, addr, opcode, &data).await?;
            }
        }
    }

    Ok(())
}

fn split_money(total_copper: u32) -> (u32, u32, u32, u32) {
    let plat = total_copper / 1000;
    let gold = (total_copper % 1000) / 100;
    let silver = (total_copper % 100) / 10;
    let copper = total_copper % 10;
    (plat, gold, silver, copper)
}

#[derive(Debug, sqlx::FromRow)]
struct LootMoneyRow {
    mincash: i32,
    maxcash: i32,
}

#[derive(Debug, sqlx::FromRow)]
struct LootCandidateRow {
    lootdrop_id: i32,
    group_probability: f32,
    group_multiplier: i32,
    group_droplimit: i32,
    item_chance: f32,
    item_charges: i32,
    itemclass: i32,
    name: String,
    lore: String,
    idfile: String,
    item_db_id: i32,
    weight: i32,
    norent: i32,
    nodrop: i32,
    size: i32,
    slots: i32,
    price: i32,
    icon: i32,
    benefitflag: i32,
    tradeskills: i32,
    cr: i32, dr: i32, pr: i32, mr: i32, fr: i32,
    astr: i32, asta: i32, aagi: i32, adex: i32, acha: i32, aint: i32, awis: i32,
    hp: i32, mana: i32, ac: i32, deity: i32,
    skillmodvalue: i32, skillmodmax: i32, skillmodtype: i32,
    banedmgrace: i32, banedmgamt: i32, banedmgbody: i32,
    magic: i32, casttime_: i32, reqlevel: i32, bardtype: i32, bardvalue: i32,
    light: i32, delay: i32, reclevel: i32, recskill: i32,
    elemdmgtype: i32, elemdmgamt: i32, range: i32, damage: i32,
    color: i64, classes: i32, races: i32,
    maxcharges: i32, itemtype: i32, material: i32, sellrate: f32,
    procrate: i32, combateffects: String, shielding: i32, stunresist: i32,
    strikethrough: i32, extradmgskill: i32, extradmgamt: i32,
    spellshield: i32, avoidance: i32, accuracy: i32,
    charmfileid: String,
    factionmod1: i32, factionmod2: i32, factionmod3: i32, factionmod4: i32,
    factionamt1: i32, factionamt2: i32, factionamt3: i32, factionamt4: i32,
    charmfile: String,
    augtype: i32,
    augslot1type: i16, augslot1visible: i16,
    augslot2type: i16, augslot2visible: i16,
    augslot3type: i16, augslot3visible: i16,
    augslot4type: i16, augslot4visible: i16,
    augslot5type: i16, augslot5visible: i16,
    ldontheme: i32, ldonprice: i32, ldonsold: i32,
    bagtype: i32, bagslots: i32, bagsize: i32, bagwr: i32,
    book: i32, booktype: i32, filename: String,
    banedmgraceamt: i32, augrestrict: i32, loregroup: i32,
    pendingloreflag: i16, artifactflag: i16, summonedflag: i16,
    favor: i32, fvnodrop: i32, endur: i32, dotshielding: i32,
    attack: i32, regen: i32, manaregen: i32, enduranceregen: i32,
    haste: i32, damageshield: i32, recastdelay: i32, recasttype: i32, guildfavor: i32,
    augdistiller: i32, attuneable: i32, nopet: i32, pointtype: i32,
    potionbelt: i32, potionbeltslots: i32, stacksize: i32, notransfer: i32, stackable: i32,
    clickeffect: i32, clicktype: i32, clicklevel2: i32, clicklevel: i32,
    proceffect: i32, proctype: i32, proclevel2: i32, proclevel: i32,
    worneffect: i32, worntype: i32, wornlevel2: i32, wornlevel: i32,
    focuseffect: i32, focustype: i32, focuslevel2: i32, focuslevel: i32,
    scrolleffect: i32, scrolltype: i32, scrolllevel2: i32, scrolllevel: i32,
}

impl LootCandidateRow {
    fn into_inventory_item(self, loot_slot: i32) -> structs::InventoryItemRow {
        structs::InventoryItemRow {
            slot_id: loot_slot,
            item_id: self.item_db_id,
            charges: self.item_charges as i16,
            itemclass: self.itemclass,
            name: self.name,
            lore: self.lore,
            idfile: self.idfile,
            item_db_id: self.item_db_id,
            weight: self.weight,
            norent: self.norent,
            nodrop: self.nodrop,
            size: self.size,
            slots: self.slots,
            price: self.price,
            icon: self.icon,
            benefitflag: self.benefitflag,
            tradeskills: self.tradeskills,
            cr: self.cr, dr: self.dr, pr: self.pr, mr: self.mr, fr: self.fr,
            astr: self.astr, asta: self.asta, aagi: self.aagi, adex: self.adex,
            acha: self.acha, aint: self.aint, awis: self.awis,
            hp: self.hp, mana: self.mana, ac: self.ac, deity: self.deity,
            skillmodvalue: self.skillmodvalue, skillmodmax: self.skillmodmax,
            skillmodtype: self.skillmodtype,
            banedmgrace: self.banedmgrace, banedmgamt: self.banedmgamt,
            banedmgbody: self.banedmgbody,
            magic: self.magic, casttime_: self.casttime_, reqlevel: self.reqlevel,
            bardtype: self.bardtype, bardvalue: self.bardvalue,
            light: self.light, delay: self.delay, reclevel: self.reclevel,
            recskill: self.recskill,
            elemdmgtype: self.elemdmgtype, elemdmgamt: self.elemdmgamt,
            range: self.range, damage: self.damage,
            color: self.color, classes: self.classes, races: self.races,
            maxcharges: self.maxcharges, itemtype: self.itemtype,
            material: self.material, sellrate: self.sellrate,
            procrate: self.procrate, combateffects: self.combateffects,
            shielding: self.shielding, stunresist: self.stunresist,
            strikethrough: self.strikethrough, extradmgskill: self.extradmgskill,
            extradmgamt: self.extradmgamt,
            spellshield: self.spellshield, avoidance: self.avoidance,
            accuracy: self.accuracy,
            charmfileid: self.charmfileid,
            factionmod1: self.factionmod1, factionmod2: self.factionmod2,
            factionmod3: self.factionmod3, factionmod4: self.factionmod4,
            factionamt1: self.factionamt1, factionamt2: self.factionamt2,
            factionamt3: self.factionamt3, factionamt4: self.factionamt4,
            charmfile: self.charmfile,
            augtype: self.augtype,
            augslot1type: self.augslot1type, augslot1visible: self.augslot1visible,
            augslot2type: self.augslot2type, augslot2visible: self.augslot2visible,
            augslot3type: self.augslot3type, augslot3visible: self.augslot3visible,
            augslot4type: self.augslot4type, augslot4visible: self.augslot4visible,
            augslot5type: self.augslot5type, augslot5visible: self.augslot5visible,
            ldontheme: self.ldontheme, ldonprice: self.ldonprice, ldonsold: self.ldonsold,
            bagtype: self.bagtype, bagslots: self.bagslots, bagsize: self.bagsize,
            bagwr: self.bagwr,
            book: self.book, booktype: self.booktype, filename: self.filename,
            banedmgraceamt: self.banedmgraceamt, augrestrict: self.augrestrict,
            loregroup: self.loregroup,
            pendingloreflag: self.pendingloreflag, artifactflag: self.artifactflag,
            summonedflag: self.summonedflag,
            favor: self.favor, fvnodrop: self.fvnodrop, endur: self.endur,
            dotshielding: self.dotshielding,
            attack: self.attack, regen: self.regen, manaregen: self.manaregen,
            enduranceregen: self.enduranceregen,
            haste: self.haste, damageshield: self.damageshield,
            recastdelay: self.recastdelay, recasttype: self.recasttype,
            guildfavor: self.guildfavor,
            augdistiller: self.augdistiller, attuneable: self.attuneable,
            nopet: self.nopet, pointtype: self.pointtype,
            potionbelt: self.potionbelt, potionbeltslots: self.potionbeltslots,
            stacksize: self.stacksize, notransfer: self.notransfer,
            stackable: self.stackable,
            clickeffect: self.clickeffect, clicktype: self.clicktype,
            clicklevel2: self.clicklevel2, clicklevel: self.clicklevel,
            proceffect: self.proceffect, proctype: self.proctype,
            proclevel2: self.proclevel2, proclevel: self.proclevel,
            worneffect: self.worneffect, worntype: self.worntype,
            wornlevel2: self.wornlevel2, wornlevel: self.wornlevel,
            focuseffect: self.focuseffect, focustype: self.focustype,
            focuslevel2: self.focuslevel2, focuslevel: self.focuslevel,
            scrolleffect: self.scrolleffect, scrolltype: self.scrolltype,
            scrolllevel2: self.scrolllevel2, scrolllevel: self.scrolllevel,
        }
    }
}

async fn resolve_npc_loot(
    pool: &sqlx::PgPool,
    loottable_id: i32,
) -> anyhow::Result<(Vec<structs::InventoryItemRow>, u32, u32, u32, u32)> {
    let mut rng = rand::thread_rng();

    let money = sqlx::query_as::<_, LootMoneyRow>(
        "SELECT mincash, maxcash FROM loottable WHERE id = $1"
    )
    .bind(loottable_id)
    .fetch_optional(pool)
    .await?;

    let (plat, gold, silver, copper) = if let Some(m) = money {
        if m.maxcash > 0 {
            let total = rng.gen_range(m.mincash.max(0)..=m.maxcash) as u32;
            split_money(total)
        } else {
            (0, 0, 0, 0)
        }
    } else {
        (0, 0, 0, 0)
    };

    let candidates = sqlx::query_as::<_, LootCandidateRow>(
        "SELECT \
            lte.lootdrop_id, \
            lte.probability AS group_probability, \
            lte.multiplier AS group_multiplier, \
            lte.droplimit AS group_droplimit, \
            lde.item_charges, \
            lde.chance AS item_chance, \
            it.itemclass, it.name, it.lore, it.idfile, it.id AS item_db_id, \
            it.weight, it.norent, it.nodrop, it.size, it.slots, it.price, it.icon, \
            it.benefitflag, it.tradeskills, \
            it.cr, it.dr, it.pr, it.mr, it.fr, \
            it.astr, it.asta, it.aagi, it.adex, it.acha, it.aint, it.awis, \
            it.hp, it.mana, it.ac, it.deity, \
            it.skillmodvalue, it.skillmodmax, it.skillmodtype, \
            it.banedmgrace, it.banedmgamt, it.banedmgbody, \
            it.magic, it.casttime_, it.reqlevel, it.bardtype, it.bardvalue, \
            it.light, it.delay, it.reclevel, it.recskill, \
            it.elemdmgtype, it.elemdmgamt, it.range, it.damage, \
            it.color, it.classes, it.races, \
            it.maxcharges, it.itemtype, it.material, it.sellrate, \
            it.procrate, it.combateffects, it.shielding, it.stunresist, \
            it.strikethrough, it.extradmgskill, it.extradmgamt, \
            it.spellshield, it.avoidance, it.accuracy, \
            it.charmfileid, \
            it.factionmod1, it.factionmod2, it.factionmod3, it.factionmod4, \
            it.factionamt1, it.factionamt2, it.factionamt3, it.factionamt4, \
            it.charmfile, \
            it.augtype, \
            it.augslot1type, it.augslot1visible, it.augslot2type, it.augslot2visible, \
            it.augslot3type, it.augslot3visible, it.augslot4type, it.augslot4visible, \
            it.augslot5type, it.augslot5visible, \
            it.ldontheme, it.ldonprice, it.ldonsold, \
            it.bagtype, it.bagslots, it.bagsize, it.bagwr, \
            it.book, it.booktype, it.filename, \
            it.banedmgraceamt, it.augrestrict, it.loregroup, it.pendingloreflag, \
            it.artifactflag, it.summonedflag, \
            it.favor, it.fvnodrop, it.endur, it.dotshielding, \
            it.attack, it.regen, it.manaregen, it.enduranceregen, \
            it.haste, it.damageshield, it.recastdelay, it.recasttype, it.guildfavor, \
            it.augdistiller, it.attuneable, it.nopet, it.pointtype, \
            it.potionbelt, it.potionbeltslots, it.stacksize, it.notransfer, it.stackable, \
            it.clickeffect, it.clicktype, it.clicklevel2, it.clicklevel, \
            it.proceffect, it.proctype, it.proclevel2, it.proclevel, \
            it.worneffect, it.worntype, it.wornlevel2, it.wornlevel, \
            it.focuseffect, it.focustype, it.focuslevel2, it.focuslevel, \
            it.scrolleffect, it.scrolltype, it.scrolllevel2, it.scrolllevel \
        FROM loottable_entries lte \
        JOIN lootdrop_entries lde ON lte.lootdrop_id = lde.lootdrop_id \
        JOIN items it ON lde.item_id = it.id \
        WHERE lte.loottable_id = $1 \
        ORDER BY lte.lootdrop_id, lde.chance DESC"
    )
    .bind(loottable_id)
    .fetch_all(pool)
    .await?;

    let mut groups: HashMap<i32, Vec<LootCandidateRow>> = HashMap::new();
    for c in candidates {
        groups.entry(c.lootdrop_id).or_default().push(c);
    }

    let mut won_items: Vec<structs::InventoryItemRow> = Vec::new();
    let mut slot = 0i32;

    for (_, entries) in &groups {
        let prob = entries[0].group_probability;
        let multiplier = entries[0].group_multiplier.max(1) as usize;
        let droplimit = entries[0].group_droplimit;

        if prob < 100.0 && rng.gen_range(0.0f32..100.0) >= prob {
            continue;
        }

        for _ in 0..multiplier {
            let mut picks = 0i32;
            for entry in entries.iter() {
                if droplimit > 0 && picks >= droplimit { break; }
                if rng.gen_range(0.0f32..100.0) < entry.item_chance {
                    won_items.push(structs::InventoryItemRow {
                        slot_id: slot,
                        item_id: entry.item_db_id,
                        charges: entry.item_charges as i16,
                        itemclass: entry.itemclass,
                        name: entry.name.clone(),
                        lore: entry.lore.clone(),
                        idfile: entry.idfile.clone(),
                        item_db_id: entry.item_db_id,
                        weight: entry.weight,
                        norent: entry.norent,
                        nodrop: entry.nodrop,
                        size: entry.size,
                        slots: entry.slots,
                        price: entry.price,
                        icon: entry.icon,
                        benefitflag: entry.benefitflag,
                        tradeskills: entry.tradeskills,
                        cr: entry.cr, dr: entry.dr, pr: entry.pr, mr: entry.mr, fr: entry.fr,
                        astr: entry.astr, asta: entry.asta, aagi: entry.aagi, adex: entry.adex,
                        acha: entry.acha, aint: entry.aint, awis: entry.awis,
                        hp: entry.hp, mana: entry.mana, ac: entry.ac, deity: entry.deity,
                        skillmodvalue: entry.skillmodvalue, skillmodmax: entry.skillmodmax,
                        skillmodtype: entry.skillmodtype,
                        banedmgrace: entry.banedmgrace, banedmgamt: entry.banedmgamt,
                        banedmgbody: entry.banedmgbody,
                        magic: entry.magic, casttime_: entry.casttime_, reqlevel: entry.reqlevel,
                        bardtype: entry.bardtype, bardvalue: entry.bardvalue,
                        light: entry.light, delay: entry.delay, reclevel: entry.reclevel,
                        recskill: entry.recskill,
                        elemdmgtype: entry.elemdmgtype, elemdmgamt: entry.elemdmgamt,
                        range: entry.range, damage: entry.damage,
                        color: entry.color, classes: entry.classes, races: entry.races,
                        maxcharges: entry.maxcharges, itemtype: entry.itemtype,
                        material: entry.material, sellrate: entry.sellrate,
                        procrate: entry.procrate, combateffects: entry.combateffects.clone(),
                        shielding: entry.shielding, stunresist: entry.stunresist,
                        strikethrough: entry.strikethrough, extradmgskill: entry.extradmgskill,
                        extradmgamt: entry.extradmgamt,
                        spellshield: entry.spellshield, avoidance: entry.avoidance,
                        accuracy: entry.accuracy,
                        charmfileid: entry.charmfileid.clone(),
                        factionmod1: entry.factionmod1, factionmod2: entry.factionmod2,
                        factionmod3: entry.factionmod3, factionmod4: entry.factionmod4,
                        factionamt1: entry.factionamt1, factionamt2: entry.factionamt2,
                        factionamt3: entry.factionamt3, factionamt4: entry.factionamt4,
                        charmfile: entry.charmfile.clone(),
                        augtype: entry.augtype,
                        augslot1type: entry.augslot1type, augslot1visible: entry.augslot1visible,
                        augslot2type: entry.augslot2type, augslot2visible: entry.augslot2visible,
                        augslot3type: entry.augslot3type, augslot3visible: entry.augslot3visible,
                        augslot4type: entry.augslot4type, augslot4visible: entry.augslot4visible,
                        augslot5type: entry.augslot5type, augslot5visible: entry.augslot5visible,
                        ldontheme: entry.ldontheme, ldonprice: entry.ldonprice, ldonsold: entry.ldonsold,
                        bagtype: entry.bagtype, bagslots: entry.bagslots, bagsize: entry.bagsize,
                        bagwr: entry.bagwr,
                        book: entry.book, booktype: entry.booktype, filename: entry.filename.clone(),
                        banedmgraceamt: entry.banedmgraceamt, augrestrict: entry.augrestrict,
                        loregroup: entry.loregroup,
                        pendingloreflag: entry.pendingloreflag, artifactflag: entry.artifactflag,
                        summonedflag: entry.summonedflag,
                        favor: entry.favor, fvnodrop: entry.fvnodrop, endur: entry.endur,
                        dotshielding: entry.dotshielding,
                        attack: entry.attack, regen: entry.regen, manaregen: entry.manaregen,
                        enduranceregen: entry.enduranceregen,
                        haste: entry.haste, damageshield: entry.damageshield,
                        recastdelay: entry.recastdelay, recasttype: entry.recasttype,
                        guildfavor: entry.guildfavor,
                        augdistiller: entry.augdistiller, attuneable: entry.attuneable,
                        nopet: entry.nopet, pointtype: entry.pointtype,
                        potionbelt: entry.potionbelt, potionbeltslots: entry.potionbeltslots,
                        stacksize: entry.stacksize, notransfer: entry.notransfer,
                        stackable: entry.stackable,
                        clickeffect: entry.clickeffect, clicktype: entry.clicktype,
                        clicklevel2: entry.clicklevel2, clicklevel: entry.clicklevel,
                        proceffect: entry.proceffect, proctype: entry.proctype,
                        proclevel2: entry.proclevel2, proclevel: entry.proclevel,
                        worneffect: entry.worneffect, worntype: entry.worntype,
                        wornlevel2: entry.wornlevel2, wornlevel: entry.wornlevel,
                        focuseffect: entry.focuseffect, focustype: entry.focustype,
                        focuslevel2: entry.focuslevel2, focuslevel: entry.focuslevel,
                        scrolleffect: entry.scrolleffect, scrolltype: entry.scrolltype,
                        scrolllevel2: entry.scrolllevel2, scrolllevel: entry.scrolllevel,
                    });
                    slot += 1;
                    picks += 1;
                }
            }
        }
    }

    Ok((won_items, plat, gold, silver, copper))
}

async fn handle_udp_packet(
    raw: &[u8],
    addr: SocketAddr,
    phase: ConnectionPhase,
    state: &mut PhaseState,
    socket: &UdpSocket,
    world_state: &Arc<WorldState>,
) -> anyhow::Result<()> {
    let len = raw.len();

    debug!(addr = %addr, len, phase = ?phase, hex = %hex_preview(raw), "UDP recv");

    if len < 2 {
        return Ok(());
    }

    if raw[0] == 0x00 && raw[1] == eq_protocol::OP_SESSION_REQUEST {
        match packet::parse_protocol_packet(raw) {
            Ok(ProtocolPacket::SessionRequest {
                protocol_version,
                connect_code,
                max_packet_size,
            }) => {
                info!(
                    addr = %addr,
                    phase = ?phase,
                    version = protocol_version,
                    "New {:?} connection", phase
                );

                let crc_bytes = if phase == ConnectionPhase::Login { 0 } else { 2 };
                let (encode_key, compress) = match phase {
                    ConnectionPhase::Zone => (0xFFFFFFFF_u32, true),
                    _ => (0_u32, false),
                };
                let session = EqSession::new(addr, connect_code, max_packet_size, crc_bytes, encode_key, compress);
                let response = packet::build_session_response(
                    connect_code,
                    session.encode_key,
                    session.crc_bytes,
                    session.max_packet_size,
                    if compress { 1 } else { 0 },
                );

                socket.send_to(&response, addr).await?;

                state.sessions.insert(addr, session);

                state.client_states.insert(addr, ClientState::new(phase));
            }
            _ => {}
        }
        return Ok(());
    }

    let session = match state.sessions.get_mut(&addr) {
        Some(s) => s,
        None => return Ok(()),
    };

    let mut raw_owned = raw.to_vec();

    // Non-zero first byte = raw app packet (no protocol framing)
    if raw[0] != 0x00 {
        // CRC strip
        if session.crc_bytes > 0 {
            if !eq_protocol::codec::verify_and_strip_crc(&mut raw_owned, session.encode_key, session.crc_bytes) {
                return Ok(());
            }
        }
        // Decompress starting at byte 1 (byte 0 is part of app data)
        if session.compress && raw_owned.len() > 1 {
            match eq_protocol::codec::decompress(&raw_owned[1..]) {
                Ok(decompressed) => {
                    let first = raw_owned[0];
                    raw_owned.clear();
                    raw_owned.push(first);
                    raw_owned.extend_from_slice(&decompressed);
                }
                Err(_) => return Ok(()),
            }
        }
        // Treat as raw app packet: first 2 bytes = opcode (LE)
        if raw_owned.len() >= 2 {
            let app_opcode = u16::from_le_bytes([raw_owned[0], raw_owned[1]]);
            let app_data = &raw_owned[2..];
            dispatch_app_packet(session, &mut state.client_states, addr, socket, phase, app_opcode, app_data, world_state).await?;
        }
        return Ok(());
    }

    // CRC decode: only SessionRequest/SessionResponse/OutOfSession are exempt (per EQEmu PacketCanBeEncoded)
    let proto_op = raw[1];
    match proto_op {
        eq_protocol::OP_SESSION_REQUEST | eq_protocol::OP_SESSION_RESPONSE | eq_protocol::OP_OUT_OF_SESSION => {}
        _ => {
            if !session.decode_packet(&mut raw_owned) {
                return Ok(());
            }
        }
    }

    match packet::parse_protocol_packet(&raw_owned) {
        Ok(ProtocolPacket::KeepAlive) => {
            send_proto_packet(session, socket, addr, &packet::build_keep_alive()).await?;
        }

        Ok(ProtocolPacket::SessionStatRequest { .. }) => {}

        Ok(ProtocolPacket::SessionDisconnect { .. }) => {
            if let Some(cs) = state.client_states.get(&addr) {
                if !cs.char_name.is_empty() && phase == ConnectionPhase::Zone && !cs.zone_transition_pending {
                    info!(character = %cs.char_name, x = cs.last_x, y = cs.last_y, z = cs.last_z, "Saving position on disconnect");
                    let _ = save_character_position(
                        &world_state.pool, &cs.char_name,
                        cs.last_x, cs.last_y, cs.last_z, cs.last_heading
                    ).await;
                }
            }
            info!(addr = %addr, phase = ?phase, "Client disconnected");
            state.sessions.remove(&addr);
            state.client_states.remove(&addr);
        }

        Ok(ProtocolPacket::Ack { .. }) | Ok(ProtocolPacket::OutOfOrderAck { .. }) => {}

        Ok(ProtocolPacket::AppPacket { sequence, data }) => {
            session.process_incoming_sequence(sequence);
            send_proto_packet(session, socket, addr, &packet::build_ack(sequence)).await?;

            if data.len() >= 2 {
                let app_opcode = u16::from_le_bytes([data[0], data[1]]);
                let app_data = &data[2..];
                dispatch_app_packet(session, &mut state.client_states, addr, socket, phase, app_opcode, app_data, world_state).await?;
            }
        }

        Ok(ProtocolPacket::Fragment { sequence, data }) => {
            session.process_incoming_sequence(sequence);
            send_proto_packet(session, socket, addr, &packet::build_ack(sequence)).await?;

            let is_first = session.fragment_assembler.pending_count() == 0;
            if let Some(complete) = session.fragment_assembler.add_fragment(sequence, &data, is_first) {
                if complete.len() >= 2 {
                    let app_opcode = u16::from_le_bytes([complete[0], complete[1]]);
                    let app_data = &complete[2..];
                    dispatch_app_packet(session, &mut state.client_states, addr, socket, phase, app_opcode, app_data, world_state).await?;
                }
            }
        }

        Ok(ProtocolPacket::Combined { sub_packets }) => {
            for sub in sub_packets {
                if sub.len() >= 2 {
                    let full = if sub[0] == 0x00 {
                        sub.clone()
                    } else {
                        [&[0x00], sub.as_slice()].concat()
                    };
                    match packet::parse_protocol_packet(&full) {
                        Ok(ProtocolPacket::AppPacket { sequence, data }) => {
                            session.process_incoming_sequence(sequence);
                            send_proto_packet(session, socket, addr, &packet::build_ack(sequence)).await?;
                            if data.len() >= 2 {
                                let app_opcode = u16::from_le_bytes([data[0], data[1]]);
                                let app_data = &data[2..];
                                dispatch_app_packet(session, &mut state.client_states, addr, socket, phase, app_opcode, app_data, world_state).await?;
                            }
                        }
                        Ok(ProtocolPacket::Fragment { sequence, data }) => {
                            session.process_incoming_sequence(sequence);
                            send_proto_packet(session, socket, addr, &packet::build_ack(sequence)).await?;
                            let is_first = session.fragment_assembler.pending_count() == 0;
                            if let Some(complete) = session.fragment_assembler.add_fragment(sequence, &data, is_first) {
                                if complete.len() >= 2 {
                                    let app_opcode = u16::from_le_bytes([complete[0], complete[1]]);
                                    let app_data = &complete[2..];
                                    dispatch_app_packet(session, &mut state.client_states, addr, socket, phase, app_opcode, app_data, world_state).await?;
                                }
                            }
                        }
                        Ok(ProtocolPacket::Ack { .. }) | Ok(ProtocolPacket::OutOfOrderAck { .. }) => {}
                        _ => {}
                    }
                }
            }
        }

        Ok(ProtocolPacket::OutboundPing) => {}
        Ok(ProtocolPacket::Unknown { opcode, .. }) => {
            debug!(opcode = format!("0x{opcode:02X}"), "Unknown protocol opcode");
        }
        Err(e) => {
            debug!(error = %e, "Parse error");
        }
        _ => {}
    }

    Ok(())
}

async fn dispatch_app_packet(
    session: &mut EqSession,
    client_states: &mut HashMap<SocketAddr, ClientState>,
    addr: SocketAddr,
    socket: &UdpSocket,
    phase: ConnectionPhase,
    opcode: u16,
    data: &[u8],
    world_state: &Arc<WorldState>,
) -> anyhow::Result<()> {
    if opcode == opcodes::OP_APP_COMBINED {
        let mut offset = 0;
        while offset < data.len() {
            let sub_len = data[offset] as usize;
            offset += 1;
            if offset + sub_len > data.len() || sub_len < 2 {
                break;
            }
            let sub_opcode = u16::from_le_bytes([data[offset], data[offset + 1]]);
            let sub_data = &data[offset + 2..offset + sub_len];
            Box::pin(dispatch_app_packet(
                session, client_states, addr, socket, phase, sub_opcode, sub_data, world_state,
            ))
            .await?;
            offset += sub_len;
        }
        return Ok(());
    }

    match phase {
        ConnectionPhase::Login => {
            login_handler::handle_login_opcode(session, socket, addr, opcode, data).await
        }
        ConnectionPhase::World => {
            let cs = client_states.get_mut(&addr).unwrap();
            world_handler::handle_world_opcode(session, cs, socket, addr, opcode, data, world_state).await
        }
        ConnectionPhase::Zone => {
            let cs = client_states.get_mut(&addr).unwrap();
            handle_zone_packet(session, cs, socket, addr, opcode, data, world_state).await
        }
    }
}

async fn send_proto_packet(
    session: &EqSession,
    socket: &UdpSocket,
    addr: SocketAddr,
    data: &[u8],
) -> anyhow::Result<()> {
    let mut buf = if session.compress && data.len() > 2 {
        let mut b = Vec::from(&data[..2]);
        b.extend_from_slice(&eq_protocol::codec::compress(&data[2..]));
        b
    } else {
        data.to_vec()
    };
    eq_protocol::codec::append_crc(&mut buf, session.encode_key, session.crc_bytes);
    socket.send_to(&buf, addr).await?;
    Ok(())
}

pub async fn send_app_packet(
    session: &mut EqSession,
    socket: &UdpSocket,
    addr: SocketAddr,
    app_opcode: u16,
    app_data: &[u8],
) -> anyhow::Result<()> {
    let mut app_payload = Vec::with_capacity(2 + app_data.len());
    app_payload.extend_from_slice(&app_opcode.to_le_bytes());
    app_payload.extend_from_slice(app_data);

    let proto_header = 2usize; // [0x00][opcode]
    let seq_size = 2usize;
    let compress_flag = if session.compress { 1usize } else { 0 };
    let crc_size = session.crc_bytes as usize;
    let max_pkt = session.max_packet_size as usize;
    let single_size = proto_header + compress_flag + seq_size + app_payload.len() + crc_size;

    if single_size <= max_pkt {
        let pkt = session.build_app_packet(app_opcode, app_data);
        debug!(opcode = format!("0x{app_opcode:04X}"), app_bytes = app_data.len(), udp_bytes = pkt.len(), "TX");
        socket.send_to(&pkt, addr).await?;
    } else {
        let total_size = app_payload.len() as u32;
        let per_frag_overhead = proto_header + compress_flag + seq_size + crc_size;
        let first_data_cap = max_pkt - per_frag_overhead - 4; // 4 for total_size
        let subsequent_data_cap = max_pkt - per_frag_overhead;

        let mut offset = 0;
        let mut frag_count = 0u32;
        while offset < app_payload.len() {
            let (cap, is_first) = if offset == 0 {
                (first_data_cap, true)
            } else {
                (subsequent_data_cap, false)
            };
            let end = (offset + cap).min(app_payload.len());
            let chunk = &app_payload[offset..end];

            let seq = session.next_sequence_out();
            let mut frag_data = Vec::new();
            frag_data.extend_from_slice(&seq.to_be_bytes());
            if is_first {
                frag_data.extend_from_slice(&total_size.to_be_bytes());
            }
            frag_data.extend_from_slice(chunk);

            let encoded = if session.compress {
                eq_protocol::codec::compress(&frag_data)
            } else {
                frag_data
            };

            let mut buf = Vec::with_capacity(max_pkt);
            buf.push(0x00);
            buf.push(eq_protocol::OP_FRAGMENT);
            buf.extend_from_slice(&encoded);
            eq_protocol::codec::append_crc(&mut buf, session.encode_key, session.crc_bytes);

            socket.send_to(&buf, addr).await?;
            offset = end;
            frag_count += 1;
        }
        debug!(
            opcode = format!("0x{app_opcode:04X}"),
            total_bytes = total_size,
            fragments = frag_count,
            "TX fragmented"
        );
    }
    Ok(())
}

async fn handle_zone_packet(
    session: &mut EqSession,
    cs: &mut ClientState,
    socket: &UdpSocket,
    addr: SocketAddr,
    opcode: u16,
    data: &[u8],
    world_state: &Arc<WorldState>,
) -> anyhow::Result<()> {
    match opcode {
        opcodes::OP_ZONE_ENTRY => {
            cs.char_name = structs::extract_zone_entry_name(data);
            cs.player_spawn_id = cs.alloc_spawn_id();

            info!(character = %cs.char_name, spawn_id = cs.player_spawn_id, "Zone: entry request");

            let record = adif_world::character::load_character_by_name(
                &world_state.pool,
                &cs.char_name,
            ).await?;

            if let Some(ref r) = record {
                cs.char_zone_id = Some(r.zone_id);
                if let Some(zone_row) = sqlx::query_as::<_, (String,)>(
                    "SELECT short_name FROM zone WHERE zoneidnumber = $1"
                )
                .bind(r.zone_id)
                .fetch_optional(&world_state.pool)
                .await? {
                    cs.char_zone_short = zone_row.0;
                }
            }

            let ppd = if let Some(ref r) = record {
                PlayerProfileData {
                    name: r.name.clone(), last_name: r.last_name.clone(),
                    race: r.race as u32, class_id: r.class_id as u32,
                    level: r.level as u8, gender: r.gender as u32,
                    deity: r.deity as u32,
                    x: r.x, y: r.y, z: r.z, heading: r.heading,
                    zone_id: r.zone_id as u16,
                    face: r.face as u8, hair_color: r.hair_color as u8,
                    beard_color: r.beard_color as u8,
                    eye_color_1: r.eye_color_1 as u8, eye_color_2: r.eye_color_2 as u8,
                    hair_style: r.hair_style as u8, beard: r.beard as u8,
                    entity_id: cs.player_spawn_id,
                }
            } else {
                warn!(character = %cs.char_name, "Zone: character not found in DB, using defaults");
                PlayerProfileData {
                    name: cs.char_name.clone(), last_name: String::new(),
                    race: 9, class_id: 1, level: 1, gender: 0, deity: 0,
                    x: -99.0, y: -585.0, z: 27.0, heading: 0.0,
                    zone_id: 52, face: 0, hair_color: 0, beard_color: 0,
                    eye_color_1: 0, eye_color_2: 0, hair_style: 0, beard: 0,
                    entity_id: cs.player_spawn_id,
                }
            };

            let race = ppd.race;
            let class_id = ppd.class_id;
            let level = ppd.level;
            cs.player_level = level as u8;
            let gender = ppd.gender;
            let deity = ppd.deity as u16;
            let x = ppd.x;
            let y = ppd.y;
            let z = ppd.z;
            let heading = ppd.heading;
            let last_name = ppd.last_name.clone();

            let mut pp = structs::build_player_profile_full(&ppd);

            // Load skills from DB and write into PP at offset 4460 (100 x u32)
            if let Some(ref r) = record {
                let skills: Vec<(i16, i16)> = sqlx::query_as(
                    "SELECT skill_id, value FROM character_skills WHERE id = $1"
                )
                .bind(r.id)
                .fetch_all(&world_state.pool)
                .await?;
                for (skill_id, value) in &skills {
                    let idx = *skill_id as usize;
                    if idx < 100 {
                        let off = 4460 + idx * 4;
                        pp[off..off + 4].copy_from_slice(&(*value as u32).to_le_bytes());
                    }
                }
                structs::recompute_pp_checksum(&mut pp);
                info!(count = skills.len(), "Zone: loaded skills from DB into PlayerProfile");
            }
            info!(
                "PP dump: checksum={:08X} gender={} race={} class={} level={} zone_id_at_13276={} name_at_12940={}",
                u32::from_le_bytes([pp[0], pp[1], pp[2], pp[3]]),
                u32::from_le_bytes([pp[4], pp[5], pp[6], pp[7]]),
                u32::from_le_bytes([pp[8], pp[9], pp[10], pp[11]]),
                u32::from_le_bytes([pp[12], pp[13], pp[14], pp[15]]),
                pp[20],
                u16::from_le_bytes([pp[13276], pp[13277]]),
                String::from_utf8_lossy(&pp[12940..12940+20]).trim_end_matches('\0'),
            );
            info!(
                "PP bytes[0..40]: {}",
                pp[..40].iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(" ")
            );
            send_app_packet(session, socket, addr, opcodes::OP_PLAYER_PROFILE, &pp).await?;
            info!(race, class_id, level, "Zone: sent PlayerProfile from DB");

            let size = match race {
                1 => 6.0, 2 => 6.0, 3 => 8.0, 4 => 5.0, 5 => 4.0,
                6 => 5.0, 7 => 5.0, 8 => 7.0, 9 => 8.0, 10 => 7.0,
                11 => 6.0, 12 => 6.0, 128 => 5.0, 130 => 5.0, _ => 6.0,
            };

            let player_spawn = structs::build_spawn_struct(&SpawnData {
                spawn_id: cs.player_spawn_id,
                name: cs.char_name.clone(), last_name,
                level, race, class_id: class_id as u8, gender: gender as u8, deity,
                x, y, z: z + 6.0, heading, size,
                npc_type: 0, cur_hp: 1, max_hp: 100, body_type: 0,
                run_speed: 0.7, walk_speed: 0.46,
                findable: 1, light: 0, texture: 0, helm_texture: 0,
                guild_id: 0xFFFFFFFF,
            });
            send_app_packet(session, socket, addr, opcodes::OP_ZONE_ENTRY, &player_spawn).await?;

            cs.spawned_npcs.clear();
            cs.target_id = None;
            cs.auto_attack = false;
            cs.last_attack_time = None;
            cs.spawned_npcs.insert(cs.player_spawn_id, SpawnedNpcInfo {
                level: level as u8,
                name: cs.char_name.clone(),
                cur_hp: 1000,
                max_hp: 1000,
                min_dmg: 0,
                max_dmg: 0,
                attack_delay: 30,
                loottable_id: 0,
                is_corpse: false,
                loot_items: Vec::new(),
                loot_platinum: 0, loot_gold: 0, loot_silver: 0, loot_copper: 0,
            });

            let zone_short = if cs.char_zone_short.is_empty() { "innothule".to_string() } else { cs.char_zone_short.clone() };
            let spawns = sqlx::query_as::<_, ZoneSpawnRow>(
                "SELECT n.name AS npc_name, n.lastname, n.level, n.race, n.class, \
                 n.gender, n.bodytype, n.hp, n.size, n.runspeed, n.walkspeed, \
                 n.texture, n.helmtexture, n.light, n.findable, n.flymode, \
                 n.mindmg, n.maxdmg, n.attack_delay, n.loottable_id, \
                 s.x, s.y, s.z, s.heading \
                 FROM spawn2 s \
                 JOIN spawnentry se ON s.spawngroupid = se.spawngroupid \
                 JOIN npc_types n ON se.npcid = n.id \
                 WHERE s.zone = $1 AND (s.version = 0 OR s.version = -1)"
            )
            .bind(&zone_short)
            .fetch_all(&world_state.pool)
            .await?;

            let mut bulk_spawns = Vec::new();
            for row in &spawns {
                let npc_id = cs.alloc_spawn_id();
                let hp = if row.hp > 0 { row.hp } else { (row.level as i64) * 20 };
                cs.spawned_npcs.insert(npc_id, SpawnedNpcInfo {
                    level: row.level as u8,
                    name: row.npc_name.replace('#', ""),
                    cur_hp: hp,
                    max_hp: hp,
                    min_dmg: row.mindmg,
                    max_dmg: row.maxdmg,
                    attack_delay: row.attack_delay,
                    loottable_id: row.loottable_id,
                    is_corpse: false,
                    loot_items: Vec::new(),
                    loot_platinum: 0, loot_gold: 0, loot_silver: 0, loot_copper: 0,
                });
                let spawn = structs::build_spawn_struct(&SpawnData {
                    spawn_id: npc_id,
                    name: row.npc_name.replace('#', ""),
                    last_name: row.lastname.clone().unwrap_or_default(),
                    level: row.level as u8,
                    race: row.race as u32,
                    class_id: row.class as u8,
                    gender: row.gender as u8,
                    deity: 0,
                    x: row.x, y: row.y, z: row.z, heading: row.heading,
                    size: if row.size > 0.0 { row.size } else { 6.0 },
                    npc_type: 1,
                    cur_hp: 100,
                    max_hp: 100,
                    body_type: row.bodytype as u8,
                    run_speed: if row.runspeed > 0.0 { row.runspeed } else { 0.7 },
                    walk_speed: if row.walkspeed > 0.0 { row.walkspeed } else { 0.46 },
                    findable: row.findable as u8,
                    light: row.light as u8,
                    texture: row.texture as u8,
                    helm_texture: row.helmtexture as u8,
                    guild_id: 0,
                });
                bulk_spawns.extend_from_slice(&spawn);
            }
            if !bulk_spawns.is_empty() {
                send_app_packet(session, socket, addr, opcodes::OP_ZONE_SPAWNS, &bulk_spawns).await?;
            }
            info!(count = spawns.len(), zone = %zone_short, "Zone: sent bulk NPC spawns via OP_ZoneSpawns");

            // Corpses (PC corpses = npc_type 2)
            let zone_id = cs.char_zone_id.unwrap_or(46);
            let corpses: Vec<(String, f32, f32, f32, f32, i32, i32, i32, i32)> = sqlx::query_as(
                "SELECT charname, x, y, z, heading, race, class, gender, level \
                 FROM character_corpses WHERE zone_id = $1 AND is_buried = 0"
            )
            .bind(zone_id)
            .fetch_all(&world_state.pool)
            .await?;
            if !corpses.is_empty() {
                let mut corpse_buf = Vec::new();
                for (name, cx, cy, cz, cheading, crace, cclass, cgender, clevel) in &corpses {
                    let corpse_id = cs.alloc_spawn_id();
                    let corpse_name = format!("{}'s corpse", name);
                    let spawn = structs::build_spawn_struct(&SpawnData {
                        spawn_id: corpse_id,
                        name: corpse_name, last_name: String::new(),
                        level: *clevel as u8, race: *crace as u32,
                        class_id: *cclass as u8, gender: *cgender as u8, deity: 0,
                        x: *cx, y: *cy, z: *cz, heading: *cheading,
                        size: 6.0, npc_type: 2, cur_hp: 0, max_hp: 0, body_type: 0,
                        run_speed: 0.0, walk_speed: 0.0,
                        findable: 0, light: 0, texture: 0, helm_texture: 0, guild_id: 0xFFFFFFFF,
                    });
                    corpse_buf.extend_from_slice(&spawn);
                }
                send_app_packet(session, socket, addr, opcodes::OP_ZONE_SPAWNS, &corpse_buf).await?;
                info!(count = corpses.len(), "Zone: sent corpses via OP_ZoneSpawns");
            }

            if let Some(ref r) = record {
                let inv_items = sqlx::query_as::<_, InventoryItemRow>(INVENTORY_QUERY)
                    .bind(r.id)
                    .fetch_all(&world_state.pool)
                    .await?;
                if inv_items.is_empty() {
                    send_app_packet(session, socket, addr, opcodes::OP_CHAR_INVENTORY, &0u32.to_le_bytes()).await?;
                } else {
                    let mut inv_buf = Vec::new();
                    for (i, item) in inv_items.iter().enumerate() {
                        inv_buf.extend_from_slice(&structs::serialize_titanium_item(item, (i + 1) as i32));
                    }
                    send_app_packet(session, socket, addr, opcodes::OP_CHAR_INVENTORY, &inv_buf).await?;
                    info!(count = inv_items.len(), bytes = inv_buf.len(), "Zone: sent inventory items");
                }
            } else {
                send_app_packet(session, socket, addr, opcodes::OP_CHAR_INVENTORY, &0u32.to_le_bytes()).await?;
            }
            send_app_packet(session, socket, addr, opcodes::OP_TIME_OF_DAY, &structs::build_time_of_day(14, 0, 1, 3100)).await?;
            send_app_packet(session, socket, addr, opcodes::OP_WEATHER, &structs::build_weather(0, 0)).await?;
        }

        opcodes::OP_REQ_NEW_ZONE => {
            let zone_id = cs.char_zone_id.unwrap_or(46);
            let zr = sqlx::query_as::<_, ZoneDbRow>(
                "SELECT short_name, long_name, safe_x, safe_y, safe_z, \
                 minclip, maxclip, fog_minclip, fog_maxclip, \
                 fog_minclip2, fog_maxclip2, fog_minclip3, fog_maxclip3, \
                 fog_minclip4, fog_maxclip4, \
                 fog_red, fog_green, fog_blue, \
                 fog_red2, fog_green2, fog_blue2, \
                 fog_red3, fog_green3, fog_blue3, \
                 fog_red4, fog_green4, fog_blue4, \
                 fog_density, sky, ztype, zone_exp_multiplier::float4 AS zone_exp_multiplier, gravity, time_type, \
                 rain_chance1, rain_chance2, rain_chance3, rain_chance4, \
                 rain_duration1, rain_duration2, rain_duration3, rain_duration4, \
                 snow_chance1, snow_chance2, snow_chance3, snow_chance4, \
                 snow_duration1, snow_duration2, snow_duration3, snow_duration4, \
                 underworld, max_z \
                 FROM zone WHERE zoneidnumber = $1"
            )
            .bind(zone_id)
            .fetch_one(&world_state.pool)
            .await?;

            let zd = ZoneData {
                short_name: zr.short_name, long_name: zr.long_name,
                zone_id: zone_id as u16,
                safe_x: zr.safe_x, safe_y: zr.safe_y, safe_z: zr.safe_z,
                minclip: zr.minclip, maxclip: zr.maxclip,
                fog_minclip: [zr.fog_minclip, zr.fog_minclip2, zr.fog_minclip3, zr.fog_minclip4],
                fog_maxclip: [zr.fog_maxclip, zr.fog_maxclip2, zr.fog_maxclip3, zr.fog_maxclip4],
                fog_red: [zr.fog_red as u8, zr.fog_red2 as u8, zr.fog_red3 as u8, zr.fog_red4 as u8],
                fog_green: [zr.fog_green as u8, zr.fog_green2 as u8, zr.fog_green3 as u8, zr.fog_green4 as u8],
                fog_blue: [zr.fog_blue as u8, zr.fog_blue2 as u8, zr.fog_blue3 as u8, zr.fog_blue4 as u8],
                fog_density: zr.fog_density,
                sky: zr.sky as u8, ztype: zr.ztype as u8,
                zone_exp_multiplier: zr.zone_exp_multiplier,
                gravity: zr.gravity, time_type: zr.time_type as u8,
                rain_chance: [zr.rain_chance1 as u8, zr.rain_chance2 as u8, zr.rain_chance3 as u8, zr.rain_chance4 as u8],
                rain_duration: [zr.rain_duration1 as u8, zr.rain_duration2 as u8, zr.rain_duration3 as u8, zr.rain_duration4 as u8],
                snow_chance: [zr.snow_chance1 as u8, zr.snow_chance2 as u8, zr.snow_chance3 as u8, zr.snow_chance4 as u8],
                snow_duration: [zr.snow_duration1 as u8, zr.snow_duration2 as u8, zr.snow_duration3 as u8, zr.snow_duration4 as u8],
                underworld: zr.underworld, max_z: zr.max_z as f32,
            };
            info!(zone = %zd.short_name, "Zone: sending zone config from DB");
            let nz = structs::build_new_zone_struct(&cs.char_name, &zd);
            send_app_packet(session, socket, addr, opcodes::OP_NEW_ZONE, &nz).await?;
        }

        opcodes::OP_REQ_CLIENT_SPAWN => {
            info!("Zone: sending zone contents and ready signals");
            let zone_short = if cs.char_zone_short.is_empty() { "innothule".to_string() } else { cs.char_zone_short.clone() };

            let door_rows = sqlx::query_as::<_, DoorRow>(
                "SELECT name, pos_y, pos_x, pos_z, heading, incline, size, \
                 doorid, opentype, invert_state, door_param \
                 FROM doors WHERE zone = $1"
            )
            .bind(&zone_short)
            .fetch_all(&world_state.pool)
            .await?;

            if door_rows.is_empty() {
                send_app_packet(session, socket, addr, opcodes::OP_SPAWN_DOOR, &0u32.to_le_bytes()).await?;
            } else {
                let door_struct_size = 80usize;
                let mut door_buf = vec![0u8; door_rows.len() * door_struct_size];
                for (i, dr) in door_rows.iter().enumerate() {
                    let off = i * door_struct_size;
                    let name_bytes = dr.name.as_bytes();
                    let name_len = name_bytes.len().min(31);
                    door_buf[off..off + name_len].copy_from_slice(&name_bytes[..name_len]);
                    door_buf[off + 32..off + 36].copy_from_slice(&dr.pos_y.to_le_bytes());
                    door_buf[off + 36..off + 40].copy_from_slice(&dr.pos_x.to_le_bytes());
                    door_buf[off + 40..off + 44].copy_from_slice(&dr.pos_z.to_le_bytes());
                    door_buf[off + 44..off + 48].copy_from_slice(&dr.heading.to_le_bytes());
                    door_buf[off + 48..off + 52].copy_from_slice(&(dr.incline as u32).to_le_bytes());
                    door_buf[off + 52..off + 54].copy_from_slice(&(dr.size as u16).to_le_bytes());
                    door_buf[off + 60] = dr.doorid as u8;
                    door_buf[off + 61] = dr.opentype as u8;
                    door_buf[off + 63] = dr.invert_state as u8;
                    door_buf[off + 64..off + 68].copy_from_slice(&(dr.door_param as u32).to_le_bytes());
                    door_buf[off + 77] = 0x01;
                    door_buf[off + 79] = 0x01;
                }
                send_app_packet(session, socket, addr, opcodes::OP_SPAWN_DOOR, &door_buf).await?;
                info!(count = door_rows.len(), zone = %zone_short, "Zone: sent doors from DB");
            }

            // Ground objects (OP_GroundSpawn per object)
            let zone_id = cs.char_zone_id.unwrap_or(46);
            let obj_rows = sqlx::query_as::<_, ObjectRow>(
                "SELECT id, xpos, ypos, zpos, heading, objectname, type, size, incline, tilt_x, tilt_y \
                 FROM object WHERE zoneid = $1"
            )
            .bind(zone_id)
            .fetch_all(&world_state.pool)
            .await?;
            for (i, obj) in obj_rows.iter().enumerate() {
                let mut obuf = vec![0u8; 96];
                obuf[8..12].copy_from_slice(&obj.size.to_le_bytes());
                obuf[12..16].copy_from_slice(&((i as u32) + 1).to_le_bytes()); // drop_id
                obuf[16..18].copy_from_slice(&(zone_id as u16).to_le_bytes());
                obuf[20..24].copy_from_slice(&(obj.incline as u32).to_le_bytes());
                obuf[28..32].copy_from_slice(&obj.tilt_x.to_le_bytes());
                obuf[32..36].copy_from_slice(&obj.tilt_y.to_le_bytes());
                obuf[36..40].copy_from_slice(&obj.heading.to_le_bytes());
                obuf[40..44].copy_from_slice(&obj.zpos.to_le_bytes());
                obuf[44..48].copy_from_slice(&obj.xpos.to_le_bytes());
                obuf[48..52].copy_from_slice(&obj.ypos.to_le_bytes());
                let name_bytes = obj.objectname.as_bytes();
                let name_len = name_bytes.len().min(31);
                obuf[52..52 + name_len].copy_from_slice(&name_bytes[..name_len]);
                obuf[88..92].copy_from_slice(&(obj.object_type as u32).to_le_bytes());
                obuf[92..96].copy_from_slice(&0xFFu32.to_le_bytes());
                send_app_packet(session, socket, addr, opcodes::OP_GROUND_SPAWN, &obuf).await?;
            }
            if !obj_rows.is_empty() {
                info!(count = obj_rows.len(), "Zone: sent ground objects from DB");
            }

            let zone_short = if cs.char_zone_short.is_empty() { "innothule".to_string() } else { cs.char_zone_short.clone() };
            let zp_rows = sqlx::query_as::<_, ZonePointRow>(
                "SELECT number, target_y, target_x, target_z, target_heading, \
                 target_zone_id, target_instance \
                 FROM zone_points WHERE zone = $1"
            )
            .bind(&zone_short)
            .fetch_all(&world_state.pool)
            .await?;

            let count = zp_rows.len() as u32;
            let entry_size = 24usize; // ZonePoint_Entry: u32 + f32 + f32 + f32 + f32 + u16 + u16
            let mut zp_buf = Vec::with_capacity(4 + (count as usize + 1) * entry_size);
            zp_buf.extend_from_slice(&count.to_le_bytes());
            for row in &zp_rows {
                zp_buf.extend_from_slice(&(row.number as u32).to_le_bytes());
                zp_buf.extend_from_slice(&row.target_y.to_le_bytes());
                zp_buf.extend_from_slice(&row.target_x.to_le_bytes());
                zp_buf.extend_from_slice(&row.target_z.to_le_bytes());
                zp_buf.extend_from_slice(&row.target_heading.to_le_bytes());
                zp_buf.extend_from_slice(&(row.target_zone_id as u16).to_le_bytes());
                zp_buf.extend_from_slice(&(row.target_instance as u16).to_le_bytes());
            }
            zp_buf.extend_from_slice(&[0u8; 24]); // extra empty entry per EQEmu
            send_app_packet(session, socket, addr, opcodes::OP_SEND_ZONE_POINTS, &zp_buf).await?;
            info!(count, zone = %zone_short, "Zone: sent zone points from DB");

            send_app_packet(session, socket, addr, opcodes::OP_SEND_AA_STATS, &[]).await?;
            send_app_packet(session, socket, addr, opcodes::OP_SEND_EXP_ZONEIN, &[]).await?;
            send_app_packet(session, socket, addr, opcodes::OP_WORLD_OBJECTS_SENT, &[]).await?;
            info!("Zone: sent zone ready signals");
        }

        opcodes::OP_CLIENT_READY => {
            let sa = structs::build_spawn_appearance(cs.player_spawn_id, 0x10, cs.player_spawn_id);
            send_app_packet(session, socket, addr, opcodes::OP_SPAWN_APPEARANCE, &sa).await?;

            // OP_ExpUpdate: exp ratio (0-330) + aa exp ratio
            let mut exp_buf = [0u8; 8];
            exp_buf[0..4].copy_from_slice(&0u32.to_le_bytes()); // exp = 0 (bottom of level)
            exp_buf[4..8].copy_from_slice(&0u32.to_le_bytes()); // aaxp = 0
            send_app_packet(session, socket, addr, opcodes::OP_EXP_UPDATE, &exp_buf).await?;

            // OP_RaidUpdate: ZoneInSendName_Struct (136 bytes)
            let mut raid_buf = vec![0u8; 136];
            raid_buf[0..4].copy_from_slice(&0x0Au32.to_le_bytes()); // unknown0 = 0x0A
            let name_bytes = cs.char_name.as_bytes();
            let name_len = name_bytes.len().min(63);
            raid_buf[4..4 + name_len].copy_from_slice(&name_bytes[..name_len]);
            raid_buf[68..68 + name_len].copy_from_slice(&name_bytes[..name_len]);
            send_app_packet(session, socket, addr, opcodes::OP_RAID_UPDATE, &raid_buf).await?;

            // OP_HPUpdate: SpawnHPUpdate_Struct (10 bytes)
            let mut hp_buf = [0u8; 10];
            hp_buf[0..4].copy_from_slice(&1000u32.to_le_bytes());
            hp_buf[4..8].copy_from_slice(&1000i32.to_le_bytes());
            hp_buf[8..10].copy_from_slice(&(cs.player_spawn_id as i16).to_le_bytes());
            send_app_packet(session, socket, addr, opcodes::OP_HP_UPDATE, &hp_buf).await?;

            // OP_GuildMOTD: GuildMOTD_Struct (648 bytes)
            let mut guild_motd = vec![0u8; 648];
            let gm_name = cs.char_name.as_bytes();
            let gm_len = gm_name.len().min(63);
            guild_motd[4..4 + gm_len].copy_from_slice(&gm_name[..gm_len]);
            send_app_packet(session, socket, addr, opcodes::OP_GUILD_MOTD, &guild_motd).await?;

            // Weather re-send (matches EQEmu CompleteConnect)
            send_app_packet(session, socket, addr, opcodes::OP_WEATHER, &structs::build_weather(0, 0)).await?;

            info!(character = %cs.char_name, "=== CLIENT IN ZONE ===");
        }

        opcodes::OP_CLIENT_UPDATE => {
            if data.len() >= 36 {
                cs.last_y = f32::from_le_bytes([data[4], data[5], data[6], data[7]]);
                cs.last_x = f32::from_le_bytes([data[24], data[25], data[26], data[27]]);
                cs.last_z = f32::from_le_bytes([data[28], data[29], data[30], data[31]]);
                let heading_raw = u16::from_le_bytes([data[32], data[33]]) & 0x0FFF;
                cs.last_heading = heading_raw as f32 / 4.0;
            }
        }
        opcodes::OP_CAMP => {
            info!(character = %cs.char_name, x = cs.last_x, y = cs.last_y, z = cs.last_z, "Zone: camp request — saving position");
            save_character_position(&world_state.pool, &cs.char_name,
                cs.last_x, cs.last_y, cs.last_z, cs.last_heading).await?;
        }
        opcodes::OP_LOGOUT => {
            info!(character = %cs.char_name, "Zone: logout — saving and disconnecting");
            save_character_position(&world_state.pool, &cs.char_name,
                cs.last_x, cs.last_y, cs.last_z, cs.last_heading).await?;
            send_app_packet(session, socket, addr, opcodes::OP_PRE_LOGOUT_REPLY, &[]).await?;
            send_app_packet(session, socket, addr, opcodes::OP_LOGOUT_REPLY, &[]).await?;
            let mut disconnect = vec![0x00u8, 0x05];
            disconnect.extend_from_slice(&session.connect_code.to_be_bytes());
            crate::eq_protocol::codec::append_crc(&mut disconnect, session.encode_key, session.crc_bytes);
            socket.send_to(&disconnect, addr).await?;
        }
        opcodes::OP_ACK_PACKET => {}
        opcodes::OP_SEND_AA_TABLE => {
            send_app_packet(session, socket, addr, opcodes::OP_SEND_AA_TABLE, &[]).await?;
        }
        opcodes::OP_UPDATE_AA => {
            send_app_packet(session, socket, addr, opcodes::OP_UPDATE_AA, &[]).await?;
        }
        opcodes::OP_SEND_TRIBUTES => {
            send_app_packet(session, socket, addr, opcodes::OP_SEND_TRIBUTES, &[]).await?;
        }
        opcodes::OP_GUILD_TRIBUTES => {
            send_app_packet(session, socket, addr, opcodes::OP_GUILD_TRIBUTES, &[]).await?;
        }
        opcodes::OP_SEND_EXP_ZONEIN => {
            send_app_packet(session, socket, addr, opcodes::OP_SEND_EXP_ZONEIN, &[]).await?;
        }
        opcodes::OP_CHANNEL_MESSAGE => {
            if !data.is_empty() {
                info!("Zone: chat ({} bytes)", data.len());
            }
        }
        opcodes::OP_SET_SERVER_FILTER => {}
        opcodes::OP_TARGET_MOUSE => {
            if data.len() >= 4 {
                let new_target = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
                if new_target == 0 {
                    cs.target_id = None;
                    debug!("Zone: target cleared");
                } else {
                    cs.target_id = Some(new_target);
                    debug!(target_id = new_target, "Zone: target set");
                }
            }
        }
        opcodes::OP_TARGET_COMMAND => {
            if data.len() >= 4 {
                let new_target = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
                if cs.spawned_npcs.contains_key(&new_target) {
                    cs.target_id = Some(new_target);
                    debug!(target_id = new_target, "Zone: /target command");
                    send_app_packet(session, socket, addr, opcodes::OP_TARGET_COMMAND, data).await?;
                }
            }
        }
        opcodes::OP_CONSIDER => {
            if data.len() >= 8 {
                let target_id = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);

                if let Some(npc) = cs.spawned_npcs.get(&target_id) {
                    let con_color = structs::get_con_color_titanium(cs.player_level, npc.level);
                    let faction: u32 = 5; // FACTION_INDIFFERENTLY

                    let mut resp = vec![0u8; 28];
                    resp[0..4].copy_from_slice(&cs.player_spawn_id.to_le_bytes());
                    resp[4..8].copy_from_slice(&target_id.to_le_bytes());
                    resp[8..12].copy_from_slice(&faction.to_le_bytes());
                    resp[12..16].copy_from_slice(&con_color.to_le_bytes());
                    resp[16..20].copy_from_slice(&(npc.cur_hp as i32).to_le_bytes());
                    resp[20..24].copy_from_slice(&(npc.max_hp as i32).to_le_bytes());
                    // pvpcon=0 and padding already zeroed

                    send_app_packet(session, socket, addr, opcodes::OP_CONSIDER, &resp).await?;
                    info!(
                        target = %npc.name, target_level = npc.level,
                        player_level = cs.player_level, con_color,
                        "Zone: consider"
                    );
                }
            }
        }

        opcodes::OP_AUTO_ATTACK => {
            if data.len() >= 4 {
                let toggle = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
                cs.auto_attack = toggle != 0;
                if cs.auto_attack {
                    cs.last_attack_time = None;
                    info!(character = %cs.char_name, target = ?cs.target_id, "Auto-attack ON");
                } else {
                    cs.last_attack_time = None;
                    info!(character = %cs.char_name, "Auto-attack OFF");
                }
            }
        }
        opcodes::OP_AUTO_ATTACK_2 => {}

        opcodes::OP_ZONE_CHANGE => {
            if data.len() >= 88 {
                let char_name_bytes = &data[0..64];
                let zone_id = u16::from_le_bytes([data[64], data[65]]);
                let _instance_id = u16::from_le_bytes([data[66], data[67]]);
                let zc_y = f32::from_le_bytes([data[68], data[69], data[70], data[71]]);
                let zc_x = f32::from_le_bytes([data[72], data[73], data[74], data[75]]);
                let zc_z = f32::from_le_bytes([data[76], data[77], data[78], data[79]]);

                info!(zone_id, x = zc_x, y = zc_y, z = zc_z, "Zone: zone change request");
                cs.auto_attack = false;

                // Look up target from zone_points if zoneID=0 (unsolicited)
                let zone_short = if cs.char_zone_short.is_empty() { "innothule".to_string() } else { cs.char_zone_short.clone() };
                let target = if zone_id == 0 {
                    sqlx::query_as::<_, (i32, f32, f32, f32, f32)>(
                        "SELECT target_zone_id, target_x, target_y, target_z, target_heading \
                         FROM zone_points WHERE zone = $1 \
                         ORDER BY (($2 - x) * ($2 - x) + ($3 - y) * ($3 - y) + ($4 - z) * ($4 - z)) \
                         LIMIT 1"
                    )
                    .bind(&zone_short)
                    .bind(zc_x)
                    .bind(zc_y)
                    .bind(zc_z)
                    .fetch_optional(&world_state.pool)
                    .await?
                } else {
                    sqlx::query_as::<_, (i32, f32, f32, f32, f32)>(
                        "SELECT target_zone_id, target_x, target_y, target_z, target_heading \
                         FROM zone_points WHERE zone = $1 AND target_zone_id = $2 \
                         ORDER BY (($3 - x) * ($3 - x) + ($4 - y) * ($4 - y) + ($5 - z) * ($5 - z)) \
                         LIMIT 1"
                    )
                    .bind(&zone_short)
                    .bind(zone_id as i32)
                    .bind(zc_x)
                    .bind(zc_y)
                    .bind(zc_z)
                    .fetch_optional(&world_state.pool)
                    .await?
                };

                let mut resp = vec![0u8; 88];
                resp[0..64].copy_from_slice(char_name_bytes);

                if let Some((target_zone_id, tx, ty, tz, _th)) = target {
                    resp[64..66].copy_from_slice(&(target_zone_id as u16).to_le_bytes());
                    resp[68..72].copy_from_slice(&ty.to_le_bytes());
                    resp[72..76].copy_from_slice(&tx.to_le_bytes());
                    resp[76..80].copy_from_slice(&tz.to_le_bytes());
                    resp[84..88].copy_from_slice(&1i32.to_le_bytes()); // success = 1
                    info!(target_zone_id, "Zone: approved zone change");

                    // Update character's zone in DB so world handler loads the new zone
                    cs.zone_transition_pending = true;
                    sqlx::query("UPDATE character_data SET zone_id = $1, x = $2, y = $3, z = $4 WHERE name = $5")
                        .bind(target_zone_id)
                        .bind(tx)
                        .bind(ty)
                        .bind(tz)
                        .bind(&cs.char_name)
                        .execute(&world_state.pool)
                        .await?;
                } else {
                    // Cancel — zone back to current zone
                    let current_zone = cs.char_zone_id.unwrap_or(46) as u16;
                    resp[64..66].copy_from_slice(&current_zone.to_le_bytes());
                    resp[84..88].copy_from_slice(&1i32.to_le_bytes());
                    info!("Zone: no valid zone point found, cancelling");
                }
                send_app_packet(session, socket, addr, opcodes::OP_ZONE_CHANGE, &resp).await?;

                // Disconnect zone session so client reconnects through world
                if target.is_some() {
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    let mut disconnect = vec![0x00u8, 0x05];
                    disconnect.extend_from_slice(&session.connect_code.to_be_bytes());
                    crate::eq_protocol::codec::append_crc(&mut disconnect, session.encode_key, session.crc_bytes);
                    socket.send_to(&disconnect, addr).await?;
                    info!("Zone: sent session disconnect for zone transition");
                }
            }
        }

        opcodes::OP_LOOT_REQUEST => {
            if data.len() >= 4 {
                let corpse_id = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);

                let is_valid_corpse = cs.spawned_npcs.get(&corpse_id)
                    .map(|n| n.is_corpse)
                    .unwrap_or(false);

                if !is_valid_corpse {
                    let err = structs::build_money_on_corpse(2, 0, 0, 0, 0);
                    send_app_packet(session, socket, addr, opcodes::OP_MONEY_ON_CORPSE, &err).await?;
                } else {
                    let loottable_id = cs.spawned_npcs.get(&corpse_id).unwrap().loottable_id;
                    let needs_resolve = loottable_id > 0
                        && cs.spawned_npcs.get(&corpse_id).unwrap().loot_items.is_empty()
                        && cs.spawned_npcs.get(&corpse_id).unwrap().loot_platinum == 0
                        && cs.spawned_npcs.get(&corpse_id).unwrap().loot_gold == 0
                        && cs.spawned_npcs.get(&corpse_id).unwrap().loot_silver == 0
                        && cs.spawned_npcs.get(&corpse_id).unwrap().loot_copper == 0;

                    if needs_resolve {
                        let (items, plat, gold, silver, copper) =
                            resolve_npc_loot(&world_state.pool, loottable_id).await?;
                        let corpse = cs.spawned_npcs.get_mut(&corpse_id).unwrap();
                        corpse.loot_items = items;
                        corpse.loot_platinum = plat;
                        corpse.loot_gold = gold;
                        corpse.loot_silver = silver;
                        corpse.loot_copper = copper;
                    }

                    let corpse = cs.spawned_npcs.get(&corpse_id).unwrap();
                    let money_pkt = structs::build_money_on_corpse(
                        1, corpse.loot_platinum, corpse.loot_gold,
                        corpse.loot_silver, corpse.loot_copper,
                    );
                    send_app_packet(session, socket, addr, opcodes::OP_MONEY_ON_CORPSE, &money_pkt).await?;

                    let item_count = corpse.loot_items.len();
                    for item in &corpse.loot_items {
                        let serialized = structs::serialize_titanium_item(item, item.item_db_id);
                        send_app_packet(session, socket, addr, opcodes::OP_ITEM_PACKET, &serialized).await?;
                    }

                    send_app_packet(session, socket, addr, opcodes::OP_LOOT_REQUEST, data).await?;

                    let corpse = cs.spawned_npcs.get_mut(&corpse_id).unwrap();
                    corpse.loot_platinum = 0;
                    corpse.loot_gold = 0;
                    corpse.loot_silver = 0;
                    corpse.loot_copper = 0;

                    info!(corpse_id, items = item_count, "Loot: opened corpse");
                }
            }
        }

        opcodes::OP_LOOT_ITEM => {
            if data.len() >= 12 {
                let lootee = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
                let slot_id = u16::from_le_bytes([data[8], data[9]]);

                if let Some(corpse) = cs.spawned_npcs.get_mut(&lootee) {
                    if corpse.is_corpse {
                        if let Some(idx) = corpse.loot_items.iter()
                            .position(|item| item.slot_id == slot_id as i32)
                        {
                            let item = corpse.loot_items.remove(idx);
                            info!(item_name = %item.name, slot = slot_id, "Loot: item taken");
                        }
                    }
                }

                send_app_packet(session, socket, addr, opcodes::OP_LOOT_ITEM, data).await?;
            }
        }

        opcodes::OP_END_LOOT_REQUEST => {
            if data.len() >= 4 {
                let corpse_id = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);

                send_app_packet(session, socket, addr, opcodes::OP_LOOT_COMPLETE, &[]).await?;

                let should_despawn = cs.spawned_npcs.get(&corpse_id)
                    .map(|c| c.is_corpse && c.loot_items.is_empty()
                        && c.loot_platinum == 0 && c.loot_gold == 0
                        && c.loot_silver == 0 && c.loot_copper == 0)
                    .unwrap_or(false);

                if should_despawn {
                    send_app_packet(session, socket, addr, opcodes::OP_DELETE_SPAWN,
                        &corpse_id.to_le_bytes()).await?;
                    cs.spawned_npcs.remove(&corpse_id);
                    info!(corpse_id, "Loot: empty corpse despawned");
                }

                info!(corpse_id, "Loot: window closed");
            }
        }

        opcodes::OP_FLOAT_LIST_THING => {
            let entry_size = 17usize;
            let count = data.len() / entry_size;
            for i in 0..count {
                let off = i * entry_size;
                if off + entry_size <= data.len() {
                    let _y = f32::from_le_bytes([data[off], data[off+1], data[off+2], data[off+3]]);
                    let _x = f32::from_le_bytes([data[off+4], data[off+5], data[off+6], data[off+7]]);
                    let _z = f32::from_le_bytes([data[off+8], data[off+9], data[off+10], data[off+11]]);
                    let move_type = data[off+12];
                    if move_type == 4 {
                        debug!("MovementHistory: zone line detected");
                    } else if move_type == 3 {
                        debug!("MovementHistory: teleport detected");
                    }
                }
            }
            debug!(entries = count, "Zone: movement history received");
        }

        opcodes::OP_SPAWN_APPEARANCE => {
            if data.len() >= 8 {
                let spawn_id = u16::from_le_bytes([data[0], data[1]]);
                let appear_type = u16::from_le_bytes([data[2], data[3]]);
                let parameter = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
                debug!(spawn_id, appear_type, parameter, "Zone: spawn appearance");
            }
        }

        opcodes::OP_PLAYER_STATE_ADD => {
            if data.len() >= 8 {
                let spawn_id = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
                let state = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
                debug!(spawn_id, state, "Zone: player state add");
            }
        }

        opcodes::OP_WEAR_CHANGE => {
            debug!(len = data.len(), "Zone: wear change (no-op, matching EQEmu)");
        }

        opcodes::OP_WEAPON_EQUIP_1 => {
            debug!(len = data.len(), "Zone: weapon equip visual (no-op, matching EQEmu)");
        }

        _ => {
            debug!(opcode = format!("0x{opcode:04X}"), len = data.len(), "Zone: unhandled");
        }
    }
    Ok(())
}

async fn save_character_position(
    pool: &sqlx::PgPool, char_name: &str, x: f32, y: f32, z: f32, heading: f32,
) -> anyhow::Result<()> {
    sqlx::query("UPDATE character_data SET x = $1, y = $2, z = $3, heading = $4 WHERE name = $5")
        .bind(x).bind(y).bind(z).bind(heading).bind(char_name)
        .execute(pool).await?;
    Ok(())
}
