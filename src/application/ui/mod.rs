use specs::prelude::*;
use specs::shrev::EventChannel;
use std::collections::VecDeque;
use textwrap::wrap as wrap_text;
use tui::backend::Backend;
use tui::layout::Alignment;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Paragraph};
use tui::Frame;

use tui_logger::TuiLoggerWidget;
use tui_textarea::TextArea;

use crate::application::state::State;
use crate::application::Application;
use crate::event::InputEvent;

pub fn send_input(textarea: &mut TextArea, state: &mut State) {
  if let Some(input) = textarea.lines().first() {
    state
      .ecs
      .write_resource::<EventChannel<InputEvent>>()
      .single_write(InputEvent {
        input: input.to_owned(),
      });
    textarea.delete_line_by_head();
    textarea.delete_line_by_end();
  }
}

pub fn validate(textarea: &mut TextArea) -> bool {
  if textarea.lines()[0].len() == 0 {
    textarea.set_style(Style::default().fg(Color::LightRed));
    textarea.set_block(
      Block::default()
        .borders(Borders::ALL)
        .title(format!("ERROR: {}", "pls enter a command")),
    );
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
  let body_height = chunks[0].height - 2;
  let body_width = chunks[0].width - 2;
  let body = draw_body(&mut app.state, body_height, body_width);
  rect.render_widget(body, chunks[0]);

  // Logs
  let logs = draw_logs();
  rect.render_widget(logs, chunks[1]);

  // CLI
  let _is_valid = validate(&mut app.cli_textarea);
  let cli_widget = app.cli_textarea.widget();
  rect.render_widget(cli_widget, chunks[2]);

  // Return!
  Ok(())
}

fn draw_body<'a>(state: &'a mut State<'_>, height: u16, width: u16) -> Paragraph<'a> {
  let mut spans = vec![];
  let mut lines = state
    .messages
    .iter()
    .take(height.into())
    .map(|string| wrap_text(&string, width as usize).iter().rev().map(|cow| cow.to_string()).collect::<Vec<String>>())
    .flatten()
    .collect::<VecDeque<String>>();
  while spans.len() < height as usize {
    if let Some(string) = lines.pop_front() {
      spans.push(Spans::from(Span::raw(string)));
    } else {
      spans.push(Spans::from(Span::raw("")));
    }
  }
  spans.reverse();
  Paragraph::new(spans)
    .style(Style::default().fg(Color::LightCyan))
    .alignment(Alignment::Left)
    .block(
      Block::default()
        .title("Hornvale")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .border_type(BorderType::Rounded),
    )
}

fn draw_logs<'a>() -> TuiLoggerWidget<'a> {
  TuiLoggerWidget::default()
    .style_error(Style::default().fg(Color::Yellow))
    .style_debug(Style::default().fg(Color::Yellow))
    .style_warn(Style::default().fg(Color::Yellow))
    .style_trace(Style::default().fg(Color::Yellow))
    .style_info(Style::default().fg(Color::Yellow))
    .block(
      Block::default()
        .title("Logs")
        .border_style(Style::default().fg(Color::White).bg(Color::Black))
        .borders(Borders::ALL)
        .border_type(BorderType::Thick),
    )
    .style(Style::default().fg(Color::White).bg(Color::Black))
}
