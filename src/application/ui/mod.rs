/*
use std::time::Duration;

use symbols::line;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Cell, LineGauge, Paragraph, Row, Table};
use tui::{symbols, Frame};
use tui_logger::TuiLoggerWidget;

pub mod title;
pub use title::*;

use super::actions::Actions;
use super::state::AppState;
*/

use crossterm::event::Event;
use eyre::Result;
use std::error::Error;
use std::io;
use tui::backend::CrosstermBackend;
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders};
use tui::Terminal;
use tui_textarea::{Input, Key, TextArea};

fn validate(textarea: &mut TextArea) -> bool {
  if let Err(err) = textarea.lines()[0].parse::<f64>() {
    textarea.set_style(Style::default().fg(Color::LightRed));
    textarea.set_block(Block::default().borders(Borders::ALL).title(format!("ERROR: {}", err)));
    false
  } else {
    textarea.set_style(Style::default().fg(Color::LightGreen));
    textarea.set_block(Block::default().borders(Borders::ALL).title("OK"));
    true
  }
}

use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::Frame;

use crate::application::Application;

pub mod cli;
pub use cli::*;

pub fn draw<B>(rect: &mut Frame<B>, app: &mut Application) -> std::io::Result<()>
where
  B: Backend,
{
  let layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Min(1), Constraint::Length(3), ].as_slice());
  let chunks = layout.split(rect.size());
  let mut is_valid = validate(&mut app.cli_textarea);
  let cli_widget = app.cli_textarea.widget();
  rect.render_widget(cli_widget, chunks[1]);
  /*
  match crossterm::event::read()?.into() {
    Input { key: Key::Esc, .. } => {
      app.should_continue = false;
    },
    Input { key: Key::Enter, .. } if is_valid => {
      app.should_continue = false;
    },
    Input {
      key: Key::Char('m'),
      ctrl: true,
      ..
    }
    | Input { key: Key::Enter, .. } => {},
    input => {
      // TextArea::input returns if the input modified its text
      if app.cli_textarea.input(input) {
        is_valid = validate(&mut app.cli_textarea);
      }
    },
  }
  */
  /*
  // Fill the terminal and check our constraints.
  let size = rect.size();
  check_size(&size);

  // Vertical layout
  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints(
      [
        Constraint::Min(10),
        Constraint::Length(3),
        Constraint::Length(12),
        Constraint::Length(3),
      ]
      .as_ref(),
    )
    .split(size);

  // Body & Help
  let body_chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Min(20), Constraint::Length(32)].as_ref())
    .split(chunks[0]);

  // This is where we'll display the main text output.
  let body = draw_body(app.is_loading(), app.state());
  rect.render_widget(body, body_chunks[0]);

  // Some command help, etc.
  let help = draw_help(app.actions());
  rect.render_widget(help, body_chunks[1]);

  // Duration LineGauge
  if let Some(duration) = app.state().duration() {
    let duration_block = draw_duration(duration);
    rect.render_widget(duration_block, chunks[1]);
  }

  // Logs
  let logs = draw_logs();
  rect.render_widget(logs, chunks[2]);

  // Title.  I'll probably get rid of this.
  let title = draw_title();
  rect.render_widget(title, chunks[3]);
  */
  Ok(())
}

/*
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::io;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders};
use tui::Terminal;
use tui_textarea::{Input, Key, TextArea};

fn validate(textarea: &mut TextArea) -> bool {
    if let Err(err) = textarea.lines()[0].parse::<f64>() {
        textarea.set_style(Style::default().fg(Color::LightRed));
        textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("ERROR: {}", err)),
        );
        false
    } else {
        textarea.set_style(Style::default().fg(Color::LightGreen));
        textarea.set_block(Block::default().borders(Borders::ALL).title("OK"));
        true
    }
}

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut textarea = TextArea::default();
    textarea.set_cursor_line_style(Style::default());
    let layout =
        Layout::default().constraints([Constraint::Length(3), Constraint::Min(1)].as_slice());
    let mut is_valid = validate(&mut textarea);

    loop {
        term.draw(|f| {
            let chunks = layout.split(f.size());
            let widget = textarea.widget();
            f.render_widget(widget, chunks[0]);
        })?;

        match crossterm::event::read()?.into() {
            Input { key: Key::Esc, .. } => break,
            Input {
                key: Key::Enter, ..
            } if is_valid => break,
            Input {
                key: Key::Char('m'),
                ctrl: true,
                ..
            }
            | Input {
                key: Key::Enter, ..
            } => {}
            input => {
                // TextArea::input returns if the input modified its text
                if textarea.input(input) {
                    is_valid = validate(&mut textarea);
                }
            }
        }
    }

    disable_raw_mode()?;
    crossterm::execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    term.show_cursor()?;

    println!("Input: {:?}", textarea.lines()[0]);
    Ok(())
}
*/
