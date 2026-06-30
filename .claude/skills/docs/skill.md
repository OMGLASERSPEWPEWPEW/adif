---
name: docs
description: >
  Start the ADIF docs server on port 5906 and open the docs index in the browser.
  Serves all HTML artifacts from docs/ for interactive viewing.
user_invocable: true
---

# /docs — Start the Docs Server

Launch the HTML docs server on port 5906 and open the browser.

## Execution

1. Check if the docs server is already running:

```bash
curl -s http://localhost:5906/ >/dev/null 2>&1 && echo "running" || echo "not running"
```

2. If not running, start it in a new CMD window:

```powershell
Start-Process cmd -ArgumentList '/k', 'title ADIF-Docs && python E:\development\adif\scripts\docs-server.py'
```

3. Open the docs index in the browser:

```powershell
Start-Process "http://localhost:5906/"
```

4. Report: "Docs server running at http://localhost:5906/"

## Available Pages

| Page | Description |
|------|-------------|
| `index.html` | Navigation hub |
| `zone-entry-comparison.html` | EQEmu vs ADIF bridge packet comparison |
| `eq-world-protocol.html` | EQ protocol reference (5 tabs) |
| `how-eq-works.html` | EQ architecture overview |
| `adif-roadmap.html` | ADIF roadmap and milestones |
| `struct-proto-map.html` | EQ struct to protobuf field mapping |
| `rust-server.html` | Rust zone server status |
| `tech-comparison.html` | EQ vs ADIF tech comparison |
| `opcode-audit.html` | Opcode audit and modernization |
| `postgresql-postmortem.html` | PostgreSQL migration postmortem |
