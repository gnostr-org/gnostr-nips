use ratatui::{
    backend::CrosstermBackend,
    prelude::{Alignment, Constraint, Direction, Layout, Modifier, Style},
    style::Color,
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
    Terminal,
    widgets::ListState,
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    cursor::{Hide, Show},
};
use std::{
    io::{self, stdout, Write},
    error::Error,
};

use rust_embed::RustEmbed;
use sha2::Digest;
use termimad::MadSkin;
use tracing_subscriber::prelude::SubscriberExt;

#[derive(RustEmbed)]
#[folder = "."] // Adjust the folder if NIPs are in a subdirectory
#[exclude = "*.rs"]
#[exclude = "*.md.html"]
#[exclude = "*.toml"]
#[exclude = "*.lock"]
#[exclude = "*.sh"]
#[exclude = ".*"] // Exclude hidden files like .gitignore
#[exclude = "Cargo.toml"]
#[exclude = "Cargo.lock"]
#[exclude = "build.rs"]
#[exclude = "dist-workspace.toml"]
#[exclude = "target/*"]
#[exclude = ".github/*"]
struct NipsEmbed;


struct AppState {
    nips: Vec<String>, // List of nip filenames
    selected_nip_index: usize,
    nip_content: String,
    list_state: ListState, // Added for List scrolling
}

impl AppState {
    fn new() -> Self {
        let nip_files: Vec<String> = NipsEmbed::iter()
            .filter(|name| name.ends_with(".md"))
            .map(|name| name.to_string())
            .collect();

        let initial_nip_content = if nip_files.is_empty() {
            "No NIPs found.".to_string()
        } else {
            Self::load_nip_content(&nip_files[0])
        };

        let mut list_state = ListState::default();
        list_state.select(Some(0)); // Select the first item by default

        AppState {
            nips: nip_files,
            selected_nip_index: 0,
            nip_content: initial_nip_content,
            list_state, // Initialize list_state
        }
    }

    fn load_nip_content(nip_filename: &str) -> String {
        if let Some(file) = NipsEmbed::get(nip_filename) {
            String::from_utf8_lossy(file.data.as_ref()).to_string()
        } else {
            format!("Error: NIP '{}' not found.", nip_filename)
        }
    }

    fn select_next_nip(&mut self) {
        if !self.nips.is_empty() {
            let next_index = (self.selected_nip_index + 1) % self.nips.len();
            self.selected_nip_index = next_index;
            self.list_state.select(Some(next_index)); // Update ListState
            self.nip_content = Self::load_nip_content(&self.nips[self.selected_nip_index]);
        }
    }

    fn select_previous_nip(&mut self) {
        if !self.nips.is_empty() {
            let prev_index = (self.selected_nip_index + self.nips.len() - 1) % self.nips.len();
            self.selected_nip_index = prev_index;
            self.list_state.select(Some(prev_index)); // Update ListState
            self.nip_content = Self::load_nip_content(&self.nips[self.selected_nip_index]);
        }
    }
}

fn ui<B: ratatui::backend::Backend>(f: &mut Frame<B>, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Percentage(10), // Header/NIP list
            Constraint::Percentage(90), // Content
        ])
        .split(f.size());

    // NIP List Pane
    let list_block = Block::default()
        .title("NIPs")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);

    let nip_items: Vec<ListItem> = app_state.nips.iter().enumerate().map(|(i, nip_name)| {
        let style = if i == app_state.selected_nip_index {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        ListItem::new(Span::styled(nip_name, style))
    }).collect();

    let nip_list = List::new(nip_items)
        .block(list_block)
        .highlight_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ")
        .state(&app_state.list_state); // Use ListState for highlighting and scrolling

    f.render_widget(nip_list, chunks[0]);

    // Content Pane
    let content_block = Block::default()
        .title(format!("NIP: {}", app_state.nips.get(app_state.selected_nip_index).unwrap_or(&"None".to_string())))
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);

    let content_lines: Vec<Line> = app_state.nip_content.lines()
        .map(|line| Line::from(Span::raw(line.to_string())))
        .collect();
    let content_text = Text::from(content_lines);

    let content_paragraph = Paragraph::new(content_text)
        .block(content_block)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Left)
        .wrap(ratatui::widgets::Wrap { trim: false });

    f.render_widget(content_paragraph, chunks[1]);
}

fn run_ratatui_app() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?; 
    let mut stdout = stdout.lock();

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app_state = AppState::new();

    loop {
        terminal.draw(|f| ui(f, &app_state))?;

        match event::read()? {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Char('q') => break, // Quit
                KeyCode::Char('j') | KeyCode::ArrowDown => {
                    app_state.select_next_nip();
                }
                KeyCode::Char('k') | KeyCode::ArrowUp => {
                    app_state.select_previous_nip();
                }
                _ => {}
            },
            Event::Resize(_, _) => {
                // Handle terminal resize if necessary
            }
            _ => {}
        }
    }

    execute!(terminal.backend_mut(), LeaveAlternateScreen, Show)?;
    terminal.show_cursor()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let subscriber = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stdout))
        .with(tracing_subscriber::EnvFilter::new("info"));
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber");

    tracing::info!("Starting NIP Browser TUI...");

    run_ratatui_app()?;

    tracing::info!("NIP Browser TUI exited.");
    Ok(())
}

// Helper function to remove .md extension for routing, if needed.
fn remove_md_extension(filename: &str) -> &str {
    filename.strip_suffix(".md").unwrap_or(filename)
}

// Placeholder for markdown_to_html
fn markdown_to_html(markdown: &str) -> String {
    markdown.to_string()
}

// Placeholder for calculate_sha256
fn calculate_sha256(data: &[u8]) -> String {
    let mut hasher = sha2::Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

// Placeholder for make_skin
fn make_skin() -> termimad::MadSkin {
    let mut skin = termimad::MadSkin::default();
    skin.set_headers_fg(termimad::crossterm::style::Color::Yellow);
    skin
}
