use std::io;

use crossterm_026::event;
use ratatui::{Terminal, prelude::Backend, widgets::ListState};
use tui_textarea::{Input, Key};
use crate::{ui::render, textwidget::TextWidget};

pub struct StatefulList<T> {
  pub items: Vec<T>,
  pub state: ListState,
}

impl<T> StatefulList<T> {
  fn next(&mut self) {
    let i = match self.state.selected() {
      Some(i) => {
        if i >= self.items.len() - 1 { 0 }
        else { i + 1 }
      },
      None => 0,
    };
    self.state.select(Some(i));
  }

  fn previous(&mut self) {
    let i = match self.state.selected() {
      Some(i) => {
        if i == 0 { self.items.len() - 1 }
        else { i - 1 }
      },
      None => 0,
    };
    self.state.select(Some(i));
  }
}

pub struct App<'a> {
  pub running: bool,
  pub titles: Vec<&'a str>,
  pub tab_index: usize,
  pub text_widget: TextWidget<'a>,
  pub weather_widget: StatefulList<(String, usize)>,
}

impl<'a> App<'a> {
  pub fn new(history: Vec<(String, usize)>) -> Self {
    Self {
      running: true,
      titles: vec!["day", "week", "month"],
      tab_index: 0,
      text_widget: TextWidget::new(),
      weather_widget: StatefulList {
        items: history,
        state: ListState::default(),
      }
    }
  }

  pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
      terminal.draw(|f| render(f, self))?;

      let input = event::read()?.into();
      if self.text_widget.active == false {
        match input {
          Input {key: Key::Char('q'), ..} | 
            Input {key: Key::Char('c'), ctrl: true, ..} => {
              return Ok(())
          }, 
          Input {key: Key::Up, ..} => self.weather_widget.previous(),
          Input {key: Key::Down, ..} => self.weather_widget.next(),
          Input {key: Key::Char('i'), ..} => {
            self.text_widget.activate();
          },
          _ => {},
        }
      } else {
        match input {
          Input {key: Key::Char('q'), ..} => {
            self.text_widget.textarea.input(input);
          }
          Input {key: Key::Char('c'), ctrl: true, ..} => return Ok(()), 
          Input { key: Key::Esc, .. } => { 
            // save journal data
            self.text_widget.inactivate();
          },
          Input {key: Key::Char('i'), ..} => {
            self.text_widget.textarea.input(input);
          },
          input => if self.text_widget.active {
            self.text_widget.textarea.input(input);
          },
        }
      }
    }
  }
}