# KittyCraft

![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)
![Rust 1.90+](https://img.shields.io/badge/rust-1.90%2B-orange?logo=rust)
![MCP stdio](https://img.shields.io/badge/MCP-stdio-purple)
![ArtCraft harness](https://img.shields.io/badge/ArtCraft-harness-green)

**Local-first cat and kitten advisor, breed encyclopedia, care/medical/legal hub, and AI chat workstation in pure Rust.** Includes kitten and adult breed portraits, daily diary, sitter calendar, bundled skills, and the `Ctrl+J` Katzi-Berater AI advisor chat dock.

## Features

- **Breed Encyclopedia**: Kitten and adult portraits with temperament, care, and housing guidance.
- **Care, Medical & Legal Hub**: Vaccination schedules, health logs, diary, and sitter calendar stored locally in SQLite.
- **Katzi-Berater AI Chat (`Ctrl+J`)**: Skill-grounded AI advisor dock backed by bundled `SKILL.md` skills (`skills list`).
- **Desktop Ergonomics**: `F1` Handbuch & Kurzanleitung, `Ctrl+K` Command Palette, and `Ctrl+Wheel` / `Ctrl+=` / `Ctrl+-` / `Ctrl+0` UI zoom.

## The Craft App Standard (3-Tier ArtCraft Architecture)

KittyCraft implements the **Pure-Rust ArtCraft** 3-tier containment harness:

| Layer | Crate | Role |
|---|---|---|
| **L1 Core** | `crates/kittycraft-core` | Pure-Rust cat & kitten domain engine, SQLite persistence, bundled `SKILL.md` skills, and universal `Command` dispatch bus with zero-panic lint walls (`unwrap_used` / `expect_used` / `panic` denied). |
| **L2 CLI + MCP** | `crates/kittycraft-mcp` | Single native `kittycraft` binary providing `clap` CLI subcommands (`skills list`) and a JSON-RPC `2024-11-05` MCP stdio server (`mcp`) with zero Python/Node runtime dependencies. |
| **L3 Desktop App** | `crates/kittycraft-app` | Native GPU-rendered `eframe`/`egui` workstation invoking L1 directly in-process (zero HTTP/WebView2 overhead). |

**Lineage**: Adapted from the Storytold ArtCraft pattern ([storytold/artcraft](https://github.com/storytold/artcraft)): one workspace, one command bus, headless-first CLI + MCP + native GUI.

## Quick Start

```powershell
# Launch native desktop app (or double-click start.bat)
.\start.ps1

# Smoke test
.\start.ps1 -Smoke

# MCP stdio server & bundled skills CLI
cargo run -p kittycraft-mcp -- mcp
cargo run -p kittycraft-mcp -- skills list
```

## MCP Server Configuration

```json
{
  "mcpServers": {
    "kittycraft": {
      "command": "D:/Dev/repos/kittycraft/target/release/kittycraft.exe",
      "args": ["mcp"]
    }
  }
}
```

## Verification

```powershell
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## License

MIT OR Apache-2.0
