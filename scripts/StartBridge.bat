@echo off
title ADIF Protocol Bridge
echo.
echo === ADIF Protocol Bridge ===
echo EQ Client (UDP :5998) -- Bridge -- Rust Zone Server (TCP :7000)
echo.

set PROTOC=%USERPROFILE%\protoc\bin\protoc.exe
set RUST_LOG=trace

echo Building bridge...
cd /d E:\development\adif\server
cargo build --bin adif-bridge 2>&1
if errorlevel 1 (
    echo BUILD FAILED
    pause
    exit /b 1
)

echo.
echo Starting bridge on port 5998...
echo Launch EQ client when you see "UDP listener bound"
echo.
cargo run --bin adif-bridge -- server.toml
pause
