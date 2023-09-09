mod app;

use crossterm_026 as crossterm;

use std::{error::Error, io};

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture, Event, self, KeyCode, KeyEventKind, KeyModifiers}};
use ratatui::{
  Terminal, Frame, text::Line, symbols::{DOT, block},
  layout::{Layout, Direction, Constraint}, 
  widgets::{Block, Tabs, Borders}, 
  style::{Style, Modifier, Color},
  backend::{CrosstermBackend, Backend},
};
use tui_textarea::{Key, Input, TextArea};

use crate::app::App;

fn main() -> Result<(), Box<dyn Error>> {
  enable_raw_mode()?;

  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let mut app = App::new();
  app.textarea.set_block(Block::default().borders(Borders::ALL).title("Today's journal entry..."));
  inactivate(&mut app.textarea);
  let res = run_app(&mut terminal, app);

  disable_raw_mode()?;
  execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
  terminal.show_cursor()?;

  if let Err(err) = res {
    println!("{err:?}");
  }

  Ok(())
}
/*
      if key.kind == KeyEventKind::Press {
        match key.code {
          KeyCode::Char('q') => return Ok(()),
          KeyCode::Char('c') | KeyCode::Char('C') => {
            if key.modifiers == KeyModifiers::CONTROL {
                return Ok(());
            }
          }
          KeyCode::Right => app.next(),
          KeyCode::Tab => app.next(),
          KeyCode::Left => app.previous(),
          _ => {}
        }
      } */

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
  loop {
    terminal.draw(|f| render_ui(f, &app))?;

    match event::read()?.into() {
      Input {key: Key::Char('q'), ..}
      | Input {key: Key::Char('c'), ctrl: true, ..}
          => return Ok(()), 
      Input { key: Key::Esc, .. } => { 
        inactivate(&mut app.textarea);
        app.text_active = false;
      },
      Input {key: Key::Char('i'), ..} | Input {key: Key::Enter, ..} => {
        activate(&mut app.textarea);
        app.text_active = true;
      }
      Input {key: Key::Tab, ..} | Input {key: Key::Right, ..} => app.next(),
      Input {key: Key::Left, ..} => app.previous(),
      input => if app.text_active {
        app.textarea.input(input);
      },
    }
  }
}

fn inactivate(textarea: &mut TextArea) {
  textarea.set_cursor_line_style(Style::default());
  textarea.set_cursor_style(Style::default());
  let b = textarea.block().cloned().unwrap_or_else(|| Block::default().borders(Borders::ALL));
  textarea.set_block(
      b.style(Style::default().fg(Color::DarkGray))
  );
}

fn activate(textarea: &mut TextArea) {
  textarea.set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));
  textarea.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
  let b = textarea.block().cloned().unwrap_or_else(|| Block::default().borders(Borders::ALL));
  textarea.set_block(b.style(Style::default()));
}

fn render_ui<B: Backend>(f: &mut Frame<B>, app: &App) {
  let size = f.size();

  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
    .split(size);
  let block = Block::default();
  f.render_widget(block, size);
  
  let titles: Vec<Line> = app.titles.iter().cloned().map(Line::from).collect();
  let tabs = Tabs::new(titles)
    .divider(DOT)
    .highlight_style(
      Style::default().add_modifier(Modifier::BOLD).bg(Color::Green)
    )
    .select(app.tab_index)
    .block(Block::default().title("Tab Example")
  );
  f.render_widget(tabs, chunks[0]);

  let body = match app.tab_index {
    0 => Block::default().title("Inner 1"),
    1 => Block::default().title("Inner 2").borders(Borders::ALL),
    2 => Block::default().title("Inner 3"),
    _ => unreachable!()
  };

  f.render_widget(app.textarea.widget(), chunks[1])
}