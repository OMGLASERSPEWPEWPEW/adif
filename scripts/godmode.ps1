<#
.SYNOPSIS
  Starts the EQ reference server and configures Ghouldan as an invincible god.

.DESCRIPTION
  1. Starts the akk-stack Docker containers
  2. Waits for the server to be ready
  3. Flags Ghouldan's account as GM (status 250)
  4. Prints the GM commands to use in-game

.NOTES
  Run from the ADIF project root: .\scripts\godmode.ps1
  Requires Docker Desktop to be running.
#>

param(
    [string]$CharacterName = "Ghouldan",
    [int]$GMStatus = 250
)

$akkStack = Join-Path $PSScriptRoot "..\reference\akk-stack"

# ── 1. Check Docker ─────────────────────────────────────────────
Write-Host "`n=== ADIF God Mode Setup ===" -ForegroundColor Yellow
Write-Host ""

$dockerRunning = $false
try {
    $null = docker info 2>$null
    if ($LASTEXITCODE -eq 0) { $dockerRunning = $true }
} catch {}

if (-not $dockerRunning) {
    Write-Host "Docker Desktop is not running." -ForegroundColor Red
    Write-Host "Start Docker Desktop first, then re-run this script." -ForegroundColor Red
    exit 1
}

Write-Host "[OK] Docker is running" -ForegroundColor Green

# ── 2. Start akk-stack ──────────────────────────────────────────
Write-Host "`nStarting akk-stack server..." -ForegroundColor Cyan
Push-Location $akkStack
docker compose up -d 2>&1 | ForEach-Object { Write-Host "  $_" -ForegroundColor DarkGray }
Pop-Location

# ── 3. Wait for server to be ready ──────────────────────────────
Write-Host "`nWaiting for server to be ready..." -ForegroundColor Cyan
$maxWait = 120
$elapsed = 0
$ready = $false

while ($elapsed -lt $maxWait) {
    $status = docker compose -f "$akkStack\docker-compose.yml" ps --format json 2>$null
    if ($status -match '"running"') {
        $ready = $true
        break
    }
    Start-Sleep -Seconds 5
    $elapsed += 5
    Write-Host "  Waiting... ($elapsed`s)" -ForegroundColor DarkGray
}

if (-not $ready) {
    Write-Host "Server didn't start within $maxWait seconds." -ForegroundColor Red
    Write-Host "Check: docker compose -f reference/akk-stack/docker-compose.yml logs" -ForegroundColor Yellow
    exit 1
}

Write-Host "[OK] Server is running" -ForegroundColor Green

# ── 4. Flag account as GM ───────────────────────────────────────
Write-Host "`nFlagging $CharacterName as GM (status $GMStatus)..." -ForegroundColor Cyan

# Get the account name from the character name via the database
$flagCmd = @"
cd /home/eqemu/server && mysql -u eqemu -peqemu -D peq -e "
  UPDATE account SET status = $GMStatus
  WHERE id = (
    SELECT account_id FROM character_data WHERE name = '$CharacterName' LIMIT 1
  );
  SELECT a.name as account, a.status, c.name as character_name
  FROM account a
  JOIN character_data c ON c.account_id = a.id
  WHERE c.name = '$CharacterName';
"
"@

docker compose -f "$akkStack\docker-compose.yml" exec -T eqemu-server bash -c $flagCmd 2>&1 | ForEach-Object { Write-Host "  $_" -ForegroundColor DarkGray }

Write-Host "[OK] Account flagged" -ForegroundColor Green

# ── 5. Print in-game instructions ───────────────────────────────
Write-Host "`n=== YOU'RE SET ===" -ForegroundColor Yellow
Write-Host ""
Write-Host "Launch EQ with your desktop StartEQ shortcut and log in as $CharacterName." -ForegroundColor White
Write-Host ""
Write-Host "Once in-game, type these commands in the chat window:" -ForegroundColor White
Write-Host ""
Write-Host "  #set god_mode on" -ForegroundColor Green
Write-Host "    Invincible. Can't die. Fly mode. GM speed." -ForegroundColor DarkGray
Write-Host ""
Write-Host "  #set level 65" -ForegroundColor Green
Write-Host "    Max level instantly." -ForegroundColor DarkGray
Write-Host ""
Write-Host "  #damage 999999" -ForegroundColor Green
Write-Host "    One-shot kill your target." -ForegroundColor DarkGray
Write-Host ""
Write-Host "  #cleartimers all" -ForegroundColor Green
Write-Host "    Reset ALL cooldowns (Harm Touch, discs, spells, AAs)." -ForegroundColor DarkGray
Write-Host "    Run this after every Harm Touch to use it again immediately." -ForegroundColor DarkGray
Write-Host ""
Write-Host "  #cleartimers 5088" -ForegroundColor Green
Write-Host "    Reset just the Harm Touch cooldown (timer ID 5088)." -ForegroundColor DarkGray
Write-Host ""
Write-Host "  #castspell 88" -ForegroundColor Green
Write-Host "    Cast Harm Touch directly (spell ID 88)." -ForegroundColor DarkGray
Write-Host ""
Write-Host "Other useful commands:" -ForegroundColor White
Write-Host "  #goto zonename     - Teleport to any zone" -ForegroundColor DarkGray
Write-Host "  #summon playername - Summon a player to you" -ForegroundColor DarkGray
Write-Host "  #zone zoneid       - Zone to a specific zone by ID" -ForegroundColor DarkGray
Write-Host "  #set hp_full       - Full heal yourself" -ForegroundColor DarkGray
Write-Host "  #set mana_full     - Full mana" -ForegroundColor DarkGray
Write-Host ""
Write-Host "For instant Harm Touch spam: use #cleartimers 5088 after each use." -ForegroundColor Yellow
Write-Host ""
