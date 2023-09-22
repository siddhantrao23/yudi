use tui_textarea::TextArea;
use ratatui::{style::{Style, Modifier, Color}, widgets::{Block, Borders, Padding}};

pub struct TextWidget<'a> {
  pub textarea: TextArea<'a>,
  pub active: bool,
}

impl<'a> TextWidget<'a> {
  pub fn new() -> Self {
    let mut text_widget = TextWidget {textarea: TextArea::default(), active: false};
    text_widget.textarea.set_block(
      Block::default()
        .borders(Borders::ALL)
        .title("journal")
    );

    return text_widget;      
  } 

  pub fn activate(&mut self) {
    self.textarea.set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));
    self.textarea.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    let b = self.textarea.block().cloned().unwrap_or_else(|| Block::default().borders(Borders::ALL));
    self.textarea.set_block(b.style(Style::default()));

    self.active = true;
  }
  
  pub fn inactivate(&mut self) {
    self.textarea.set_cursor_line_style(Style::default());
    self.textarea.set_cursor_style(Style::default());
    let b = self.textarea.block().cloned().unwrap_or_else(|| Block::default().borders(Borders::ALL));
    self.textarea.set_block(
        b.style(Style::default().fg(Color::DarkGray))
    );

    self.active = false;
  }
}