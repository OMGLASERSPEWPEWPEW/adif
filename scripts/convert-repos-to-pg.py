#!/usr/bin/env python3
"""
Convert EQEmu base repository files from MySQL to native PostgreSQL SQL.

Handles all 250 auto-generated base/*.h files:
  1. REPLACE INTO  -> INSERT INTO ... ON CONFLICT DO UPDATE
  2. FROM_UNIXTIME -> TO_TIMESTAMP
  3. UNIX_TIMESTAMP -> EXTRACT(EPOCH FROM ...)::int
  4. Backtick quoting -> PostgreSQL double-quote quoting
"""

import os
import re
import sys
import glob

BASE_DIR = os.path.join(
    os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
    "reference", "eqemu-server", "common", "repositories", "base"
)

DRY_RUN = "--dry-run" in sys.argv
VERBOSE = "--verbose" in sys.argv or "-v" in sys.argv

stats = {
    "files_processed": 0,
    "files_modified": 0,
    "replace_into": 0,
    "from_unixtime": 0,
    "unix_timestamp": 0,
    "backticks": 0,
    "on_conflict_added": 0,
    "upsert_set_added": 0,
    "skipped_no_replace": 0,
}

UPSERT_SET_METHOD = '''
\tstatic std::string BaseUpsertSet()
\t{
\t\tstd::vector<std::string> set_parts;
\t\tfor (const auto &col : Columns()) {
\t\t\tif (col != PrimaryKey()) {
\t\t\t\tset_parts.push_back(fmt::format("{0} = EXCLUDED.{0}", col));
\t\t\t}
\t\t}
\t\treturn Strings::Implode(", ", set_parts);
\t}
'''


def convert_file(filepath):
    with open(filepath, "r", encoding="utf-8") as f:
        content = f.read()

    original = content

    # ---- 1. REPLACE INTO -> INSERT INTO (in BaseReplace method) ----
    content = content.replace(
        '"REPLACE INTO {} ({}) "',
        '"INSERT INTO {} ({}) "'
    )
    if '"REPLACE INTO {} ({}) "' in original:
        stats["replace_into"] += 1

    # ---- 2. Rename BaseReplace() -> BaseUpsert() ----
    content = content.replace("BaseReplace()", "BaseUpsert()")

    # ---- 3. FROM_UNIXTIME -> TO_TIMESTAMP ----
    count = content.count("FROM_UNIXTIME(")
    if count > 0:
        content = content.replace("FROM_UNIXTIME(", "TO_TIMESTAMP(")
        stats["from_unixtime"] += count

    # ---- 4. UNIX_TIMESTAMP(col) -> EXTRACT(EPOCH FROM col)::int ----
    (content, n) = re.subn(
        r'"UNIX_TIMESTAMP\((\w+)\)"',
        r'"EXTRACT(EPOCH FROM \1)::int"',
        content
    )
    stats["unix_timestamp"] += n

    # Also handle UNIX_TIMESTAMP() with no argument (rare in base repos)
    (content, n2) = re.subn(
        r'"UNIX_TIMESTAMP\(\)"',
        r'"EXTRACT(EPOCH FROM NOW())::int"',
        content
    )
    stats["unix_timestamp"] += n2

    # ---- 5. Backtick quoting -> double-quote quoting ----
    # MySQL: `class`  ->  PostgreSQL: "class"
    # In C++ string: "`class`"  ->  "\"class\""
    (content, n) = re.subn(
        r'"`(\w+)`"',
        r'"\\"\1\\""',
        content
    )
    stats["backticks"] += n

    # ---- 6. Add BaseUpsertSet() method after BaseUpsert() ----
    # Match the exact method text — identical in all auto-generated files.
    base_upsert_exact = (
        '\tstatic std::string BaseUpsert()\n'
        '\t{\n'
        '\t\treturn fmt::format(\n'
        '\t\t\t"INSERT INTO {} ({}) ",\n'
        '\t\t\tTableName(),\n'
        '\t\t\tColumnsRaw()\n'
        '\t\t);\n'
        '\t}'
    )
    if "BaseUpsertSet()" not in content and base_upsert_exact in content:
        insert_pos = content.find(base_upsert_exact) + len(base_upsert_exact)
        content = content[:insert_pos] + UPSERT_SET_METHOD + content[insert_pos:]
        stats["upsert_set_added"] += 1

    # ---- 7. Add ON CONFLICT to ReplaceOne format string ----
    # Pattern: "{} VALUES ({})" with BaseUpsert() — this is ReplaceOne
    old_replace_one = (
        '"{} VALUES ({})"\n'
        '\t\t\t\tBaseUpsert(),\n'
        '\t\t\t\tStrings::Implode(",", v)'
    )
    # Try alternate spacing: sometimes there's a comma right after v
    old_replace_one_alt = (
        '"{} VALUES ({})",\n'
        '\t\t\t\tBaseUpsert(),\n'
        '\t\t\t\tStrings::Implode(",", v)'
    )

    new_replace_one = (
        '"{} VALUES ({}) ON CONFLICT ({}) DO UPDATE SET {}",\n'
        '\t\t\t\tBaseUpsert(),\n'
        '\t\t\t\tStrings::Implode(",", v),\n'
        '\t\t\t\tPrimaryKey(),\n'
        '\t\t\t\tBaseUpsertSet()'
    )

    if old_replace_one in content:
        content = content.replace(old_replace_one, new_replace_one, 1)
        stats["on_conflict_added"] += 1
    elif old_replace_one_alt in content:
        content = content.replace(old_replace_one_alt, new_replace_one, 1)
        stats["on_conflict_added"] += 1

    # ---- 8. Add ON CONFLICT to ReplaceMany format string ----
    # Pattern: "{} VALUES {}" with BaseUpsert() — this is ReplaceMany
    old_replace_many = (
        '"{} VALUES {}"\n'
        '\t\t\t\tBaseUpsert(),\n'
        '\t\t\t\tStrings::Implode(",", insert_chunks)'
    )
    old_replace_many_alt = (
        '"{} VALUES {}",\n'
        '\t\t\t\tBaseUpsert(),\n'
        '\t\t\t\tStrings::Implode(",", insert_chunks)'
    )

    new_replace_many = (
        '"{} VALUES {} ON CONFLICT ({}) DO UPDATE SET {}",\n'
        '\t\t\t\tBaseUpsert(),\n'
        '\t\t\t\tStrings::Implode(",", insert_chunks),\n'
        '\t\t\t\tPrimaryKey(),\n'
        '\t\t\t\tBaseUpsertSet()'
    )

    if old_replace_many in content:
        content = content.replace(old_replace_many, new_replace_many, 1)
        stats["on_conflict_added"] += 1
    elif old_replace_many_alt in content:
        content = content.replace(old_replace_many_alt, new_replace_many, 1)
        stats["on_conflict_added"] += 1

    # ---- Write if changed ----
    if content != original:
        stats["files_modified"] += 1
        if not DRY_RUN:
            with open(filepath, "w", encoding="utf-8") as f:
                f.write(content)
        if VERBOSE:
            print(f"  MODIFIED: {os.path.basename(filepath)}")
    else:
        if VERBOSE:
            print(f"  unchanged: {os.path.basename(filepath)}")

    stats["files_processed"] += 1


def main():
    if not os.path.isdir(BASE_DIR):
        print(f"ERROR: Directory not found: {BASE_DIR}")
        sys.exit(1)

    files = sorted(glob.glob(os.path.join(BASE_DIR, "*.h")))
    print(f"{'[DRY RUN] ' if DRY_RUN else ''}Processing {len(files)} base repository files...")
    print(f"Directory: {BASE_DIR}\n")

    for filepath in files:
        convert_file(filepath)

    # ---- Validation pass (skip in dry-run since files weren't written) ----
    issues = []
    if not DRY_RUN:
        for filepath in files:
            with open(filepath, "r", encoding="utf-8") as f:
                content = f.read()

            name = os.path.basename(filepath)

            if "REPLACE INTO" in content:
                issues.append(f"  {name}: still contains REPLACE INTO")
            if "FROM_UNIXTIME(" in content:
                issues.append(f"  {name}: still contains FROM_UNIXTIME")
            if re.search(r'"UNIX_TIMESTAMP\(', content):
                issues.append(f"  {name}: still contains UNIX_TIMESTAMP in string")
            if re.search(r'"`\w+`"', content):
                issues.append(f"  {name}: still contains backtick-quoted identifiers")

            if "BaseReplace()" in content:
                issues.append(f"  {name}: still references BaseReplace()")

            has_upsert = "static std::string BaseUpsert()" in content
            has_upsert_set = "BaseUpsertSet()" in content
            has_on_conflict = "ON CONFLICT" in content

            if has_upsert and not has_upsert_set:
                issues.append(f"  {name}: has BaseUpsert but missing BaseUpsertSet")
            if has_upsert and not has_on_conflict:
                issues.append(f"  {name}: has BaseUpsert but missing ON CONFLICT")

    # ---- Report ----
    print("=" * 60)
    print("CONVERSION REPORT")
    print("=" * 60)
    print(f"Files processed:       {stats['files_processed']}")
    print(f"Files modified:        {stats['files_modified']}")
    print(f"REPLACE INTO -> INSERT INTO:  {stats['replace_into']}")
    print(f"FROM_UNIXTIME -> TO_TIMESTAMP: {stats['from_unixtime']}")
    print(f"UNIX_TIMESTAMP -> EXTRACT:     {stats['unix_timestamp']}")
    print(f"Backtick -> double-quote:      {stats['backticks']}")
    print(f"BaseUpsertSet() added:         {stats['upsert_set_added']}")
    print(f"ON CONFLICT clauses added:     {stats['on_conflict_added']}")

    if issues:
        print(f"\nWARNINGS ({len(issues)}):")
        for issue in issues:
            print(issue)
    else:
        print("\nNo issues found — all conversions clean!")

    if DRY_RUN:
        print("\n[DRY RUN] No files were modified. Run without --dry-run to apply.")


if __name__ == "__main__":
    main()
