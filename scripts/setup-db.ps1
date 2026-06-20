<#
.SYNOPSIS
  Sets up the ADIF PostgreSQL database with schema and seed data.

.DESCRIPTION
  1. Starts the ADIF database containers (PostgreSQL + Redis)
  2. Waits for PostgreSQL to be ready
  3. Runs all migration files in order
  4. Reports table counts

.NOTES
  Run from the ADIF project root: .\scripts\setup-db.ps1
  Requires Docker Desktop to be running.
#>

param(
    [switch]$Reset  # Drop and recreate the database
)

$dbDir = Join-Path $PSScriptRoot "..\database"

Write-Host "`n=== ADIF Database Setup ===" -ForegroundColor Yellow
Write-Host ""

# ── 1. Check Docker ─────────────────────────────────────────────
$dockerRunning = $false
try {
    $null = docker info 2>$null
    if ($LASTEXITCODE -eq 0) { $dockerRunning = $true }
} catch {}

if (-not $dockerRunning) {
    Write-Host "Docker Desktop is not running. Start it first." -ForegroundColor Red
    exit 1
}
Write-Host "[OK] Docker is running" -ForegroundColor Green

# ── 2. Reset if requested ──────────────────────────────────────
if ($Reset) {
    Write-Host "`nResetting database (dropping volume)..." -ForegroundColor Cyan
    Push-Location $dbDir
    docker compose down -v 2>&1 | ForEach-Object { Write-Host "  $_" -ForegroundColor DarkGray }
    Pop-Location
}

# ── 3. Start containers ────────────────────────────────────────
Write-Host "`nStarting PostgreSQL + Redis..." -ForegroundColor Cyan
Push-Location $dbDir
docker compose up -d 2>&1 | ForEach-Object { Write-Host "  $_" -ForegroundColor DarkGray }
Pop-Location

# ── 4. Wait for PostgreSQL ──────────────────────────────────────
Write-Host "`nWaiting for PostgreSQL to accept connections..." -ForegroundColor Cyan
$maxWait = 60
$elapsed = 0
$ready = $false

while ($elapsed -lt $maxWait) {
    $result = docker compose -f "$dbDir\docker-compose.yml" exec -T postgres pg_isready -U adif 2>$null
    if ($LASTEXITCODE -eq 0) {
        $ready = $true
        break
    }
    Start-Sleep -Seconds 2
    $elapsed += 2
}

if (-not $ready) {
    Write-Host "PostgreSQL didn't start within $maxWait seconds." -ForegroundColor Red
    exit 1
}
Write-Host "[OK] PostgreSQL is ready" -ForegroundColor Green

# ── 5. Check if migrations already ran ──────────────────────────
$tableCount = docker compose -f "$dbDir\docker-compose.yml" exec -T postgres `
    psql -U adif -d adif -t -c "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'public';" 2>$null
$tableCount = $tableCount.Trim()

if ([int]$tableCount -gt 0 -and -not $Reset) {
    Write-Host "`n[OK] Database already has $tableCount tables. Use -Reset to recreate." -ForegroundColor Green
} else {
    # Docker's initdb only runs on first creation. Run migrations manually if needed.
    Write-Host "`nRunning migrations..." -ForegroundColor Cyan
    $migrations = Get-ChildItem "$dbDir\migrations\*.sql" | Sort-Object Name
    foreach ($sql in $migrations) {
        Write-Host "  Running $($sql.Name)..." -ForegroundColor DarkGray
        $content = Get-Content $sql.FullName -Raw
        docker compose -f "$dbDir\docker-compose.yml" exec -T postgres `
            psql -U adif -d adif -c $content 2>&1 | ForEach-Object {
                if ($_ -match "ERROR") { Write-Host "    $_" -ForegroundColor Red }
            }
    }
}

# ── 6. Report ───────────────────────────────────────────────────
Write-Host "`n=== Database Summary ===" -ForegroundColor Yellow

$summary = docker compose -f "$dbDir\docker-compose.yml" exec -T postgres `
    psql -U adif -d adif -c "
        SELECT table_name,
               (xpath('/row/cnt/text()', xml_count))[1]::text::int as row_count
        FROM (
            SELECT table_name,
                   query_to_xml('SELECT COUNT(*) AS cnt FROM ' || table_name, false, true, '') as xml_count
            FROM information_schema.tables
            WHERE table_schema = 'public'
            ORDER BY table_name
        ) t;
    " 2>$null

Write-Host $summary

Write-Host "`n[OK] ADIF database is ready at localhost:5432" -ForegroundColor Green
Write-Host "     Connection: postgresql://adif:adif_dev@localhost:5432/adif" -ForegroundColor DarkGray
Write-Host "     Redis:      localhost:6379" -ForegroundColor DarkGray
Write-Host ""
