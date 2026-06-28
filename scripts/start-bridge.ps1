# ADIF Protocol Bridge Launcher
# Starts the EQ-to-Protobuf bridge on UDP port 5998
# Then launches the EQ client

$ErrorActionPreference = "Stop"
$env:PROTOC = "$env:USERPROFILE\protoc\bin\protoc.exe"

Write-Host ""
Write-Host "=== ADIF Protocol Bridge ===" -ForegroundColor Cyan
Write-Host "Bridge: UDP :5998 (EQ client) -> TCP :7000 (zone server)" -ForegroundColor Gray
Write-Host ""

# Build if needed
Write-Host "Building bridge..." -ForegroundColor Yellow
Push-Location E:\development\adif\server
cargo build --bin adif-bridge 2>&1 | Select-Object -Last 3
Pop-Location

# Start bridge in its own window
Write-Host "Starting bridge..." -ForegroundColor Green
Start-Process cmd -ArgumentList '/k', "title ADIF-Bridge && cd /d E:\development\adif\server && set PROTOC=$env:USERPROFILE\protoc\bin\protoc.exe && cargo run --bin adif-bridge -- server.toml"

Start-Sleep -Seconds 3

# Launch EQ client
Write-Host "Launching EQ client..." -ForegroundColor Green
$eqPath = "E:\development\adif\reference\eq-client"
Start-Process -FilePath "$eqPath\eqgame.exe" -ArgumentList "patchme" -WorkingDirectory $eqPath

Write-Host ""
Write-Host "Bridge is running in the ADIF-Bridge window." -ForegroundColor Cyan
Write-Host "Watch that window for packet logs." -ForegroundColor Gray
Write-Host "The EQ client should connect to the bridge on port 5998." -ForegroundColor Gray
