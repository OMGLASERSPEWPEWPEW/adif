<#
.SYNOPSIS
  Validates the ADIF protobuf protocol layer.
.DESCRIPTION
  Runs buf lint, buf build, buf breaking, and the C# round-trip tests.
  Use before committing proto changes.
.EXAMPLE
  .\scripts\proto-check.ps1
#>

$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
$failed = 0

$buf = (Get-Command buf -ErrorAction SilentlyContinue).Source
if (-not $buf) {
    $buf = Join-Path $env:LOCALAPPDATA "Microsoft\WindowsApps\buf.exe"
}
if (-not (Test-Path $buf)) {
    Write-Host "buf not found. Install: https://buf.build/docs/installation" -ForegroundColor Red
    exit 1
}

Write-Host "`n=== ADIF Proto Check ===" -ForegroundColor Yellow

# ── 1. buf lint ──
Write-Host "`n[1/4] buf lint..." -ForegroundColor Cyan
$output = & $buf lint "$root\proto" 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host "  PASS" -ForegroundColor Green
} else {
    Write-Host "  FAIL" -ForegroundColor Red
    $output | ForEach-Object { Write-Host "  $_" -ForegroundColor Red }
    $failed++
}

# ── 2. buf build ──
Write-Host "`n[2/4] buf build..." -ForegroundColor Cyan
$output = & $buf build "$root\proto" 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host "  PASS" -ForegroundColor Green
} else {
    Write-Host "  FAIL" -ForegroundColor Red
    $output | ForEach-Object { Write-Host "  $_" -ForegroundColor Red }
    $failed++
}

# ── 3. buf breaking ──
$baseline = "$root\proto\image.bin"
if (Test-Path $baseline) {
    Write-Host "`n[3/4] buf breaking --against image.bin..." -ForegroundColor Cyan
    $output = & $buf breaking "$root\proto" --against $baseline 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "  PASS (no breaking changes)" -ForegroundColor Green
    } else {
        Write-Host "  BREAKING CHANGES DETECTED" -ForegroundColor Red
        $output | ForEach-Object { Write-Host "  $_" -ForegroundColor Red }
        $failed++
    }
} else {
    Write-Host "`n[3/4] buf breaking... SKIPPED (no baseline at proto/image.bin)" -ForegroundColor Yellow
}

# ── 4. C# round-trip tests ──
Write-Host "`n[4/4] dotnet run ProtoRoundTrip..." -ForegroundColor Cyan
$output = dotnet run --project "$root\tests\ProtoRoundTrip" 2>&1
$testExit = $LASTEXITCODE
$output | ForEach-Object { Write-Host "  $_" }
if ($testExit -eq 0) {
    Write-Host "  PASS" -ForegroundColor Green
} else {
    Write-Host "  FAIL" -ForegroundColor Red
    $failed++
}

# ── Summary ──
Write-Host "`n=== Results ===" -ForegroundColor Yellow
if ($failed -eq 0) {
    Write-Host "ALL CHECKS PASSED" -ForegroundColor Green
} else {
    Write-Host "$failed CHECK(S) FAILED" -ForegroundColor Red
    exit 1
}
