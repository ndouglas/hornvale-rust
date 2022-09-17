use crossterm::event::Event;
use eyre::Result;
use std::error::Error;
use std::io;
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Paragraph};
use tui::Terminal;
use tui_logger::TuiLoggerWidget;
use tui_textarea::{Input, Key, TextArea};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::Frame;

use crate::application::Application;
use crate::application::ApplicationState;

pub fn validate(textarea: &mut TextArea) -> bool {
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

pub fn draw<B>(rect: &mut Frame<B>, app: &mut Application) -> std::io::Result<()>
where
  B: Backend,
{
  let layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Min(1), Constraint::Length(12), Constraint::Length(3)].as_slice());
  let chunks = layout.split(rect.size());

  // Hornvale!
  let body = draw_body(app.is_busy, &mut app.state);
  rect.render_widget(body, chunks[0]);

  // Logs
  let logs = draw_logs();
  rect.render_widget(logs, chunks[1]);

  // CLI
  let mut is_valid = validate(&mut app.cli_textarea);
  let cli_widget = app.cli_textarea.widget();
  rect.render_widget(cli_widget, chunks[2]);

  // Return!
  Ok(())
}

fn draw_body<'a>(is_busy: bool, state: &ApplicationState) -> Paragraph<'a> {
  let initialized_text = if state.is_initialized() {
    "Initialized"
  } else {
    "Not Initialized !"
  };
  let loading_text = if is_busy { "Loading..." } else { "" };
  let sleep_text = if let Some(sleeps) = state.count_sleep() {
    format!("Sleep count: {}", sleeps)
  } else {
    String::default()
  };
  let tick_text = if let Some(ticks) = state.count_tick() {
    format!("Tick count: {}", ticks)
  } else {
    String::default()
  };
  Paragraph::new(vec![
    Spans::from(Span::raw(initialized_text)),
    Spans::from(Span::raw(loading_text)),
    Spans::from(Span::raw(sleep_text)),
    Spans::from(Span::raw(tick_text)),
  ])
  .style(Style::default().fg(Color::LightCyan))
  .alignment(Alignment::Left)
  .block(
    Block::default()
      // .title("Body")
      .borders(Borders::ALL)
      .style(Style::default().fg(Color::White))
      .border_type(BorderType::Plain),
  )
}

fn draw_logs<'a>() -> TuiLoggerWidget<'a> {
  TuiLoggerWidget::default()
    .style_error(Style::default().fg(Color::Red))
    .style_debug(Style::default().fg(Color::Green))
    .style_warn(Style::default().fg(Color::Yellow))
    .style_trace(Style::default().fg(Color::Gray))
    .style_info(Style::default().fg(Color::Blue))
    .block(
      Block::default()
        .title("Logs")
        .border_style(Style::default().fg(Color::White).bg(Color::Black))
        .borders(Borders::ALL),
    )
    .style(Style::default().fg(Color::White).bg(Color::Black))
}
