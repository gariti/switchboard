//! Screen components for switchboard.

mod browser;

pub use browser::BrowserScreen;

use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::layout::Rect;

/// Action that a screen can request.
#[derive(Debug, Clone)]
pub enum ScreenAction {
    /// No action needed.
    None,
    /// Display a status message.
    StatusMessage(String),
    /// Launch a client with the given command and args.
    LaunchClient { command: String, args: Vec<String> },
}

/// Trait for screen components.
pub trait Screen {
    /// Handle a key event.
    fn handle_key(&mut self, key: KeyEvent) -> ScreenAction;

    /// Draw the screen.
    fn draw(&mut self, f: &mut Frame, area: Rect);
}
