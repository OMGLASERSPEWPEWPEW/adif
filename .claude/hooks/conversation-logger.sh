#!/bin/bash
# =============================================================================
# conversation-logger — Stop hook
# Parses the Claude transcript and logs conversation excerpts to a structured
# memory directory. Entries are written to both a daily chronological log and
# topic-specific "heap" files based on keyword matching.
#
# Hook type: Stop
# Lifecycle: Runs after every agent response
# Requires: python
# =============================================================================

hook_data=$(cat)

python - "$hook_data" << 'PYTHON'
import json, sys, os, re
from datetime import datetime

# === CONFIGURATION ===
LOG_DIR = ".claude/memory"
MAX_SNIPPETS = 50
MAX_SNIPPET_LENGTH = 8000

CONCEPTS = {
    "Architecture": r"module|import|export|build|typescript|webpack|vite",
    "Testing": r"test|spec|assert|mock|fixture|coverage",
    "Database": r"database|sql|query|migration|schema|index",
    "API": r"api|endpoint|route|request|response|middleware",
    "Auth": r"auth|login|session|token|oauth|permission",
    "UI": r"component|render|style|layout|responsive|animation",
    "DevOps": r"deploy|ci|cd|docker|container|pipeline",
    "Docs": r"readme|documentation|comment|changelog|guide",
}
# === END CONFIGURATION ===

SYSTEM_REMINDER_RE = re.compile(r"<system-reminder>.*?</system-reminder>", re.DOTALL)

def strip_system_reminders(text):
    return SYSTEM_REMINDER_RE.sub("", text).strip()

hook_data_raw = sys.argv[1] if len(sys.argv) > 1 else "{}"
try:
    hook = json.loads(hook_data_raw)
    transcript_path = hook.get("transcript_path", "")
    session_id = hook.get("session_id", "unknown")
except Exception:
    transcript_path = ""
    session_id = "unknown"

if not transcript_path or not os.path.exists(transcript_path):
    print('{"continue": true}')
    sys.exit(0)

project_dir = os.environ.get("CLAUDE_PROJECT_DIR", "")
if not project_dir:
    project_dir = os.path.dirname(
        os.path.dirname(os.path.dirname(os.path.dirname(transcript_path)))
    )

log_base = os.path.join(project_dir, LOG_DIR)
heap_dir = os.path.join(log_base, "heaps")
daily_dir = os.path.join(log_base, "daily")

os.makedirs(heap_dir, exist_ok=True)
os.makedirs(daily_dir, exist_ok=True)

now = datetime.now()
timestamp = now.strftime("%Y-%m-%d_%H-%M-%S")
date_str = now.strftime("%Y-%m-%d")

state_file = f"/tmp/claude-logger-{session_id}.json"
last_line = 0
try:
    if os.path.exists(state_file):
        with open(state_file, "r") as f:
            state = json.load(f)
            last_line = state.get("last_line", 0)
except Exception:
    pass

snippets = []
pending_tools = []
current_line = 0

def flush_pending_tools():
    """Collapse consecutive tool-only assistant turns into one line."""
    if pending_tools:
        names = ", ".join(pending_tools)
        snippets.append(f"**assistant**: [tools: {names}]")
        pending_tools.clear()

try:
    with open(transcript_path, "r") as f:
        for line in f:
            current_line += 1
            if current_line <= last_line:
                continue
            try:
                entry = json.loads(line.strip())
                msg = entry.get("message", {})
                if not isinstance(msg, dict):
                    continue
                role = msg.get("role", "")
                if role not in ("user", "assistant"):
                    continue
                content = msg.get("content", "")

                if isinstance(content, list):
                    text_parts = []
                    tool_names = []
                    for block in content:
                        if isinstance(block, dict):
                            if block.get("type") == "text":
                                text_parts.append(block.get("text", ""))
                            elif block.get("type") == "tool_use":
                                tool_names.append(block.get("name", "?"))
                        elif isinstance(block, str):
                            text_parts.append(block)

                    text_content = " ".join(t for t in text_parts if t.strip())

                    if role == "user":
                        flush_pending_tools()
                        text_content = strip_system_reminders(text_content)
                        if text_content:
                            if len(text_content) > MAX_SNIPPET_LENGTH:
                                text_content = text_content[:MAX_SNIPPET_LENGTH] + "..."
                            snippets.append(f"**user**: {text_content}")

                    elif role == "assistant":
                        if text_content.strip():
                            flush_pending_tools()
                            if tool_names:
                                text_content += f" [tools: {', '.join(tool_names)}]"
                            if len(text_content) > MAX_SNIPPET_LENGTH:
                                text_content = text_content[:MAX_SNIPPET_LENGTH] + "..."
                            snippets.append(f"**assistant**: {text_content}")
                        elif tool_names:
                            pending_tools.extend(tool_names)
                        # else: empty assistant message, skip

                elif isinstance(content, str) and content.strip():
                    flush_pending_tools()
                    if role == "user":
                        content = strip_system_reminders(content)
                    if content:
                        if len(content) > MAX_SNIPPET_LENGTH:
                            content = content[:MAX_SNIPPET_LENGTH] + "..."
                        snippets.append(f"**{role}**: {content.strip()}")

            except Exception:
                continue

    flush_pending_tools()

except Exception:
    print('{"continue": true}')
    sys.exit(0)

try:
    with open(state_file, "w") as f:
        json.dump({"last_line": current_line}, f)
except Exception:
    pass

if not snippets:
    print('{"continue": true}')
    sys.exit(0)

# --- Daily log ---
transcript = "\n\n".join(snippets[-MAX_SNIPPETS:])

daily_log = os.path.join(daily_dir, f"{date_str}.md")
try:
    with open(daily_log, "a") as f:
        f.write(f"---\n")
        f.write(f"## Session {session_id[:8]} -- {timestamp}\n\n")
        f.write(transcript)
        f.write(f"\n\n")
except Exception:
    pass

# --- Topic heaps ---
for concept_name, pattern in CONCEPTS.items():
    if re.search(pattern, transcript, re.IGNORECASE):
        heap_file = os.path.join(heap_dir, f"{concept_name}.md")
        try:
            summary = "\n\n".join(snippets[-20:])
            if len(summary) > 4000:
                summary = summary[:4000] + "..."
            with open(heap_file, "a") as f:
                f.write(f"## {timestamp} (session {session_id[:8]})\n\n")
                f.write(summary)
                f.write(f"\n\n---\n\n")
        except Exception:
            pass

print('{"continue": true}')
PYTHON
