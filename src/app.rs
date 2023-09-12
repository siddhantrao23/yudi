use std::io;

use crossterm_026::event;
use ratatui::{Terminal, prelude::Backend};
use tui_textarea::{Input, Key};
use crate::{ui::render, textwidget::TextWidget};

pub struct App<'a> {
  pub running: bool,
  pub titles: Vec<&'a str>,
  pub tab_index: usize,
  pub text_widget: TextWidget<'a>,
}

impl<'a> Default for App<'a> {
  fn default() -> Self {
    Self {
      running: true,
      titles: vec!["day", "week", "month"],
      tab_index: 0,
      text_widget: TextWidget::new(),
    }
  }
}

impl<'a> App<'a> {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn next(&mut self) {
    self.tab_index = (self.tab_index + 1) % self.titles.len();
  }

  pub fn previous(&mut self) {
    if self.tab_index > 0 {
      self.tab_index -= 1;
    } else {
      self.tab_index = self.titles.len() - 1;
    }
  }

  pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
      terminal.draw(|f| render(f, &self))?;

      match event::read()?.into() {
        Input {key: Key::Char('q'), ..}
        | Input {key: Key::Char('c'), ctrl: true, ..}
            => return Ok(()), 
        Input { key: Key::Esc, .. } | Input {key: Key::Enter, ..} => { 
          // save journal data
          self.text_widget.inactivate();
        },
        Input {key: Key::Tab, ..} | Input {key: Key::Right, ..} => self.next(),
        Input {key: Key::Left, ..} => self.previous(),
        Input {key: Key::Char('i'), ..} => {
          self.text_widget.activate();
        },
        input => if self.text_widget.active {
          self.text_widget.textarea.input(input);
        },
      }
    }
  }
}