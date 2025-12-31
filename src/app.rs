//! Main application state and event loop.

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use std::io;
use std::sync::Arc;
use std::time::Duration;

use crate::screens::{BrowserScreen, Screen, ScreenAction};
use crate::services::Theme;

/// Result of running the application.
#[derive(Debug)]
pub enum AppResult {
    /// User quit normally.
    Exit,
    /// Launch a client with the given command and arguments.
    LaunchClient { command: String, args: Vec<String> },
}

/// Application state.
pub struct App {
    should_quit: bool,
    launch_client: Option<(String, Vec<String>)>,

    // Theme
    theme: Arc<Theme>,

    // Screens
    browser_screen: BrowserScreen,

    // Status bar info
    status_message: String,
}

impl App {
    /// Create a new application instance.
    pub async fn new() -> Result<Self> {
        let theme = Arc::new(Theme::load());

        // Initialize screens
        let browser_screen = BrowserScreen::new(theme.clone());

        Ok(Self {
            should_quit: false,
            launch_client: None,
            theme,
            browser_screen,
            status_message: "Select a network to connect".to_string(),
        })
    }

    /// Run the application.
    pub async fn run(&mut self) -> Result<AppResult> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Main event loop
        let result = self.event_loop(&mut terminal).await;

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        result
    }

    /// Main event loop.
    async fn event_loop(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    ) -> Result<AppResult> {
        loop {
            // Draw UI
            terminal.draw(|f| self.draw(f))?;

            // Poll for events with timeout
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    // Global key handlers
                    match (key.modifiers, key.code) {
                        (KeyModifiers::CONTROL, KeyCode::Char('c'))
                        | (KeyModifiers::CONTROL, KeyCode::Char('q')) => {
                            self.should_quit = true;
                        }
                        (_, KeyCode::Char('q')) => {
                            self.should_quit = true;
                        }
                        (_, KeyCode::Char('?')) => {
                            self.status_message = "j/k: navigate │ Enter: connect │ r: new art │ q: quit".to_string();
                        }
                        _ => {
                            // Delegate to browser screen
                            match self.browser_screen.handle_key(key) {
                                ScreenAction::None => {}
                                ScreenAction::StatusMessage(msg) => {
                                    self.status_message = msg;
                                }
                                ScreenAction::LaunchClient { command, args } => {
                                    self.launch_client = Some((command, args));
                                    self.should_quit = true;
                                }
                            }
                        }
                    }
                }
            }

            if self.should_quit {
                break;
            }
        }

        Ok(if let Some((command, args)) = self.launch_client.take() {
            AppResult::LaunchClient { command, args }
        } else {
            AppResult::Exit
        })
    }

    /// Draw the UI.
    fn draw(&mut self, f: &mut ratatui::Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title bar
                Constraint::Min(0),    // Main content
                Constraint::Length(1), // Status bar
            ])
            .split(f.area());

        // Title bar
        let title = Paragraph::new(Line::from(vec![
            Span::styled(
                " ☎ SWITCHBOARD ",
                Style::default()
                    .fg(self.theme.color6)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "─ Unified Terminal Communication Hub",
                Style::default().fg(self.theme.color8),
            ),
        ]))
        .style(Style::default().bg(self.theme.color0));

        f.render_widget(title, chunks[0]);

        // Main content area
        self.browser_screen.draw(f, chunks[1]);

        // Status bar
        let status = Paragraph::new(Line::from(vec![
            Span::raw(" "),
            Span::styled(&self.status_message, Style::default().fg(self.theme.color7)),
            Span::raw(" │ "),
            Span::styled("j/k", Style::default().fg(self.theme.color8)),
            Span::styled(" Navigate", Style::default().fg(self.theme.color7)),
            Span::raw(" "),
            Span::styled("Enter", Style::default().fg(self.theme.color8)),
            Span::styled(" Connect", Style::default().fg(self.theme.color7)),
            Span::raw(" "),
            Span::styled("r", Style::default().fg(self.theme.color8)),
            Span::styled(" Refresh", Style::default().fg(self.theme.color7)),
            Span::raw(" "),
            Span::styled("q", Style::default().fg(self.theme.color8)),
            Span::styled(" Quit", Style::default().fg(self.theme.color7)),
        ]));
        f.render_widget(status, chunks[2]);
    }
}
