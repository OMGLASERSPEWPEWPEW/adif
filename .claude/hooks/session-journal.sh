#!/bin/bash
# =============================================================================
# session-journal — Multi-lifecycle hook
# Stamps a per-session journal in .claude/journals/<session_id>.md tracking
# tool usage, state transitions, session lifecycle events, AND conversation
# content (user messages are captured on Stop from the transcript).
#
# Hook type: PreToolUse, PostToolUse, Notification, SessionStart, Stop
# Lifecycle: Runs on every lifecycle event
# Requires: python
# =============================================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$PROJECT_DIR" 2>/dev/null || true

hook_data=$(cat)

python - "$hook_data" << 'PYTHON'
import json, os, re, sys
from datetime import datetime

SYSTEM_REMINDER_RE = re.compile(r"<system-reminder>.*?</system-reminder>", re.DOTALL)
MAX_USER_MSG = 4000

raw = sys.argv[1] if len(sys.argv) > 1 else "{}"
try:
    hook = json.loads(raw)
except Exception:
    hook = {}

event = hook.get("hook_event_name") or "unknown"

def emit():
    if event == "Stop":
        print(json.dumps({"continue": True}))
    elif event == "PreToolUse":
        print(json.dumps({"decision": "allow"}))

def extract_text(content):
    if isinstance(content, str):
        return content.strip()
    if isinstance(content, list):
        parts = []
        for block in content:
            if isinstance(block, dict):
                if block.get("type") == "text":
                    parts.append(block.get("text", ""))
                elif block.get("type") == "tool_result":
                    pass
            elif isinstance(block, str):
                parts.append(block)
        return " ".join(p for p in parts if p.strip())
    return ""

def capture_conversation(transcript_path, journal_path, state_key):
    if not transcript_path or not os.path.exists(transcript_path):
        return
    state_file = f"/tmp/claude-journal-{state_key}.json"
    last_line = 0
    try:
        if os.path.exists(state_file):
            with open(state_file, "r") as f:
                last_line = json.load(f).get("last_line", 0)
    except Exception:
        pass

    messages = []
    current_line = 0
    try:
        with open(transcript_path, "r", encoding="utf-8") as f:
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
                    content = msg.get("content", "")
                    text = extract_text(content)
                    text = SYSTEM_REMINDER_RE.sub("", text).strip()
                    if not text:
                        continue
                    if role == "user" and len(text) > 20:
                        if len(text) > MAX_USER_MSG:
                            text = text[:MAX_USER_MSG] + "..."
                        messages.append(("user", text))
                    elif role == "assistant" and len(text) > 20:
                        if len(text) > MAX_USER_MSG:
                            text = text[:MAX_USER_MSG] + "..."
                        messages.append(("assistant", text))
                except Exception:
                    continue
    except Exception:
        return

    try:
        with open(state_file, "w") as f:
            json.dump({"last_line": current_line}, f)
    except Exception:
        pass

    if not messages:
        return
    now = datetime.now().isoformat(timespec="seconds")
    try:
        with open(journal_path, "a") as f:
            f.write(f"\n### Conversation ({now})\n")
            for role, text in messages:
                prefix = "USER" if role == "user" else "CLAUDE"
                f.write(f"\n**{prefix}**: {text}\n")
            f.write("\n---\n")
    except Exception:
        pass

try:
    session_id = hook.get("session_id") or "unknown"
    tool_name = hook.get("tool_name") or ""
    cwd = hook.get("cwd") or os.getcwd()
    transcript = hook.get("transcript_path") or ""

    safe_session = re.sub(r"[^A-Za-z0-9_-]", "_", str(session_id))[:64]
    journals_dir = os.path.join(cwd, ".claude", "journals")
    os.makedirs(journals_dir, exist_ok=True)
    path = os.path.join(journals_dir, f"{safe_session}.md")

    now = datetime.now().isoformat(timespec="seconds")

    if event == "SessionStart":
        state = "starting"
    elif event == "PreToolUse":
        state = f"awaiting:{tool_name}" if tool_name else "awaiting"
    elif event == "PostToolUse":
        state = "running"
    elif event == "Notification":
        state = "notify"
    elif event == "Stop":
        state = "idle"
        capture_conversation(transcript, path, safe_session)
    elif event == "SessionEnd":
        state = "ended"
    else:
        state = event.lower()

    line = f"- {now} [{event}] state={state}"
    if tool_name:
        line += f" tool={tool_name}"
    if transcript:
        line += f" transcript={os.path.basename(transcript)}"

    header = ""
    if not os.path.exists(path):
        header = (
            f"---\n"
            f"session_id: {session_id}\n"
            f"pid: {os.getpid()}\n"
            f"cwd: {cwd}\n"
            f"started_at: {now}\n"
            f"---\n\n"
        )

    with open(path, "a") as f:
        if header:
            f.write(header)
        f.write(line + "\n")
except Exception:
    pass

emit()
PYTHON
