# CLAUDE.md - Switchboard

## Project Overview

Switchboard is a unified terminal communication hub - a TUI launcher for various terminal-based chat and network clients.

**Philosophy:** Don't reinvent the wheel. Launch existing battle-tested TUI clients for each protocol.

## Build & Run

```bash
# Development
cargo run

# Release build
cargo build --release

# Check for errors
cargo check
```

## Architecture

```
src/
├── main.rs              # Entry point, process replacement logic
├── app.rs               # Application state, event loop, main layout
├── screens/
│   ├── mod.rs           # Screen trait definition
│   └── browser.rs       # Main protocol browser screen
├── models/
│   ├── mod.rs
│   └── protocol.rs      # Protocol definitions and client info
├── services/
│   ├── mod.rs
│   ├── ascii_art.rs     # Splash screen ASCII art
│   ├── launcher.rs      # Client availability checking
│   └── theme.rs         # Wallust color integration
└── utils/
    └── mod.rs           # Utilities (currently empty)
```

## Supported Protocols

| Protocol | Client | NixOS Package |
|----------|--------|---------------|
| IRC | weechat | `pkgs.weechat` |
| Matrix | gomuks | `pkgs.gomuks` |
| XMPP | profanity | `pkgs.profanity` |
| Tox | toxic | `pkgs.toxic` |
| Gemini | amfora | `pkgs.amfora` |
| Gopher | amfora | `pkgs.amfora` |
| Usenet | slrn | `pkgs.slrn` |
| Tildeverse | ssh | `pkgs.openssh` |

## Keybindings

| Key | Action |
|-----|--------|
| j/k/up/down | Navigate protocol list |
| Enter | Launch selected client |
| r | Refresh ASCII art |
| ? | Show help |
| q | Quit |

## Adding a New Protocol

1. Add entry to `Protocol::all()` in `src/models/protocol.rs`
2. Specify the binary name, default args, icon, and nix package
3. Test that `which <binary>` works on your system

## Theme Integration

Colors are loaded from `~/.cache/wallust/colors-original.json` (wallust integration).
Falls back to default ANSI colors if wallust isn't configured.

## Development Notes

- Uses ratatui 0.29 for TUI rendering
- Uses `which` crate to check client availability
- On launch, the process is replaced with the client (Unix process replacement)
- No tmux wrapping needed - clients run directly in current terminal
