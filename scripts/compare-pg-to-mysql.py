"""
Compare PostgreSQL (ADIF) against MariaDB (akk-stack) for 100% schema parity.

Usage: python scripts/compare-pg-to-mysql.py [--generate-ddl]

Connects to both databases, compares every table and column, and reports gaps.
With --generate-ddl, outputs PostgreSQL CREATE TABLE statements for missing tables.
"""

import sys
import json
from collections import defaultdict

import pymysql
import psycopg2

MYSQL_CONFIG = {
    "host": "127.0.0.1",
    "port": 3306,
    "user": "eqemu",
    "password": "eqemu-adif-2026",
    "database": "peq",
}

PG_CONFIG = {
    "host": "127.0.0.1",
    "port": 5433,
    "user": "adif",
    "password": "adif_dev",
    "database": "adif",
}

# MySQL type -> PostgreSQL type mapping
TYPE_MAP = {
    "tinyint": "smallint",
    "smallint": "smallint",
    "mediumint": "integer",
    "int": "integer",
    "bigint": "bigint",
    "float": "real",
    "double": "double precision",
    "decimal": "numeric",
    "varchar": "character varying",
    "char": "character",
    "tinytext": "text",
    "text": "text",
    "mediumtext": "text",
    "longtext": "text",
    "tinyblob": "bytea",
    "blob": "bytea",
    "mediumblob": "bytea",
    "longblob": "bytea",
    "datetime": "timestamp without time zone",
    "timestamp": "timestamp without time zone",
    "date": "date",
    "time": "time without time zone",
    "year": "smallint",
    "binary": "bytea",
    "varbinary": "bytea",
    "bit": "bit",
    "set": "text",
    "enum": "character varying",
    "json": "jsonb",
}

# database_schema.h categories for grouping
SCHEMA_CATEGORIES = {
    "Character": [
        "adventure_stats", "char_recipe_list", "character_activities",
        "character_alt_currency", "character_alternate_abilities",
        "character_auras", "character_bandolier", "character_bind",
        "character_buffs", "character_corpses", "character_currency",
        "character_data", "character_disciplines", "character_enabledtasks",
        "character_expedition_lockouts", "character_exp_modifiers",
        "character_evolving_items", "character_inspect_messages",
        "character_instance_safereturns", "character_item_recast",
        "character_languages", "character_leadership_abilities",
        "character_material", "character_memmed_spells",
        "character_parcels", "character_parcels_containers",
        "character_pet_buffs", "character_pet_info",
        "character_pet_inventory", "character_pet_name",
        "character_peqzone_flags", "character_potionbelt",
        "character_skills", "character_spells", "character_stats_record",
        "character_task_timers", "character_tasks", "character_tribute",
        "completed_tasks", "data_buckets", "faction_values", "friends",
        "guild_members", "guilds", "instance_list_player", "inventory",
        "inventory_snapshots", "keyring", "mail", "player_titlesets",
        "quest_globals", "timers", "trader", "zone_flags",
    ],
    "Content": [
        "aa_ability", "aa_rank_effects", "aa_rank_prereqs", "aa_ranks",
        "adventure_template", "adventure_template_entry",
        "adventure_template_entry_flavor", "alternate_currency", "auras",
        "base_data", "blocked_spells", "books", "char_create_combinations",
        "char_create_point_allocations", "damageshieldtypes", "doors",
        "dynamic_zone_templates", "faction_association", "faction_base_data",
        "faction_list", "faction_list_mod", "fishing", "forage",
        "global_loot", "graveyard", "grid", "grid_entries",
        "ground_spawns", "horses", "items", "items_evolving_details",
        "ldon_trap_entries", "ldon_trap_templates", "lootdrop",
        "lootdrop_entries", "loottable", "loottable_entries",
        "merchantlist", "npc_emotes", "npc_faction", "npc_faction_entries",
        "npc_scale_global_base", "npc_spells", "npc_spells_effects",
        "npc_spells_effects_entries", "npc_spells_entries", "npc_types",
        "npc_types_tint", "object", "pets", "pets_beastlord_data",
        "pets_equipmentset", "pets_equipmentset_entries", "skill_caps",
        "spawn2", "spawn_conditions", "spawnentry", "spawngroup",
        "spells_new", "start_zones", "starting_items", "task_activities",
        "tasks", "tasksets", "tradeskill_recipe", "tradeskill_recipe_entries",
        "traps", "tribute_levels", "tributes", "veteran_reward_templates",
        "zone", "zone_points",
    ],
    "Server": [
        "chatchannels", "chatchannel_reserved_names", "command_settings",
        "command_subsettings", "content_flags", "db_str", "eqtime",
        "launcher", "launcher_zones", "spawn_condition_values",
        "spawn_events", "level_exp_mods", "logsys_categories",
        "name_filter", "perl_event_export_settings", "profanity_list",
        "rule_sets", "titles", "rule_values", "variables",
    ],
    "State": [
        "adventure_members", "banned_ips", "bug_reports", "bugs", "buyer",
        "buyer_buy_lines", "buyer_trade_items",
        "completed_shared_task_activity_state",
        "completed_shared_task_members", "completed_shared_tasks",
        "discord_webhooks", "dynamic_zone_lockouts", "dynamic_zone_members",
        "dynamic_zones", "gm_ips", "group_id", "group_leaders",
        "instance_list", "ip_exemptions", "lfguild", "merc_buffs",
        "merchantlist_temp", "mercs", "object_contents", "raid_details",
        "raid_leaders", "raid_members", "reports", "respawn_times",
        "saylink", "server_scheduled_events", "spawn2_disabled",
        "player_event_aa_purchase", "player_event_killed_npc",
        "player_event_killed_named_npc", "player_event_killed_raid_npc",
        "player_event_log_settings", "player_event_logs",
        "player_event_loot_items", "player_event_merchant_purchase",
        "player_event_merchant_sell", "player_event_npc_handin",
        "player_event_npc_handin_entries", "player_event_speech",
        "player_event_trade", "player_event_trade_entries",
        "shared_task_activity_state", "shared_task_dynamic_zones",
        "shared_task_members", "shared_tasks", "zone_state_spawns",
    ],
    "Login": [
        "login_accounts", "login_api_tokens", "login_server_admins",
        "login_server_list_types", "login_world_servers",
    ],
    "Version": ["db_version", "inventory_versions"],
    "Bot": [
        "bot_blocked_buffs", "bot_buffs", "bot_command_settings",
        "bot_create_combinations", "bot_data", "bot_heal_rotation_members",
        "bot_heal_rotation_targets", "bot_heal_rotations",
        "bot_inspect_messages", "bot_inventories", "bot_owner_options",
        "bot_pet_buffs", "bot_pet_inventories", "bot_pets", "bot_settings",
        "bot_spell_casting_chances", "bot_spell_settings",
        "bot_spells_entries", "bot_stances", "bot_timers",
    ],
    "Merc": [
        "merc_armorinfo", "merc_inventory", "merc_merchant_entries",
        "merc_merchant_template_entries", "merc_merchant_templates",
        "merc_name_types", "merc_npc_types", "merc_spell_list_entries",
        "merc_spell_lists", "merc_stance_entries", "merc_stats",
        "merc_subtypes", "merc_templates", "merc_types", "merc_weaponinfo",
    ],
}


def get_mysql_schema(conn):
    """Get all tables and their columns from MariaDB."""
    schema = {}
    with conn.cursor() as cur:
        cur.execute(
            "SELECT table_name FROM information_schema.tables "
            "WHERE table_schema = %s AND table_type = 'BASE TABLE'",
            (MYSQL_CONFIG["database"],),
        )
        tables = [row[0] for row in cur.fetchall()]

        for table in tables:
            cur.execute(
                "SELECT column_name, column_type, is_nullable, column_default, "
                "column_key, extra, ordinal_position, data_type "
                "FROM information_schema.columns "
                "WHERE table_schema = %s AND table_name = %s "
                "ORDER BY ordinal_position",
                (MYSQL_CONFIG["database"], table),
            )
            columns = {}
            for row in cur.fetchall():
                columns[row[0]] = {
                    "name": row[0],
                    "column_type": row[1],
                    "nullable": row[2] == "YES",
                    "default": row[3],
                    "key": row[4],
                    "extra": row[5],
                    "position": row[6],
                    "data_type": row[7],
                }
            schema[table] = columns

    # Get primary keys
    with conn.cursor() as cur:
        for table in tables:
            cur.execute(
                "SELECT column_name FROM information_schema.key_column_usage "
                "WHERE table_schema = %s AND table_name = %s "
                "AND constraint_name = 'PRIMARY' ORDER BY ordinal_position",
                (MYSQL_CONFIG["database"], table),
            )
            pk_cols = [row[0] for row in cur.fetchall()]
            for col in pk_cols:
                if col in schema[table]:
                    schema[table][col]["is_pk"] = True

    return schema


def get_pg_schema(conn):
    """Get all tables and their columns from PostgreSQL."""
    schema = {}
    with conn.cursor() as cur:
        cur.execute(
            "SELECT table_name FROM information_schema.tables "
            "WHERE table_schema = 'public' AND table_type = 'BASE TABLE'"
        )
        tables = [row[0] for row in cur.fetchall()]

        for table in tables:
            cur.execute(
                "SELECT column_name, data_type, is_nullable, column_default, "
                "ordinal_position, character_maximum_length, numeric_precision "
                "FROM information_schema.columns "
                "WHERE table_schema = 'public' AND table_name = %s "
                "ORDER BY ordinal_position",
                (table,),
            )
            columns = {}
            for row in cur.fetchall():
                columns[row[0]] = {
                    "name": row[0],
                    "data_type": row[1],
                    "nullable": row[2] == "YES",
                    "default": row[3],
                    "position": row[4],
                    "char_max_len": row[5],
                    "numeric_precision": row[6],
                }
            schema[table] = columns

    # Get primary keys
    with conn.cursor() as cur:
        for table in tables:
            cur.execute(
                "SELECT kcu.column_name "
                "FROM information_schema.table_constraints tc "
                "JOIN information_schema.key_column_usage kcu "
                "  ON tc.constraint_name = kcu.constraint_name "
                "  AND tc.table_schema = kcu.table_schema "
                "WHERE tc.constraint_type = 'PRIMARY KEY' "
                "  AND tc.table_schema = 'public' "
                "  AND tc.table_name = %s "
                "ORDER BY kcu.ordinal_position",
                (table,),
            )
            pk_cols = [row[0] for row in cur.fetchall()]
            for col in pk_cols:
                if col in schema[table]:
                    schema[table][col]["is_pk"] = True

    return schema


def normalize_mysql_type(data_type):
    """Normalize a MySQL data type to its PostgreSQL equivalent base type."""
    dt = data_type.lower().strip()
    return TYPE_MAP.get(dt, dt)


def types_compatible(mysql_col, pg_col):
    """Check if MySQL and PostgreSQL column types are compatible."""
    mysql_base = normalize_mysql_type(mysql_col["data_type"])
    pg_type = pg_col["data_type"].lower()

    # Direct match
    if mysql_base == pg_type:
        return True

    # integer vs smallint (tinyint -> smallint, but some PG tables use integer)
    int_types = {"smallint", "integer", "bigint"}
    if mysql_base in int_types and pg_type in int_types:
        # PG type must be >= MySQL type
        int_order = ["smallint", "integer", "bigint"]
        return int_order.index(pg_type) >= int_order.index(mysql_base)

    # SERIAL types show as "integer" in information_schema
    if mysql_base in ("integer", "smallint", "bigint") and pg_type in ("integer", "smallint", "bigint"):
        return True

    # varchar/char -> character varying
    if mysql_base in ("character varying", "character") and pg_type in ("character varying", "character", "text"):
        return True

    # text types are all text
    if mysql_base == "text" and pg_type == "text":
        return True

    # numeric/decimal
    if mysql_base == "numeric" and pg_type == "numeric":
        return True

    # float -> real
    if mysql_base == "real" and pg_type in ("real", "double precision"):
        return True

    # double -> double precision
    if mysql_base == "double precision" and pg_type in ("double precision", "real"):
        return True

    # timestamp
    if mysql_base in ("timestamp without time zone",) and pg_type in ("timestamp without time zone", "timestamp with time zone"):
        return True

    # date/time
    if mysql_base == pg_type:
        return True

    # bytea for blobs
    if mysql_base == "bytea" and pg_type == "bytea":
        return True

    # text for SET/ENUM
    if mysql_base in ("text", "character varying") and pg_type in ("text", "character varying"):
        return True

    # jsonb
    if mysql_base == "jsonb" and pg_type == "jsonb":
        return True

    # USER-DEFINED (e.g., enum types in PG)
    if pg_type == "user-defined":
        return True

    return False


def get_category(table_name):
    """Get the database_schema.h category for a table."""
    for cat, tables in SCHEMA_CATEGORIES.items():
        if table_name in tables:
            return cat
    return "Uncategorized"


def mysql_to_pg_type(col):
    """Convert a MySQL column definition to PostgreSQL type string."""
    ct = col["column_type"].lower()
    dt = col["data_type"].lower()
    extra = col.get("extra", "")

    # Auto-increment -> SERIAL
    if "auto_increment" in extra:
        if dt in ("bigint",):
            return "BIGSERIAL"
        return "SERIAL"

    # ENUM -> VARCHAR(255)
    if dt == "enum":
        return "VARCHAR(255)"

    # SET -> TEXT
    if dt == "set":
        return "TEXT"

    # Specific type mappings with size preservation
    if dt == "tinyint":
        return "SMALLINT"
    if dt == "smallint":
        return "SMALLINT"
    if dt == "mediumint":
        return "INTEGER"
    if dt == "int":
        return "INTEGER"
    if dt == "bigint":
        return "BIGINT"
    if dt == "float":
        return "REAL"
    if dt == "double":
        return "DOUBLE PRECISION"

    if dt == "decimal":
        # Extract precision from column_type like decimal(10,2)
        if "(" in ct:
            prec = ct.split("(")[1].rstrip(")")
            # Remove "unsigned"
            prec = prec.replace(" unsigned", "")
            return f"NUMERIC({prec})"
        return "NUMERIC"

    if dt in ("varchar", "char"):
        # Extract length
        if "(" in ct:
            length = ct.split("(")[1].split(")")[0]
            if dt == "varchar":
                return f"VARCHAR({length})"
            return f"CHAR({length})"
        return "VARCHAR(255)"

    if dt in ("tinytext", "text", "mediumtext", "longtext"):
        return "TEXT"

    if dt in ("tinyblob", "blob", "mediumblob", "longblob", "binary", "varbinary"):
        return "BYTEA"

    if dt == "datetime":
        return "TIMESTAMP"
    if dt == "timestamp":
        return "TIMESTAMP"
    if dt == "date":
        return "DATE"
    if dt == "time":
        return "TIME"
    if dt == "year":
        return "SMALLINT"

    if dt == "bit":
        if "(" in ct:
            length = ct.split("(")[1].split(")")[0]
            return f"BIT({length})"
        return "BIT(1)"

    if dt == "json":
        return "JSONB"

    return ct.upper()


def mysql_default_to_pg(col):
    """Convert MySQL default value to PostgreSQL syntax."""
    default = col.get("default")
    dt = col["data_type"].lower()

    if default is None:
        return ""

    # CURRENT_TIMESTAMP
    if default in ("CURRENT_TIMESTAMP", "current_timestamp()"):
        return " DEFAULT CURRENT_TIMESTAMP"

    # String defaults
    if dt in ("varchar", "char", "text", "tinytext", "mediumtext", "longtext", "enum", "set"):
        escaped = default.replace("'", "''")
        return f" DEFAULT '{escaped}'"

    # Numeric defaults
    if dt in ("tinyint", "smallint", "mediumint", "int", "bigint", "float", "double", "decimal"):
        return f" DEFAULT {default}"

    # Binary/blob -> no default
    if dt in ("blob", "tinyblob", "mediumblob", "longblob", "binary", "varbinary"):
        return ""

    return f" DEFAULT '{default}'"


def generate_create_table(table_name, mysql_cols):
    """Generate PostgreSQL CREATE TABLE DDL from MySQL column info."""
    lines = []
    pk_cols = []
    has_serial = False

    for col_name, col in sorted(mysql_cols.items(), key=lambda x: x[1]["position"]):
        pg_type = mysql_to_pg_type(col)
        nullable = "" if col["nullable"] else " NOT NULL"

        # SERIAL implies NOT NULL, don't duplicate
        if pg_type in ("SERIAL", "BIGSERIAL"):
            has_serial = True
            nullable = ""
            default = ""
        else:
            default = mysql_default_to_pg(col)

        # Quote reserved words
        reserved = {"end", "group", "order", "key", "range", "rank", "class",
                     "int", "interval", "check", "default", "index", "comment",
                     "type", "level", "zone", "name", "slot", "value", "rule"}
        col_ref = f'"{col_name}"' if col_name.lower() in reserved else col_name

        lines.append(f"    {col_ref} {pg_type}{nullable}{default}")

        if col.get("is_pk"):
            pk_cols.append(col_ref)

    if pk_cols:
        lines.append(f"    PRIMARY KEY ({', '.join(pk_cols)})")

    col_count = len(mysql_cols)
    ddl = f"-- EQEmu: {table_name} ({col_count} columns). Created for 100% MariaDB parity.\n"
    ddl += f"CREATE TABLE IF NOT EXISTS {table_name} (\n"
    ddl += ",\n".join(lines)
    ddl += "\n);\n"

    return ddl


def main():
    generate_ddl = "--generate-ddl" in sys.argv

    print("=" * 70)
    print("PostgreSQL vs MariaDB Schema Comparison")
    print("=" * 70)
    print()

    # Connect to both databases
    print("Connecting to MariaDB (akk-stack)...", end=" ")
    mysql_conn = pymysql.connect(**MYSQL_CONFIG)
    print("OK")

    print("Connecting to PostgreSQL (ADIF)...", end=" ")
    pg_conn = psycopg2.connect(**PG_CONFIG)
    print("OK")
    print()

    # Get schemas
    print("Reading MariaDB schema...", end=" ")
    mysql_schema = get_mysql_schema(mysql_conn)
    print(f"{len(mysql_schema)} tables")

    print("Reading PostgreSQL schema...", end=" ")
    pg_schema = get_pg_schema(pg_conn)
    print(f"{len(pg_schema)} tables")
    print()

    # Compare
    missing_tables = []
    column_issues = []  # (table, issue_type, details)
    matched_tables = []
    pg_only_tables = []

    for table in sorted(mysql_schema.keys()):
        if table not in pg_schema:
            missing_tables.append(table)
            continue

        # Table exists - compare columns
        mysql_cols = mysql_schema[table]
        pg_cols = pg_schema[table]
        table_issues = []

        # Build case-insensitive lookup for PG columns
        pg_cols_lower = {k.lower(): v for k, v in pg_cols.items()}

        # Intentional renames (PG reserved words or design improvements)
        known_renames = {
            ("base_data", "end"): "endurance",
            ("base_data", "end_regen"): "endurance_regen",
            ("base_data", "end_fac"): "endurance_fac",
        }

        for col_name, mysql_col in mysql_cols.items():
            col_lower = col_name.lower()
            # Check for intentional renames
            rename_key = (table, col_lower)
            if rename_key in known_renames:
                pg_name = known_renames[rename_key]
                if pg_name in pg_cols_lower:
                    continue  # Intentional rename, not a gap

            if col_lower in pg_cols_lower:
                pg_col = pg_cols_lower[col_lower]
                if col_name != pg_col["name"]:
                    # Case difference only — not a real issue for PG
                    pass
                if not types_compatible(mysql_col, pg_col):
                    table_issues.append((
                        "TYPE_MISMATCH", col_name,
                        f"MySQL: {mysql_col['column_type']} -> PG: {pg_col['data_type']}"
                    ))
            else:
                table_issues.append(("MISSING_COLUMN", col_name, mysql_col["column_type"]))

        # Check for PK mismatches (case-insensitive)
        mysql_pk = sorted([c.lower() for c, v in mysql_cols.items() if v.get("is_pk")])
        pg_pk = sorted([c.lower() for c, v in pg_cols.items() if v.get("is_pk")])
        if mysql_pk != pg_pk:
            table_issues.append(("PK_MISMATCH", "", f"MySQL: {mysql_pk} vs PG: {pg_pk}"))

        # Extra columns in PG (case-insensitive check)
        mysql_cols_lower = {k.lower() for k in mysql_cols.keys()}
        for col_name in pg_cols:
            if col_name.lower() not in mysql_cols_lower:
                table_issues.append(("EXTRA_COLUMN", col_name, pg_cols[col_name]["data_type"]))

        if table_issues:
            column_issues.append((table, table_issues))
        else:
            matched_tables.append(table)

    # PG-only tables
    for table in sorted(pg_schema.keys()):
        if table not in mysql_schema:
            pg_only_tables.append(table)

    # --- Report ---
    print("=" * 70)
    print("SUMMARY")
    print("=" * 70)
    print(f"  MariaDB tables:        {len(mysql_schema)}")
    print(f"  PostgreSQL tables:     {len(pg_schema)}")
    print(f"  Perfect matches:       {len(matched_tables)}")
    print(f"  Missing from PG:       {len(missing_tables)}")
    print(f"  Column mismatches:     {len(column_issues)}")
    print(f"  PG-only tables:        {len(pg_only_tables)}")
    print()

    # Missing tables by category
    if missing_tables:
        print("=" * 70)
        print(f"MISSING TABLES ({len(missing_tables)})")
        print("=" * 70)
        by_cat = defaultdict(list)
        for t in missing_tables:
            by_cat[get_category(t)].append(t)
        for cat in ["Character", "Content", "Server", "State", "Login", "Version", "Bot", "Merc", "Uncategorized"]:
            if cat in by_cat:
                print(f"\n  [{cat}] ({len(by_cat[cat])} tables)")
                for t in sorted(by_cat[cat]):
                    col_count = len(mysql_schema[t])
                    print(f"    - {t} ({col_count} cols)")
        print()

    # Column mismatches
    if column_issues:
        print("=" * 70)
        print(f"COLUMN MISMATCHES ({len(column_issues)} tables)")
        print("=" * 70)
        for table, issues in sorted(column_issues):
            cat = get_category(table)
            missing = [i for i in issues if i[0] == "MISSING_COLUMN"]
            types = [i for i in issues if i[0] == "TYPE_MISMATCH"]
            pks = [i for i in issues if i[0] == "PK_MISMATCH"]
            extras = [i for i in issues if i[0] == "EXTRA_COLUMN"]

            print(f"\n  {table} [{cat}]")
            mysql_count = len(mysql_schema[table])
            pg_count = len(pg_schema[table])
            print(f"    Columns: MySQL={mysql_count}, PG={pg_count}")

            for issue_type, col, detail in issues:
                if issue_type == "MISSING_COLUMN":
                    print(f"    MISSING: {col} ({detail})")
                elif issue_type == "TYPE_MISMATCH":
                    print(f"    TYPE:    {col} — {detail}")
                elif issue_type == "PK_MISMATCH":
                    print(f"    PK:      {detail}")
                elif issue_type == "EXTRA_COLUMN":
                    print(f"    EXTRA:   {col} ({detail}) [PG only]")
        print()

    # Perfect matches
    if matched_tables:
        print("=" * 70)
        print(f"PERFECT MATCHES ({len(matched_tables)} tables)")
        print("=" * 70)
        for t in matched_tables:
            print(f"    {t} ({len(mysql_schema[t])} cols)")
        print()

    # PG-only tables
    if pg_only_tables:
        print("=" * 70)
        print(f"PG-ONLY TABLES ({len(pg_only_tables)}) — not in MariaDB")
        print("=" * 70)
        for t in pg_only_tables:
            print(f"    {t} ({len(pg_schema[t])} cols)")
        print()

    # Generate DDL for missing tables
    if generate_ddl and missing_tables:
        print("=" * 70)
        print("GENERATED DDL FOR MISSING TABLES")
        print("=" * 70)
        print()
        for table in sorted(missing_tables):
            ddl = generate_create_table(table, mysql_schema[table])
            print(ddl)

    # Separate real issues from extra-PG-column-only tables
    real_issues = []
    extra_only = []
    for table, issues in column_issues:
        has_missing = any(i[0] == "MISSING_COLUMN" for i in issues)
        has_type = any(i[0] == "TYPE_MISMATCH" for i in issues)
        if has_missing or has_type:
            real_issues.append((table, issues))
        else:
            extra_only.append((table, issues))

    # Scorecard
    total = len(mysql_schema)
    done = len(matched_tables) + len(extra_only)  # Extra PG columns don't break parity
    pct = (done / total * 100) if total else 0
    print("=" * 70)
    print(f"PARITY SCORE: {done}/{total} ({pct:.1f}%)")
    print(f"  Perfect matches:         {len(matched_tables)}")
    print(f"  Extra PG cols only:      {len(extra_only)} (not breaking)")
    print(f"  Missing tables:          {len(missing_tables)}")
    print(f"  Real column issues:      {len(real_issues)}")
    remaining = len(missing_tables) + len(real_issues)
    if remaining == 0:
        print("  STATUS: 100% PARITY ACHIEVED")
    else:
        print(f"  REMAINING: {remaining} tables need work")
    print("=" * 70)

    mysql_conn.close()
    pg_conn.close()


if __name__ == "__main__":
    main()
