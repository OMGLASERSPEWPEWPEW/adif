"""Multi-port UDP proxy for capturing EQ Titanium client <-> EQEmu server packets.

Sits between the EQ client and EQEmu reference server on all game ports.
Logs every packet with opcode-aware decoding and full hex dumps.
Rewrites ZoneServerInfo port so zone traffic flows through the proxy.

Port layout (proxy intercepts, forwards to offset EQEmu ports):
  Client -> :5998 (proxy) -> :6998 (EQEmu login)
  Client -> :9000 (proxy) -> :9010 (EQEmu world)
  Client -> :7000 (proxy) -> :7001 (EQEmu zone)

Setup:
  1. login.json:        titanium_port = 6998
  2. eqemu_config.json: world.telnet.port = "9010", zones.ports.low = "7001"
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

# --- EQ CRC32 table — copied verbatim from codec.rs (which was ported from EQEmu crc32.cpp) ---
CRC32_TABLE = [
    0x00000000, 0x77073096, 0xEE0E612C, 0x990951BA, 0x076DC419, 0x706AF48F, 0xE963A535, 0x9E6495A3,
    0x0EDB8832, 0x79DCB8A4, 0xE0D5E91E, 0x97D2D988, 0x09B64C2B, 0x7EB17CBD, 0xE7B82D07, 0x90BF1D91,
    0x1DB71064, 0x6AB020F2, 0xF3B97148, 0x84BE41DE, 0x1ADAD47D, 0x6DDDE4EB, 0xF4D4B551, 0x83D385C7,
    0x136C9856, 0x646BA8C0, 0xFD62F97A, 0x8A65C9EC, 0x14015C4F, 0x63066CD9, 0xFA0F3D63, 0x8D080DF5,
    0x3B6E20C8, 0x4C69105E, 0xD56041E4, 0xA2677172, 0x3C03E4D1, 0x4B04D447, 0xD20D85FD, 0xA50AB56B,
    0x35B5A8FA, 0x42B2986C, 0xDBBBC9D6, 0xACBCF940, 0x32D86CE3, 0x45DF5C75, 0xDCD60DCF, 0xABD13D59,
    0x26D930AC, 0x51DE003A, 0xC8D75180, 0xBFD06116, 0x21B4F4B5, 0x56B3C423, 0xCFBA9599, 0xB8BDA50F,
    0x2802B89E, 0x5F058808, 0xC60CD9B2, 0xB10BE924, 0x2F6F7C87, 0x58684C11, 0xC1611DAB, 0xB6662D3D,
    0x76DC4190, 0x01DB7106, 0x98D220BC, 0xEFD5102A, 0x71B18589, 0x06B6B51F, 0x9FBFE4A5, 0xE8B8D433,
    0x7807C9A2, 0x0F00F934, 0x9609A88E, 0xE10E9818, 0x7F6A0DBB, 0x086D3D2D, 0x91646C97, 0xE6635C01,
    0x6B6B51F4, 0x1C6C6162, 0x856530D8, 0xF262004E, 0x6C0695ED, 0x1B01A57B, 0x8208F4C1, 0xF50FC457,
    0x65B0D9C6, 0x12B7E950, 0x8BBEB8EA, 0xFCB9887C, 0x62DD1DDF, 0x15DA2D49, 0x8CD37CF3, 0xFBD44C65,
    0x4DB26158, 0x3AB551CE, 0xA3BC0074, 0xD4BB30E2, 0x4ADFA541, 0x3DD895D7, 0xA4D1C46D, 0xD3D6F4FB,
    0x4369E96A, 0x346ED9FC, 0xAD678846, 0xDA60B8D0, 0x44042D73, 0x33031DE5, 0xAA0A4C5F, 0xDD0D7CC9,
    0x5005713C, 0x270241AA, 0xBE0B1010, 0xC90C2086, 0x5768B525, 0x206F85B3, 0xB966D409, 0xCE61E49F,
    0x5EDEF90E, 0x29D9C998, 0xB0D09822, 0xC7D7A8B4, 0x59B33D17, 0x2EB40D81, 0xB7BD5C3B, 0xC0BA6CAD,
    0xEDB88320, 0x9ABFB3B6, 0x03B6E20C, 0x74B1D29A, 0xEAD54739, 0x9DD277AF, 0x04DB2615, 0x73DC1683,
    0xE3630B12, 0x94643B84, 0x0D6D6A3E, 0x7A6A5AA8, 0xE40ECF0B, 0x9309FF9D, 0x0A00AE27, 0x7D079EB1,
    0xF00F9344, 0x8708A3D2, 0x1E01F268, 0x6906C2FE, 0xF762575D, 0x806567CB, 0x196C3671, 0x6E6B06E7,
    0xFED41B76, 0x89D32BE0, 0x10DA7A5A, 0x67DD4ACC, 0xF9B9DF6F, 0x8EBEEFF9, 0x17B7BE43, 0x60B08ED5,
    0xD6D6A3E8, 0xA1D1937E, 0x38D8C2C4, 0x4FDFF252, 0xD1BB67F1, 0xA6BC5767, 0x3FB506DD, 0x48B2364B,
    0xD80D2BDA, 0xAF0A1B4C, 0x36034AF6, 0x41047A60, 0xDF60EFC3, 0xA867DF55, 0x316E8EEF, 0x4669BE79,
    0xCB61B38C, 0xBC66831A, 0x256FD2A0, 0x5268E236, 0xCC0C7795, 0xBB0B4703, 0x220216B9, 0x5505262F,
    0xC5BA3BBE, 0xB2BD0B28, 0x2BB45A92, 0x5CB36A04, 0xC2D7FFA7, 0xB5D0CF31, 0x2CD99E8B, 0x5BDEAE1D,
    0x9B64C2B0, 0xEC63F226, 0x756AA39C, 0x026D930A, 0x9C0906A9, 0xEB0E363F, 0x72076785, 0x05005713,
    0x95BF4A82, 0xE2B87A14, 0x7BB12BAE, 0x0CB61B38, 0x92D28E9B, 0xE5D5BE0D, 0x7CDCEFB7, 0x0BDBDF21,
    0x86D3D2D4, 0xF1D4E242, 0x68DDB3F8, 0x1FDA836E, 0x81BE16CD, 0xF6B9265B, 0x6FB077E1, 0x18B74777,
    0x88085AE6, 0xFF0F6A70, 0x66063BCA, 0x11010B5C, 0x8F659EFF, 0xF862AE69, 0x616BFFD3, 0x166CCF45,
    0xA00AE278, 0xD70DD2EE, 0x4E048354, 0x3903B3C2, 0xA7672661, 0xD06016F7, 0x4969474D, 0x3E6E77DB,
    0xAED16A4A, 0xD9D65ADC, 0x40DF0B66, 0x37D83BF0, 0xA9BCAE53, 0xDEBB9EC5, 0x47B2CF7F, 0x30B5FFE9,
    0xBDBDF21C, 0xCABAC28A, 0x53B39330, 0x24B4A3A6, 0xBAD03605, 0xCDD70693, 0x54DE5729, 0x23D967BF,
    0xB3667A2E, 0xC4614AB8, 0x5D681B02, 0x2A6F2B94, 0xB40BBE37, 0xC30C8EA1, 0x5A05DF1B, 0x2D02EF8D,
]

def eq_crc16(data, key):
    """CRC16 as used by EQ session layer: lower 16 bits of CRC32 with key prepended LE."""
    crc = 0xFFFFFFFF
    key_bytes = struct.pack("<I", key)
    for b in key_bytes:
        crc = (crc >> 8) ^ CRC32_TABLE[(b ^ crc) & 0xFF]
    for b in data:
        crc = (crc >> 8) ^ CRC32_TABLE[(b ^ crc) & 0xFF]
    return (~crc) & 0xFFFF

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

    def __init__(self, phase, server_host, server_port, log_file, zone_listen_port=None):
        self.phase = phase
        self.server_addr = (server_host, server_port)
        self.log_file = log_file
        self.zone_listen_port = zone_listen_port
        self.transport = None
        self.client_addr = None
        self.pkt_count = 0
        self.encode_key = 0

    def connection_made(self, transport):
        self.transport = transport
        sock = transport.get_extra_info("socket")
        local = sock.getsockname()
        print(f"  [{self.phase}] Listening on :{local[1]} -> {self.server_addr[0]}:{self.server_addr[1]}")

    def datagram_received(self, data, addr):
        self.pkt_count += 1
        ts = datetime.datetime.now().strftime("%H:%M:%S.%f")[:-3]

        is_from_server = (addr[0] == self.server_addr[0] and addr[1] == self.server_addr[1])

        if is_from_server:
            direction = "SERVER->CLIENT"
            arrow = "<--"
            # Track encode_key from SessionResponse
            if len(data) >= 11 and data[0] == 0x00 and data[1] == 0x02:
                self.encode_key = struct.unpack_from(">I", data, 6)[0]
            # Rewrite ZoneServerInfo port if zone proxy is configured
            if self.phase == "WORLD" and self.zone_listen_port and len(data) >= 138:
                if data[0] == 0x00 and data[1] == 0x09 and len(data) >= 8:
                    app_op = struct.unpack_from("<H", data, 4)[0]
                    if app_op == 0x61B6:  # OP_ZoneServerInfo
                        old_port = struct.unpack_from("<H", data, 134)[0]
                        data = bytearray(data)
                        struct.pack_into("<H", data, 134, self.zone_listen_port)
                        # Recalculate CRC (last 2 bytes)
                        body = bytes(data[:-2])
                        crc = eq_crc16(body, self.encode_key)
                        struct.pack_into(">H", data, len(data) - 2, crc)
                        data = bytes(data)
                        print(f"  *** Rewrote ZoneServerInfo port {old_port} -> {self.zone_listen_port}")
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

    zone_listen_port = routes["ZONE"][0] if "ZONE" in routes else None

    transports = []
    for phase, (listen_port, server_host, server_port) in routes.items():
        zlp = zone_listen_port if phase == "WORLD" else None
        transport, _ = await loop.create_datagram_endpoint(
            lambda ph=phase, sh=server_host, sp=server_port, z=zlp: ProxyEndpoint(ph, sh, sp, log_file, z),
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
