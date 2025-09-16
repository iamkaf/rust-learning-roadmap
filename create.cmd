@echo off
REM Wrapper script for create-project tool (Windows)

REM Change to script directory (where the workspace root is)
cd /d "%~dp0"

REM Run the create-project tool with all passed arguments
cargo run --bin create-project -p tools -- %*