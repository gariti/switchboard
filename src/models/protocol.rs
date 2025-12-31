//! Protocol definitions for supported communication networks.

/// Category of protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolCategory {
    /// Real-time chat (IRC, Matrix, XMPP)
    Chat,
    /// Peer-to-peer encrypted (Tox)
    P2P,
    /// SmolNet protocols (Gemini, Gopher)
    SmolNet,
    /// Classic internet (Usenet)
    Classic,
    /// Community shells (Tildeverse)
    Community,
}

/// A supported communication protocol.
#[derive(Debug, Clone)]
pub struct Protocol {
    /// Display name
    pub name: &'static str,
    /// Short description
    pub description: &'static str,
    /// Category for grouping
    pub category: ProtocolCategory,
    /// Binary to check/launch
    pub binary: &'static str,
    /// Default arguments when launching
    pub default_args: &'static [&'static str],
    /// Icon (nerd font codepoint or ASCII)
    pub icon: &'static str,
    /// NixOS package name for install hint
    pub nix_package: &'static str,
    /// Whether this is an SSH-based protocol
    pub is_ssh: bool,
    /// SSH host for SSH-based protocols
    pub ssh_host: Option<&'static str>,
}

impl Protocol {
    /// Get all supported protocols.
    pub fn all() -> Vec<Protocol> {
        vec![
            Protocol {
                name: "IRC",
                description: "Internet Relay Chat - The OG chat protocol since 1988",
                category: ProtocolCategory::Chat,
                binary: "weechat",
                default_args: &[],
                icon: "\u{f6a8}", // nf-md-chat
                nix_package: "weechat",
                is_ssh: false,
                ssh_host: None,
            },
            Protocol {
                name: "Matrix",
                description: "Decentralized, encrypted communication network",
                category: ProtocolCategory::Chat,
                binary: "gomuks",
                default_args: &[],
                icon: "\u{f1d8}", // nf-fa-paper_plane (approximate)
                nix_package: "gomuks",
                is_ssh: false,
                ssh_host: None,
            },
            Protocol {
                name: "XMPP",
                description: "Jabber - Federated messaging since 1999",
                category: ProtocolCategory::Chat,
                binary: "profanity",
                default_args: &[],
                icon: "\u{f4fc}", // nf-md-message_text
                nix_package: "profanity",
                is_ssh: false,
                ssh_host: None,
            },
            Protocol {
                name: "Tox",
                description: "P2P encrypted messaging - No servers, no masters",
                category: ProtocolCategory::P2P,
                binary: "toxic",
                default_args: &[],
                icon: "\u{f49c}", // nf-md-lock
                nix_package: "toxic",
                is_ssh: false,
                ssh_host: None,
            },
            Protocol {
                name: "Gemini",
                description: "Lightweight hypertext protocol - The cozy web",
                category: ProtocolCategory::SmolNet,
                binary: "amfora",
                default_args: &[],
                icon: "\u{f484}", // nf-md-earth (approximate)
                nix_package: "amfora",
                is_ssh: false,
                ssh_host: None,
            },
            Protocol {
                name: "Gopher",
                description: "The 1991 protocol that should have won",
                category: ProtocolCategory::SmolNet,
                binary: "amfora",  // amfora supports gopher too
                default_args: &["gopher://gopher.floodgap.com"],
                icon: "\u{f7a2}", // nf-md-turtle
                nix_package: "amfora",
                is_ssh: false,
                ssh_host: None,
            },
            Protocol {
                name: "Usenet",
                description: "NNTP newsgroups - Decentralized discussion since 1980",
                category: ProtocolCategory::Classic,
                binary: "slrn",
                default_args: &[],
                icon: "\u{f0c0}", // nf-fa-users
                nix_package: "slrn",
                is_ssh: false,
                ssh_host: None,
            },
            Protocol {
                name: "Tildeverse",
                description: "Public Unix shells - SSH into a community",
                category: ProtocolCategory::Community,
                binary: "ssh",
                default_args: &[],
                icon: "\u{f489}", // nf-md-console
                nix_package: "openssh",
                is_ssh: true,
                ssh_host: Some("tilde.town"),
            },
        ]
    }

    /// Check if the client binary is available.
    pub fn is_available(&self) -> bool {
        which::which(self.binary).is_ok()
    }

    /// Get the command and args to launch this protocol's client.
    pub fn launch_command(&self) -> (String, Vec<String>) {
        let args: Vec<String> = self.default_args.iter().map(|s| s.to_string()).collect();
        (self.binary.to_string(), args)
    }
}
