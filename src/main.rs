use std::io;
use std::time::Duration;

use chrono::Local;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use fern::Dispatch;
use log::{info, LevelFilter};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};

fn setup_logger() -> Result<(), fern::InitError> {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] {}: {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(LevelFilter::Info)
        .chain(fern::log_file("bookie.log")?)
        .apply()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logger().expect("Failed to setup logger");

    // --- terminal init ---
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // sample rows
    let rows = vec![
        String::from("Alpha row here"),
        String::from("Bravo row here"),
        String::from("Charlie row here"),
    ];
    let mut selected: usize = 0;
    let mut running = true;

    // initial render BEFORE reading input
    terminal.draw(|f| {
        // split: top (details), middle (list), bottom (command/prompt)
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(7), // details area
                    Constraint::Min(3),    // list area
                    Constraint::Length(1), // command / status
                ]
                .as_ref(),
            )
            .split(f.area());

        // top: details about selected item
        let detail = {
            let title = format!("Title: {}", rows[selected]);
            let rating = "Rating: ★★★★☆";
            let notes = "Notes: (none)";
            Paragraph::new(vec![
                Line::from(Span::raw(title)),
                Line::from(Span::raw("Author: Unknown".to_string())),
                Line::from(Span::raw(rating)),
                Line::from(Span::raw("")),
                Line::from(Span::raw(notes)),
            ])
            .block(Block::default().borders(Borders::ALL).title("Book Details"))
        };
        f.render_widget(detail, chunks[0]);

        // middle: list with selection marker at first character (we'll prefix ">" at column 0)
        let items: Vec<ListItem> = rows
            .iter()
            .enumerate()
            .map(|(i, t)| {
                let mut line = String::new();
                if i == selected {
                    line.push('>'); // selection marker first char
                    line.push(' ');
                } else {
                    line.push(' ');
                    line.push(' ');
                }
                line.push_str(t);
                ListItem::new(Line::from(Span::raw(line)))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Library"))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">");
        f.render_widget(list, chunks[1]);

        // bottom: command/status line
        let status = Paragraph::new(Span::raw(": add / rate / quit"))
            .block(Block::default().borders(Borders::TOP));
        f.render_widget(status, chunks[2]);
    })?;

    info!(target: "Bookie", "App started. Selected index = {}", selected);

    // input loop (non-blocking poll with timeout to allow redraws if desired)
    while running {
        // wait for an event, but with small timeout so we can redraw if needed
        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                info!(target: "Bookie", "Key event: {:?}", code);

                match code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        info!(target: "Bookie", "Quitting");
                        running = false;
                    }
                    KeyCode::Down => {
                        selected = (selected + 1) % rows.len();
                    }
                    KeyCode::Up => {
                        selected = (selected + rows.len() - 1) % rows.len();
                    }
                    KeyCode::Enter => {
                        info!(target: "Bookie", "Enter pressed on index {}", selected);
                        // placeholder: handle selection
                    }
                    _ => {}
                }

                // redraw after state change
                terminal.draw(|f| {
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(
                            [
                                Constraint::Length(7),
                                Constraint::Min(3),
                                Constraint::Length(1),
                            ]
                            .as_ref(),
                        )
                        .split(f.area());

                    let detail = Paragraph::new(vec![
                        Line::from(Span::raw(format!("Title: {}", rows[selected]))),
                        Line::from(Span::raw("Author: Unknown".to_string())),
                        Line::from(Span::raw("Rating: ★★★★☆")),
                        Line::from(Span::raw("")),
                        Line::from(Span::raw("Notes: (none)")),
                    ])
                    .block(Block::default().borders(Borders::ALL).title("Book Details"));
                    f.render_widget(detail, chunks[0]);

                    let items: Vec<ListItem> = rows
                        .iter()
                        .enumerate()
                        .map(|(i, t)| {
                            let mut line = String::new();
                            if i == selected {
                                line.push('>');
                                line.push(' ');
                            } else {
                                line.push(' ');
                                line.push(' ');
                            }
                            line.push_str(t);
                            ListItem::new(Line::from(Span::raw(line)))
                        })
                        .collect();

                    let list = List::new(items)
                        .block(Block::default().borders(Borders::ALL).title("Library"))
                        .highlight_style(Style::default().fg(Color::Black).bg(Color::White))
                        .highlight_symbol(">");
                    f.render_widget(list, chunks[1]);

                    let status = Paragraph::new(Span::raw(": add / rate / quit"))
                        .block(Block::default().borders(Borders::TOP));
                    f.render_widget(status, chunks[2]);
                })?;
            }
        } else {
            // timeout expired — could do periodic tasks here
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    info!(target: "Bookie", "App exited");
    Ok(())
}
