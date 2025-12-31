//! Client launcher service.

use crate::models::Protocol;

/// Service for launching protocol clients.
pub struct Launcher;

impl Launcher {
    /// Check if a protocol's client is available on the system.
    pub fn is_available(protocol: &Protocol) -> bool {
        protocol.is_available()
    }

    /// Get install hint for missing client.
    pub fn install_hint(protocol: &Protocol) -> String {
        format!(
            "Client '{}' not found. Install with:\n  nix-shell -p {}",
            protocol.binary,
            protocol.nix_package
        )
    }

    /// Get the command and arguments to launch a protocol client.
    pub fn get_launch_command(protocol: &Protocol) -> (String, Vec<String>) {
        protocol.launch_command()
    }
}
