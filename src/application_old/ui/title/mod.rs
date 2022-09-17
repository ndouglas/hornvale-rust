use tui::layout::Alignment;
use tui::style::{Color, Style};
use tui::widgets::{Block, BorderType, Borders, Paragraph};

pub fn draw_title<'a>() -> Paragraph<'a> {
  Paragraph::new("Hornvale")
    .style(Style::default().fg(Color::LightCyan))
    .alignment(Alignment::Center)
    .block(
      Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .border_type(BorderType::Plain),
    )
}
