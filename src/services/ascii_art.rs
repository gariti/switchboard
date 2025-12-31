//! ASCII art for the switchboard splash screen.

use rand::seq::SliceRandom;

/// Collection of ASCII art pieces for the splash screen.
pub struct AsciiArt;

impl AsciiArt {
    /// Get a random piece of ASCII art.
    pub fn random() -> &'static str {
        let art = [
            SWITCHBOARD_CLASSIC,
            NETWORK_NODES,
            PATCH_PANEL,
            TELEGRAPH,
        ];
        art.choose(&mut rand::thread_rng()).unwrap_or(&SWITCHBOARD_CLASSIC)
    }

    /// Get a random hacker quote.
    pub fn random_quote() -> &'static str {
        let quotes = [
            "\"Information wants to be free.\" - Stewart Brand",
            "\"The Net interprets censorship as damage and routes around it.\" - John Gilmore",
            "\"We reject kings, presidents and voting. We believe in rough consensus and running code.\" - Dave Clark",
            "\"In cyberspace, the First Amendment is a local ordinance.\" - John Perry Barlow",
            "\"Number, please?\" - Every switchboard operator, 1920s",
            "\"The street finds its own uses for things.\" - William Gibson",
            "\"There is no cloud, just other people's computers.\" - Unknown",
            "\"Privacy is not something that I'm merely entitled to, it's an absolute prerequisite.\" - Marlon Brando",
            "\"Arguing that you don't care about privacy because you have nothing to hide is like saying you don't care about free speech because you have nothing to say.\" - Edward Snowden",
            "\"Unix is user-friendly. It's just very selective about who its friends are.\" - Unknown",
        ];
        quotes.choose(&mut rand::thread_rng()).unwrap_or(&quotes[0])
    }
}

/// Classic telephone switchboard with patch cables.
pub const SWITCHBOARD_CLASSIC: &str = r#"
    ╔═══════════════════════════════════════════════════════════════════╗
    ║                                                                   ║
    ║    ┌──┐   ┌──┐   ┌──┐   ┌──┐   ┌──┐   ┌──┐   ┌──┐   ┌──┐        ║
    ║    │▓▓│   │▓▓│   │▓▓│   │▓▓│   │▓▓│   │▓▓│   │▓▓│   │▓▓│        ║
    ║    └┬─┘   └┬─┘   └┬─┘   └┬─┘   └┬─┘   └┬─┘   └┬─┘   └┬─┘        ║
    ║     │      │      │      │      │      │      │      │          ║
    ║     ├──────┼──────┼──────┴──────┼──────┼──────┼──────┤          ║
    ║     │      │      │             │      │      │      │          ║
    ║    IRC  MATRIX  XMPP          TOX  GEMINI GOPHER USENET TILDE   ║
    ║                                                                   ║
    ╚═══════════════════════════════════════════════════════════════════╝
"#;

/// Network nodes with connections.
pub const NETWORK_NODES: &str = r#"
                    ╭───────╮
                    │ MATRIX│
                    ╰───┬───╯
                        │
        ╭───────╮   ╭───┴───╮   ╭───────╮
        │  IRC  ├───┤  YOU  ├───┤  TOX  │
        ╰───┬───╯   ╰───┬───╯   ╰───┬───╯
            │           │           │
            │       ╭───┴───╮       │
            ╰───────┤ XMPP  ├───────╯
                    ╰───┬───╯
                        │
            ╭───────────┼───────────╮
            │           │           │
        ╭───┴───╮   ╭───┴───╮   ╭───┴───╮
        │GEMINI │   │USENET │   │ TILDE │
        ╰───────╯   ╰───────╯   ╰───────╯
"#;

/// Patch panel / rack style.
pub const PATCH_PANEL: &str = r#"
    ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
    ┃  ○ IRC      ○ MATRIX    ○ XMPP      ○ TOX                   ┃
    ┃  ┃          ┃           ┃           ┃                       ┃
    ┃  ╰──────────┴───────────┴───────────╯                       ┃
    ┃                         ┏━━━━━━━━━━━┓                       ┃
    ┃                         ┃ SWITCHBOARD┃                       ┃
    ┃                         ┗━━━━━━━━━━━┛                       ┃
    ┃  ╭──────────┬───────────┬───────────╮                       ┃
    ┃  ┃          ┃           ┃           ┃                       ┃
    ┃  ○ GEMINI   ○ GOPHER    ○ USENET    ○ TILDE                 ┃
    ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
"#;

/// Telegraph / morse code style.
pub const TELEGRAPH: &str = r#"
    ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄
    █                                                             █
    █   ─ ─ ─   ─ ─ ─   ─ ─ ─     S W I T C H B O A R D          █
    █                                                             █
    █   ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐  █
    █   │ IRC │ │MATRX│ │XMPP │ │ TOX │ │GMINI│ │GOPHR│ │USENT│  █
    █   └──┬──┘ └──┬──┘ └──┬──┘ └──┬──┘ └──┬──┘ └──┬──┘ └──┬──┘  █
    █      │       │       │       │       │       │       │      █
    █   ═══╧═══════╧═══════╧═══════╧═══════╧═══════╧═══════╧═══   █
    █                                                             █
    █▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄█
"#;
