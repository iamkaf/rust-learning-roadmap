# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a comprehensive Rust learning roadmap with 150 projects organized in a Cargo workspace. The repository teaches Rust through hands-on projects progressing from basic syntax to advanced systems programming.

## 🏗️ Workspace Structure & Usage Guide

This repository is organized as a **Cargo workspace** to accommodate the diverse project types and dependencies across all 150 projects. Each category has its own workspace member with appropriate dependencies.

### 📁 **Project Organization**

```
rust-learning-roadmap/
├── Cargo.toml                    # Workspace root
├── README.md
├── basic-projects/               # Projects 1-30 (Beginner)
│   ├── Cargo.toml               # Dependencies: rand, chrono, etc.
│   └── src/bin/                 # Individual project files
├── ownership-projects/           # Projects 31-45 (Ownership & Borrowing)
│   ├── Cargo.toml               # Dependencies: regex
│   └── src/bin/
├── web-projects/                # Web & Async Projects
│   ├── Cargo.toml               # Dependencies: tokio, axum, reqwest
│   └── src/bin/
├── game-projects/               # Game Development Projects
│   ├── Cargo.toml               # Dependencies: bevy, macroquad, rand
│   └── src/bin/
├── desktop-projects/            # GUI & Desktop Applications
│   ├── Cargo.toml               # Dependencies: iced, egui, ratatui
│   └── src/bin/
├── wasm-projects/               # WebAssembly Projects
│   ├── Cargo.toml               # Dependencies: wasm-bindgen, web-sys
│   └── src/lib.rs               # WebAssembly library
└── advanced-projects/           # Complex Systems (Projects 101-150)
    ├── Cargo.toml               # Dependencies: tokio, serde, anyhow
    └── src/bin/
```

### 🚀 **Getting Started**

1. **Clone and explore the workspace:**
   ```bash
   git clone <repository-url>
   cd rust-learning-roadmap

   # Build entire workspace
   cargo build

   # Check workspace status
   cargo check
   ```

2. **Run existing projects:**
   ```bash
   # Project 1: Hello World
   cargo run --bin 01_hello_world -p basic-projects

   # Project 4: Number Guessing Game
   cargo run --bin 04_guessing_game -p basic-projects
   ```

### ⚡ **Working with Projects**

#### **Running Projects by Category**
```bash
# Basic projects (1-30)
cargo run --bin <project_name> -p basic-projects

# Ownership projects (31-45)
cargo run --bin <project_name> -p ownership-projects

# Web projects
cargo run --bin <project_name> -p web-projects

# Game projects
cargo run --bin <project_name> -p game-projects

# Desktop projects
cargo run --bin <project_name> -p desktop-projects

# Advanced projects (101-150)
cargo run --bin <project_name> -p advanced-projects
```

#### **WebAssembly Projects**
WebAssembly projects require special handling:
```bash
# Build WASM project
wasm-pack build wasm-projects --target web

# Serve WASM in browser
# (requires local web server for security)
```

#### **Building Specific Workspace Members**
```bash
# Build only basic projects
cargo build -p basic-projects

# Build only game projects
cargo build -p game-projects

# Build all projects in workspace
cargo build
```

### 🛠️ **Utility Tools**

This roadmap includes powerful utility tools to enhance your learning experience:

#### **Next Project Tool** 🎯
Find your next uncompleted project instantly:

```bash
# Find the next uncompleted project
./next

# Show all uncompleted projects
./next --all

# Find projects from specific level (1-10)
./next --level 3

# Find projects in specific workspace
./next --workspace basic-projects

# Using cargo alias
cargo next --level 5
```

#### **Code Search Tool** 🔍
Search across all workspace members with powerful filtering:

```bash
# Basic search
./search "tokio"

# Search with context
./search "struct" -C 3

# Built-in shortcuts
./search --structs     # Find all struct definitions
./search --functions   # Find all function definitions
./search --traits      # Find all trait definitions
./search --enums       # Find all enum definitions

# Search in specific workspace
./search "main" -w basic-projects

# Case insensitive search
./search "Hello" -i

# Count matches only
./search "fn" --count

# Using cargo alias
cargo find --functions -C 2
```

#### **Project Creator Tool** ✨
Interactive project creation wizard:

```bash
# Launch interactive creator
./create

# Using cargo alias
cargo create
```

**Interactive Flow:**
1. Shows next recommended project
2. Options: Create next project OR choose specific project number
3. Generates starter template with TODO comments
4. Provides run command for immediate testing

#### **Progress Tracker Tool** 🎉
Advanced analytics dashboard with gamification:

```bash
# Main progress dashboard
./progress

# Detailed statistics
./progress --stats

# ASCII celebration art
./progress --ascii

# Using cargo alias
cargo progress
```

**Features:**
- **Visual Progress Bars**: Colorful progress indicators with completion percentages
- **Achievement System**: Bronze/Silver/Gold/Platinum milestones with unlock celebrations
- **XP & Leveling**: Experience points system with level progression (1-10)
- **Streak Tracking**: Daily coding streaks with best streak records
- **Lines of Code Counter**: Real-time tracking of total Rust code written
- **Analytics**: Projects per week, coding velocity trends, workspace breakdowns
- **Gamification**: Motivational quotes, rank titles, ASCII celebrations
- **Data Persistence**: Progress saved in gitignored `.progress.json` file

**Achievement Tiers:**
- 🥉 Bronze (30 projects): Rust Syntax Master
- 🥈 Silver (65 projects): Rust Ownership Master
- 🥇 Gold (100 projects): Complete Rust Developer
- 💎 Platinum (150 projects): Rust Systems Architect

#### **Cross-Platform Support**
- **Unix/Linux/macOS**: Use `./next`, `./search`, `./create`, and `./progress`
- **Windows**: Use `next.cmd`, `search.cmd`, `create.cmd`, and `progress.cmd`
- **Cargo Aliases**: Use `cargo next`, `cargo find`, `cargo create`, and `cargo progress` on any platform

### 📝 **Adding New Projects**

#### **Easy Way (Recommended):**
```bash
./create        # Interactive project creator
cargo create    # Using cargo alias
```

#### **Manual Way:**
1. **Find your next project** using `./next` or `cargo next`
2. **Determine the appropriate workspace member** based on project type and dependencies
3. **Create new binary file** in the `src/bin/` directory:
   ```bash
   # Example: Adding project 5 (Temperature Converter)
   touch basic-projects/src/bin/05_temperature_converter.rs
   ```
4. **Implement your project** in the new file
5. **Run your project:**
   ```bash
   cargo run --bin 05_temperature_converter -p basic-projects
   ```
6. **Search for examples** using `./search` if you need reference code

### 🔧 **Dependency Management**

Each workspace member has curated dependencies for its project types:

- **basic-projects**: `rand`, `chrono`, `uuid` - fundamental crates
- **ownership-projects**: `regex` - for string processing projects
- **web-projects**: `tokio`, `axum`, `reqwest`, `serde` - async web stack
- **game-projects**: `bevy`, `macroquad`, `rand` - game development
- **desktop-projects**: `iced`, `egui`, `ratatui` - GUI frameworks
- **wasm-projects**: `wasm-bindgen`, `web-sys` - WebAssembly bindings
- **advanced-projects**: `tokio`, `serde`, `anyhow` - system building tools

**Adding dependencies:** Edit the `Cargo.toml` of the appropriate workspace member.

### 🎯 **Learning Workflow**

1. **Start with basic-projects** (Projects 1-30) to learn Rust syntax
2. **Progress to ownership-projects** (Projects 31-45) for memory management concepts
3. **Choose your path:** web, games, desktop, or advanced projects
4. **Build incrementally** - each project builds on previous knowledge
5. **Experiment freely** - the workspace structure encourages exploration

## Additional Workspace Members

Beyond the main project workspaces, there are also:

- **tools/**: Utility tools for finding next projects and code searching
- **experiments/**: Personal experimentation and self-study projects

## Key Dependencies by Workspace (Complete List)

- **basic-projects**: `rand`, `chrono`, `uuid`
- **ownership-projects**: `regex`
- **web-projects**: `tokio`, `axum`, `reqwest`, `serde`
- **game-projects**: `bevy`, `macroquad`, `rand`
- **desktop-projects**: `iced`, `egui`, `ratatui`
- **wasm-projects**: `wasm-bindgen`, `web-sys`
- **advanced-projects**: `tokio`, `serde`, `anyhow`
- **tools**: `clap`, `regex`, `anyhow`, `colored`, `walkdir`, `dialoguer`, `indicatif`, `chrono`, `serde`, `serde_json`
- **experiments**: `serde`, `anyhow`, `clap`, `tokio`, `reqwest`, `rand`

## Workspace Management Notes

- All projects are tracked in README.md with checkboxes for completion status
- Progress analytics stored in `.progress.json` (gitignored for personal data)
- The workspace uses Rust Edition 2024
- Cross-platform support with shell scripts (Unix) and .cmd files (Windows)
- Cargo aliases available for all utility tools: `cargo next`, `cargo find`, `cargo create`, `cargo progress`

## Testing & Quality

The repository doesn't appear to have centralized testing commands. Test individual projects with standard `cargo test` in their respective workspace members.