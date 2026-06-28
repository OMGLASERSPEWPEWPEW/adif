"""Multi-port UDP proxy for capturing EQ Titanium client <-> EQEmu server packets.

Sits between the EQ client and EQEmu reference server on all game ports.
Logs every packet with opcode-aware decoding and full hex dumps.

Port layout (proxy intercepts, forwards to offset EQEmu ports):
  Client -> :5998 (proxy) -> :6998 (EQEmu login)
  Client -> :9000 (proxy) -> :9010 (EQEmu world)

Setup:
  1. login.json:        titanium_port = 6998
  2. eqemu_config.json: world.telnet.port = "9010"
  3. eqhost.txt:        keep 127.0.0.1:5998 (hits proxy)
  4. Run: python scripts/udp-proxy.py
  5. Start EQEmu servers, then EQ client

Usage:
  python scripts/udp-proxy.py                    # default ports
  python scripts/udp-proxy.py --login 5998:6998  # custom login mapping
  python scripts/udp-proxy.py --no-world         # login only
"""

import asyncio
import struct
import sys
import datetime
import argparse
from pathlib import Path

# --- EQ Session Protocol Opcodes (bytes 0-1, big-endian) ---

SESSION_OPCODES = {
    0x0001: "SessionRequest",
    0x0002: "SessionResponse",
    0x0003: "Combined",
    0x0004: "SessionDisconnect",
    0x0006: "KeepAlive",
    0x0007: "SessionStatRequest",
    0x0008: "SessionStatResponse",
    0x0009: "Packet",
    0x000A: "OversizedPacket",
    0x000D: "Fragment",
    0x0011: "OutOfOrder",
    0x0015: "Ack",
}

# --- EQ Application Opcodes (from titanium/opcodes.rs) ---

# Login phase (login server <-> client)
LOGIN_OPCODES = {
    0x0001: "OP_LoginSessionReady",
    0x0002: "OP_LoginLogin",
    0x0004: "OP_LoginServerListRequest",
    0x000D: "OP_LoginPlayRequest",
    0x0016: "OP_LoginChatMessage",
    0x0017: "OP_LoginAccepted",
    0x0018: "OP_LoginServerListResponse",
    0x0021: "OP_LoginPlayResponse",
}

# World + Zone phase (game server <-> client)
APP_OPCODES = {
    # World
    0x6957: "OP_GuildsList",
    0x3C25: "OP_ApproveWorld",
    0x0FA6: "OP_LogServer",
    0x024D: "OP_MOTD",
    0x04EC: "OP_ExpansionInfo",
    0x52A4: "OP_PostEnterWorld",
    0x4513: "OP_SendCharInfo",
    0x61B6: "OP_ZoneServerInfo",
    0x4DD0: "OP_SendLoginInfo",
    0x7CBA: "OP_EnterWorld",
    0x509D: "OP_WorldComplete",
    # Zone
    0x7213: "OP_ZoneEntry",
    0x75DF: "OP_PlayerProfile",
    0x0920: "OP_NewZone",
    0x0322: "OP_ReqClientSpawn",
    0x2E78: "OP_ZoneSpawns",
    0x6563: "OP_SetServerFilter",
    0x3EBA: "OP_SendZonepoints",
    0x7AC5: "OP_ReqNewZone",
    0x1580: "OP_TimeOfDay",
    0x0587: "OP_SendExpZonein",
    0x65CA: "OP_Consider",
    0x7C32: "OP_SpawnAppearance",
    0x55BC: "OP_DeleteSpawn",
    0x5E20: "OP_ClientReady",
    0x1860: "OP_NewSpawn",
    0x254D: "OP_Weather",
    0x6C47: "OP_TargetMouse",
    0x14CB: "OP_ClientUpdate",
    0x1004: "OP_ChannelMessage",
    0x3BCF: "OP_HPUpdate",
    0x7752: "OP_AckPacket",
    0x7825: "OP_CrashDump",
}


def hex_dump(data, width=32):
    """Full hex + ASCII dump."""
    lines = []
    for i in range(0, len(data), width):
        chunk = data[i:i + width]
        hex_str = " ".join(f"{b:02X}" for b in chunk)
        ascii_str = "".join(chr(b) if 32 <= b < 127 else "." for b in chunk)
        lines.append(f"  {i:04X}  {hex_str:<{width * 3}}  {ascii_str}")
    return "\n".join(lines)


def decode_packet(data, phase):
    """Peek at EQ protocol layers to extract meaningful info."""
    if len(data) < 2:
        return "TooShort", None, None

    proto_op = struct.unpack_from(">H", data, 0)[0]
    proto_name = SESSION_OPCODES.get(proto_op, f"Proto_0x{proto_op:04X}")

    app_op = None
    app_name = None

    if proto_op == 0x0009 and len(data) >= 6:
        # Single app packet: [proto_op:2][seq:2][app_op:2][data...]
        app_op = struct.unpack_from("<H", data, 4)[0]
        if phase == "LOGIN":
            app_name = LOGIN_OPCODES.get(app_op, APP_OPCODES.get(app_op, f"0x{app_op:04X}"))
        else:
            app_name = APP_OPCODES.get(app_op, f"0x{app_op:04X}")

    elif proto_op == 0x0003 and len(data) >= 4:
        # Combined packet - decode first sub-packet's opcode
        sub_len = data[2]
        if len(data) >= 3 + sub_len and sub_len >= 2:
            app_op = struct.unpack_from("<H", data, 3)[0]
            if phase == "LOGIN":
                app_name = LOGIN_OPCODES.get(app_op, APP_OPCODES.get(app_op, f"0x{app_op:04X}"))
            else:
                app_name = APP_OPCODES.get(app_op, f"0x{app_op:04X}")
            app_name = f"{app_name}+..." # indicate there are more sub-packets

    elif proto_op == 0x0001 and len(data) >= 12:
        # SessionRequest - extract protocol version and session ID
        session_id = struct.unpack_from(">I", data, 2)[0]
        proto_ver = struct.unpack_from(">I", data, 6)[0]
        app_name = f"session=0x{session_id:08X} proto={proto_ver}"

    elif proto_op == 0x0002 and len(data) >= 19:
        # SessionResponse - extract session ID, CRC bytes, encoding
        session_id = struct.unpack_from(">I", data, 2)[0]
        crc_bytes = data[10]
        app_name = f"session=0x{session_id:08X} crc_bytes={crc_bytes}"

    return proto_name, app_op, app_name


class ProxyEndpoint(asyncio.DatagramProtocol):
    """Proxies UDP between a client and a server, logging everything."""

    def __init__(self, phase, server_host, server_port, log_file):
        self.phase = phase
        self.server_addr = (server_host, server_port)
        self.log_file = log_file
        self.transport = None
        self.client_addr = None
        self.pkt_count = 0

    def connection_made(self, transport):
        self.transport = transport
        sock = transport.get_extra_info("socket")
        local = sock.getsockname()
        print(f"  [{self.phase}] Listening on :{local[1]} -> {self.server_addr[0]}:{self.server_addr[1]}")

    def datagram_received(self, data, addr):
        self.pkt_count += 1
        ts = datetime.datetime.now().strftime("%H:%M:%S.%f")[:-3]

        if addr[0] == self.server_addr[0] and addr[1] == self.server_addr[1]:
            direction = "SERVER->CLIENT"
            arrow = "<--"
            if self.client_addr:
                self.transport.sendto(data, self.client_addr)
        else:
            direction = "CLIENT->SERVER"
            arrow = "-->"
            self.client_addr = addr
            self.transport.sendto(data, self.server_addr)

        proto_name, app_op, app_name = decode_packet(data, self.phase)

        # Console: one-line summary
        opcode_str = f" {app_name}" if app_name else ""
        print(f"  [{ts}] {self.phase:5s} #{self.pkt_count:4d} {arrow} {direction:15s} {len(data):5d}B  {proto_name}{opcode_str}")

        # Log file: full detail
        entry = (
            f"[{ts}] #{self.pkt_count} {self.phase} {direction} ({len(data)} bytes) "
            f"from {addr[0]}:{addr[1]}\n"
            f"  Protocol: {proto_name}"
        )
        if app_op is not None:
            entry += f"  AppOpcode: 0x{app_op:04X}"
        if app_name:
            entry += f" ({app_name})"
        entry += f"\n{hex_dump(data)}\n\n"

        self.log_file.write(entry)
        self.log_file.flush()


async def run_proxy(routes, log_path):
    """Start proxy endpoints for all configured routes."""
    loop = asyncio.get_event_loop()

    ts = datetime.datetime.now().strftime("%Y%m%d_%H%M%S")
    actual_log_path = log_path or f"scripts/capture-{ts}.log"

    print(f"\n{'=' * 70}")
    print(f"  EQ Packet Proxy - {datetime.datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    print(f"  Log: {actual_log_path}")
    print(f"{'=' * 70}\n")

    log_file = open(actual_log_path, "w", encoding="utf-8")
    log_file.write(f"=== EQ Packet Capture {datetime.datetime.now()} ===\n")
    log_file.write(f"Routes:\n")
    for phase, (listen_port, server_host, server_port) in routes.items():
        log_file.write(f"  {phase}: :{listen_port} -> {server_host}:{server_port}\n")
    log_file.write(f"\n")

    transports = []
    for phase, (listen_port, server_host, server_port) in routes.items():
        transport, _ = await loop.create_datagram_endpoint(
            lambda ph=phase, sh=server_host, sp=server_port: ProxyEndpoint(ph, sh, sp, log_file),
            local_addr=("0.0.0.0", listen_port),
        )
        transports.append(transport)

    print(f"\n  Waiting for packets... (Ctrl+C to stop)\n")
    print(f"  {'-' * 66}")

    try:
        await asyncio.Future()  # run forever
    except asyncio.CancelledError:
        pass
    finally:
        for t in transports:
            t.close()
        log_file.close()
        print(f"\n  Capture saved to {actual_log_path}")


def parse_port_mapping(s):
    """Parse 'listen:forward' into (listen_port, forward_port)."""
    parts = s.split(":")
    if len(parts) != 2:
        raise argparse.ArgumentTypeError(f"Expected listen:forward, got '{s}'")
    return int(parts[0]), int(parts[1])


def main():
    parser = argparse.ArgumentParser(
        description="Multi-port UDP proxy for EQ Titanium packet capture"
    )
    parser.add_argument(
        "--login", type=parse_port_mapping, default="5998:6998",
        help="Login proxy listen:forward ports (default: 5998:6998)"
    )
    parser.add_argument(
        "--world", type=parse_port_mapping, default="9000:9010",
        help="World proxy listen:forward ports (default: 9000:9010)"
    )
    parser.add_argument(
        "--zone", type=parse_port_mapping, default=None,
        help="Optional zone proxy listen:forward ports (e.g., 7000:7001)"
    )
    parser.add_argument(
        "--no-login", action="store_true",
        help="Skip login proxy"
    )
    parser.add_argument(
        "--no-world", action="store_true",
        help="Skip world proxy"
    )
    parser.add_argument(
        "--host", default="127.0.0.1",
        help="Server host to forward to (default: 127.0.0.1)"
    )
    parser.add_argument(
        "--log", default=None,
        help="Log file path (default: scripts/capture-<timestamp>.log)"
    )
    args = parser.parse_args()

    routes = {}
    if not args.no_login:
        listen, forward = args.login
        routes["LOGIN"] = (listen, args.host, forward)
    if not args.no_world:
        listen, forward = args.world
        routes["WORLD"] = (listen, args.host, forward)
    if args.zone:
        listen, forward = args.zone
        routes["ZONE"] = (listen, args.host, forward)

    if not routes:
        print("No routes configured. Use --login, --world, or --zone.")
        sys.exit(1)

    try:
        asyncio.run(run_proxy(routes, args.log))
    except KeyboardInterrupt:
        print("\n  Shutting down.")


if __name__ == "__main__":
    main()
