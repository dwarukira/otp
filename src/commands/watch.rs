use std::{
    io,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use anyhow::Result;
use arboard::Clipboard;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

use ratatui::{
    backend::CrosstermBackend,
    prelude::*,
    widgets::*,
    Terminal,
};

use crate::services::{otp, storage};

struct App {
    selected: usize,
}

pub fn run() -> Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();

    execute!(
        stdout,
        EnterAlternateScreen,
    )?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    let mut app = App {
        selected: 0,
    };

    loop {
        let accounts = storage::load()?;

        if !accounts.is_empty() && app.selected >= accounts.len() {
            app.selected = accounts.len() - 1;
        }

        terminal.draw(|frame| {
            draw(
                frame,
                &accounts,
                app.selected,
            );
        })?;

        if event::poll(
            Duration::from_millis(250),
        )? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q')
                    | KeyCode::Esc => {
                        break;
                    }

                    KeyCode::Down => {
                        if app.selected + 1 < accounts.len() {
                            app.selected += 1;
                        }
                    }

                    KeyCode::Up => {
                        if app.selected > 0 {
                            app.selected -= 1;
                        }
                    }

                    KeyCode::Enter
                    | KeyCode::Char('c') => {
                        if !accounts.is_empty() {
                            copy_selected(
                                &accounts,
                                app.selected,
                            )?;
                        }
                    }

                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )?;

    Ok(())
}

fn copy_selected(
    accounts: &[crate::models::account::Account],
    selected: usize,
) -> Result<()> {
    let account = &accounts[selected];

    let code =
        otp::generate(&account.secret)?;

    let mut clipboard =
        Clipboard::new()?;

    clipboard.set_text(code)?;

    Ok(())
}

fn draw(
    frame: &mut Frame,
    accounts: &[crate::models::account::Account],
    selected: usize,
) {
    let remaining = remaining_seconds();

    let layout = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(1),
    ])
    .split(frame.area());

    let progress =
        ((30 - remaining) as f64 / 30.0)
            * 100.0;

    let gauge = Gauge::default()
        .block(
            Block::bordered()
                .title(" TOTP Timer ")
                .border_style(
                    Style::default()
                        .fg(Color::Blue),
                ),
        )
        .percent(progress as u16)
        .label(format!(
            "{}s remaining",
            remaining
        ));

    frame.render_widget(
        gauge,
        layout[0],
    );

    let rows: Vec<Row> = accounts
        .iter()
        .enumerate()
        .map(|(idx, account)| {
            let code = otp::generate(
                &account.secret,
            )
            .unwrap_or_else(|_| {
                "ERROR".to_string()
            });

            let code_color =
                match remaining {
                    0..=5 => Color::Red,
                    6..=10 => Color::Yellow,
                    _ => Color::Green,
                };

            let row_style =
                if idx == selected {
                    Style::default()
                        .bg(Color::DarkGray)
                        .fg(Color::White)
                        .add_modifier(
                            Modifier::BOLD,
                        )
                } else {
                    Style::default()
                };

            Row::new(vec![
                Cell::from(
                    account.name.clone(),
                )
                .style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(
                            Modifier::BOLD,
                        ),
                ),
                Cell::from(code)
                    .style(
                        Style::default()
                            .fg(code_color)
                            .add_modifier(
                                Modifier::BOLD,
                            ),
                    ),
            ])
            .style(row_style)
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ],
    )
    .header(
        Row::new(vec![
            "Account",
            "Code",
        ])
        .style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(
                    Modifier::BOLD,
                ),
        ),
    )
    .block(
        Block::bordered()
            .title(" OTP Codes ")
            .border_style(
                Style::default()
                    .fg(Color::Blue),
            ),
    )
    .column_spacing(2);

    frame.render_widget(
        table,
        layout[1],
    );

    let footer = Paragraph::new(
        format!(
            "↑↓ Select | Enter/C Copy OTP | q Quit | {} accounts",
            accounts.len()
        ),
    )
    .style(
        Style::default()
            .fg(Color::DarkGray),
    );

    frame.render_widget(
        footer,
        layout[2],
    );
}

fn remaining_seconds() -> u64 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    30 - (now % 30)
}