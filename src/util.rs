use ratatui::{style::{Style, Modifier, Color}, widgets::{Block, Borders}};
use tui_textarea::TextArea;

pub fn inactivate(textarea: &mut TextArea) {
  textarea.set_cursor_line_style(Style::default());
  textarea.set_cursor_style(Style::default());
  let b = textarea.block().cloned().unwrap_or_else(|| Block::default().borders(Borders::ALL));
  textarea.set_block(
      b.style(Style::default().fg(Color::DarkGray))
  );
}

pub fn activate(textarea: &mut TextArea) {
  textarea.set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));
  textarea.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
  let b = textarea.block().cloned().unwrap_or_else(|| Block::default().borders(Borders::ALL));
  textarea.set_block(b.style(Style::default()));
}