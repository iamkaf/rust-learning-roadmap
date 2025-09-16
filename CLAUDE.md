# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a comprehensive Rust learning roadmap with 150 projects organized in a Cargo workspace. The repository teaches Rust through hands-on projects progressing from basic syntax to advanced systems programming.

## Architecture & Structure

The codebase is organized as a **Cargo workspace** with specialized workspace members:

- **basic-projects/**: Projects 1-30 (beginner syntax and control flow)
- **ownership-projects/**: Projects 31-45 (ownership, borrowing, lifetimes)
- **web-projects/**: Web and async projects (tokio, axum, reqwest)
- **game-projects/**: Game development (bevy, macroquad, rand)
- **desktop-projects/**: GUI applications (iced, egui, ratatui)
- **wasm-projects/**: WebAssembly projects (wasm-bindgen, web-sys)
- **advanced-projects/**: Complex systems (Projects 101-150)
- **tools/**: Utility tools for finding next projects and code searching
- **experiments/**: Personal experimentation and self-study projects

Each workspace member has its own `Cargo.toml` with curated dependencies appropriate for its project types.

## Development Commands

### Building Projects
```bash
# Build entire workspace
cargo build

# Build specific workspace member
cargo build -p basic-projects
cargo build -p game-projects

# Check workspace
cargo check
```

### Running Projects
Projects are located in `src/bin/` directories with naming convention `NN_project_name.rs`:

```bash
# Run specific project
cargo run --bin 01_hello_world -p basic-projects
cargo run --bin 04_guessing_game -p basic-projects
cargo run --bin <project_name> -p <workspace-member>
```

### WebAssembly Projects
```bash
# Build WASM project
wasm-pack build wasm-projects --target web
```

### Utility Tools

**Next Project Tool** - Find uncompleted projects:
```bash
./next                          # Find next uncompleted project
./next --all                    # Show all uncompleted projects
./next --level 3                # Find projects from specific level
./next --workspace basic-projects  # Find projects in specific workspace
cargo next --level 5            # Using cargo alias
```

**Code Search Tool** - Search across workspace:
```bash
./search "pattern"              # Basic search
./search "struct" -C 3          # Search with context
./search --structs              # Find struct definitions
./search --functions            # Find function definitions
./search --traits               # Find trait definitions
./search --enums                # Find enum definitions
./search "main" -w basic-projects  # Search in specific workspace
cargo find --functions -C 2     # Using cargo alias
```

## Project Creation Workflow

1. Use `./next` or `cargo next` to find your next project
2. Determine the appropriate workspace member based on project type
3. Create new binary file: `touch <workspace>/src/bin/NN_project_name.rs`
4. Implement the project
5. Run with: `cargo run --bin NN_project_name -p <workspace>`

## Key Dependencies by Workspace

- **basic-projects**: `rand`, `chrono`, `uuid`
- **ownership-projects**: `regex`
- **web-projects**: `tokio`, `axum`, `reqwest`, `serde`
- **game-projects**: `bevy`, `macroquad`, `rand`
- **desktop-projects**: `iced`, `egui`, `ratatui`
- **wasm-projects**: `wasm-bindgen`, `web-sys`
- **advanced-projects**: `tokio`, `serde`, `anyhow`
- **tools**: `clap`, `regex`, `anyhow`, `colored`, `walkdir`
- **experiments**: `serde`, `anyhow`, `clap`, `tokio`, `reqwest`, `rand`

## Workspace Management

- All projects are tracked in README.md with checkboxes for completion status
- The workspace uses Rust Edition 2024
- Cross-platform support with shell scripts (Unix) and .cmd files (Windows)
- Cargo aliases available for `cargo next` and `cargo find` commands

## Testing & Quality

The repository doesn't appear to have centralized testing commands. Test individual projects with standard `cargo test` in their respective workspace members.