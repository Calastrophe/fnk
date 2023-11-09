@echo off
setlocal enabledelayedexpansion

REM Start frontend
start cmd.exe /k "cd frontend && set CARGO_TARGET_DIR=../target-trunk && trunk serve --address 0.0.0.0"

REM Start backend
start cmd.exe /k "cd backend && cargo watch -- cargo run -- --port 8081"

endlocal
