@echo off
REM Wrapper script for search-code tool (Windows)

REM Change to script directory (where the workspace root is)
cd /d "%~dp0"

REM Run the search-code tool with all passed arguments
cargo run --bin search-code -p tools -- %*