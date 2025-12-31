//! switchboard - Unified Terminal Communication Hub
//!
//! "Number, please?" - Every switchboard operator, 1920s
//!
//! A terminal-based launcher for communication clients across
//! IRC, Matrix, XMPP, Tox, Gemini, Gopher, Usenet, and the Tildeverse.

mod app;
mod models;
mod screens;
mod services;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Run the TUI application
    let mut app = app::App::new().await?;
    match app.run().await? {
        app::AppResult::Exit => {}
        app::AppResult::LaunchClient { command, args } => {
            // Replace current process with the client using Unix exec syscall
            // This is safe - it's not shell execution, just process replacement
            #[cfg(unix)]
            {
                use std::os::unix::process::CommandExt;
                let err = std::process::Command::new(&command)
                    .args(&args)
                    .exec();
                eprintln!("Failed to launch {}: {}", command, err);
                std::process::exit(1);
            }
            #[cfg(not(unix))]
            {
                // On non-Unix, spawn and wait
                let status = std::process::Command::new(&command)
                    .args(&args)
                    .status()?;
                std::process::exit(status.code().unwrap_or(1));
            }
        }
    }

    Ok(())
}
