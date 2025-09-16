@echo off
REM Wrapper script for progress-tracker tool (Windows)

REM Change to script directory (where the workspace root is)
cd /d "%~dp0"

REM Run the progress-tracker tool with all passed arguments
cargo run --bin progress-tracker -p tools -- %*