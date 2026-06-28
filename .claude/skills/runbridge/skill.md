---
name: runbridge
description: >
  Start the ADIF protocol bridge in a visible CMD window with trace logging.
  Kills any existing bridge process first, then launches scripts/StartBridge.bat.
user_invocable: true
---

# /runbridge — Start the Protocol Bridge

Launch the EQ protocol bridge in a new CMD window so trace output is visible.

## Execution

1. Kill any running `adif-bridge.exe` process (clean restart):

```powershell
try { Stop-Process -Name "adif-bridge" -Force -ErrorAction Stop } catch {}
```

2. Launch `scripts/StartBridge.bat` in a new CMD window:

```powershell
Start-Process cmd -ArgumentList '/k', 'title ADIF-Bridge && E:\development\adif\scripts\StartBridge.bat'
```

3. Report: "Bridge starting in CMD window — watch for 'UDP listeners bound' before connecting the client."
