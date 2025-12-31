//! Main browser screen for selecting protocols.

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};
use ratatui_garnish::{shadow::HalfShadow, GarnishableWidget};
use std::sync::Arc;

use crate::models::Protocol;
use crate::services::{AsciiArt, Launcher, Theme};

use super::{Screen, ScreenAction};

/// Browser screen for selecting and launching protocol clients.
pub struct BrowserScreen {
    /// All available protocols.
    protocols: Vec<Protocol>,
    /// Selection state.
    list_state: ListState,
    /// Theme colors.
    theme: Arc<Theme>,
    /// Current splash art.
    splash_art: &'static str,
    /// Current quote.
    quote: &'static str,
}

impl BrowserScreen {
    /// Create a new browser screen.
    pub fn new(theme: Arc<Theme>) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            protocols: Protocol::all(),
            list_state,
            theme,
            splash_art: AsciiArt::random(),
            quote: AsciiArt::random_quote(),
        }
    }

    /// Move selection up.
    fn move_up(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.protocols.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    /// Move selection down.
    fn move_down(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.protocols.len().saturating_sub(1) {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    /// Get the currently selected protocol.
    fn selected_protocol(&self) -> Option<&Protocol> {
        self.list_state.selected().and_then(|i| self.protocols.get(i))
    }

    /// Launch the selected protocol's client.
    fn launch_selected(&self) -> ScreenAction {
        if let Some(protocol) = self.selected_protocol() {
            if !Launcher::is_available(protocol) {
                return ScreenAction::StatusMessage(Launcher::install_hint(protocol));
            }

            let (command, args) = Launcher::get_launch_command(protocol);
            ScreenAction::LaunchClient { command, args }
        } else {
            ScreenAction::None
        }
    }
}

impl Screen for BrowserScreen {
    fn handle_key(&mut self, key: KeyEvent) -> ScreenAction {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.move_up();
                ScreenAction::None
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.move_down();
                ScreenAction::None
            }
            KeyCode::Enter => self.launch_selected(),
            KeyCode::Char('r') => {
                // Refresh splash art
                self.splash_art = AsciiArt::random();
                self.quote = AsciiArt::random_quote();
                ScreenAction::None
            }
            _ => ScreenAction::None,
        }
    }

    fn draw(&mut self, f: &mut Frame, area: Rect) {
        // Split into left (protocol list) and right (preview/splash)
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ])
            .split(area);

        // Draw protocol list
        self.draw_protocol_list(f, chunks[0]);

        // Draw preview/splash area
        self.draw_preview(f, chunks[1]);
    }
}

impl BrowserScreen {
    /// Draw the protocol selection list.
    fn draw_protocol_list(&mut self, f: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self.protocols
            .iter()
            .map(|p| {
                let available = Launcher::is_available(p);
                let status = if available { "●" } else { "○" };
                let style = if available {
                    Style::default().fg(self.theme.color2) // Green
                } else {
                    Style::default().fg(self.theme.color8) // Dim
                };

                let content = Line::from(vec![
                    Span::styled(format!(" {} ", status), style),
                    Span::styled(p.icon, Style::default().fg(self.theme.color6)),
                    Span::raw(" "),
                    Span::styled(
                        p.name,
                        if available {
                            Style::default().fg(self.theme.foreground)
                        } else {
                            Style::default().fg(self.theme.color8)
                        },
                    ),
                ]);
                ListItem::new(content)
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Networks ")
                    .border_style(Style::default().fg(self.theme.color6)),
            )
            .highlight_style(
                Style::default()
                    .bg(self.theme.color8)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("▶ ");

        // Note: Can't use garnish with stateful widgets, render directly
        f.render_stateful_widget(list, area, &mut self.list_state);
    }

    /// Draw the preview/splash area.
    fn draw_preview(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),      // Splash art
                Constraint::Length(6),   // Protocol info
            ])
            .split(area);

        // Splash art
        let splash_text = Text::raw(self.splash_art);
        let splash = Paragraph::new(splash_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" SWITCHBOARD ")
                    .border_style(Style::default().fg(self.theme.color4)),
            )
            .style(Style::default().fg(self.theme.color6));

        let garnished_splash = splash.garnish(HalfShadow::default());
        f.render_widget(garnished_splash, chunks[0]);

        // Protocol info or quote
        let info_content = if let Some(protocol) = self.selected_protocol() {
            let available = Launcher::is_available(protocol);
            let status_line = if available {
                Line::from(vec![
                    Span::styled("Status: ", Style::default().fg(self.theme.color8)),
                    Span::styled("Ready", Style::default().fg(self.theme.color2)),
                    Span::styled(" │ Press ", Style::default().fg(self.theme.color8)),
                    Span::styled("Enter", Style::default().fg(self.theme.color6)),
                    Span::styled(" to connect", Style::default().fg(self.theme.color8)),
                ])
            } else {
                Line::from(vec![
                    Span::styled("Status: ", Style::default().fg(self.theme.color8)),
                    Span::styled("Not installed", Style::default().fg(self.theme.color1)),
                    Span::styled(" │ ", Style::default().fg(self.theme.color8)),
                    Span::styled(
                        format!("nix-shell -p {}", protocol.nix_package),
                        Style::default().fg(self.theme.color3),
                    ),
                ])
            };

            Text::from(vec![
                Line::from(vec![
                    Span::styled(protocol.icon, Style::default().fg(self.theme.color6)),
                    Span::raw(" "),
                    Span::styled(
                        protocol.name,
                        Style::default().fg(self.theme.foreground).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!(" ({})", protocol.binary),
                        Style::default().fg(self.theme.color8),
                    ),
                ]),
                Line::from(""),
                Line::from(Span::styled(
                    protocol.description,
                    Style::default().fg(self.theme.color7),
                )),
                Line::from(""),
                status_line,
            ])
        } else {
            Text::from(vec![
                Line::from(Span::styled(
                    self.quote,
                    Style::default().fg(self.theme.color5).add_modifier(Modifier::ITALIC),
                )),
            ])
        };

        let info = Paragraph::new(info_content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(self.theme.color8)),
            )
            .wrap(Wrap { trim: true });

        f.render_widget(info, chunks[1]);
    }
}
