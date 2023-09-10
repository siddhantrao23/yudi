mod app;
mod weather;
mod util;

use crossterm_026 as crossterm;
use util::{inactivate, activate};
use weather::request_weather;

use std::{error::Error, io};

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture, self}};
use ratatui::{
  Terminal, Frame, text::Line, symbols::DOT,
  layout::{Layout, Direction, Constraint}, 
  widgets::{Block, Tabs, Borders}, 
  style::{Style, Modifier, Color},
  backend::{CrosstermBackend, Backend},
};
use tui_textarea::{Key, Input};

use crate::app::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let stdout = io::stdout();
  let mut stdout = stdout.lock(); 

  enable_raw_mode()?;

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
  request_weather().await?;

  if let Err(err) = res {
    println!("{err:?}");
  }

  Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
  loop {
    terminal.draw(|f| render_ui(f, &app))?;

    match event::read()?.into() {
      Input {key: Key::Char('q'), ..}
      | Input {key: Key::Char('c'), ctrl: true, ..}
          => return Ok(()), 
      Input { key: Key::Esc, .. } | Input {key: Key::Enter, ..} => { 
        // save journal data
        inactivate(&mut app.textarea);
        app.text_active = false;
      },
      Input {key: Key::Tab, ..} | Input {key: Key::Right, ..} => app.next(),
      Input {key: Key::Left, ..} => app.previous(),
      Input {key: Key::Char('i'), ..} => {
        activate(&mut app.textarea);
        app.text_active = true;
      },
      input => if app.text_active {
        app.textarea.input(input);
      },
    }
  }
}



fn render_ui<B: Backend>(f: &mut Frame<B>, app: &App) {
  let size = f.size();

  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Length(1), Constraint::Length(4)].as_ref())
    .split(size);

  let titles: Vec<Line> = app.titles.iter().cloned().map(Line::from).collect();
  let tabs = Tabs::new(titles)
    .divider(DOT)
    .highlight_style(
      Style::default().add_modifier(Modifier::BOLD).bg(Color::Green)
    )
    .select(app.tab_index)
    .block(Block::default());
  f.render_widget(tabs, chunks[0]);

  let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
    .split(chunks[1]);
  
  match app.tab_index {
    0 => f.render_widget(app.textarea.widget(), chunks[1]),
    1 => f.render_widget(app.textarea.widget(), chunks[1]),
    2 => {},
    _ => unreachable!()
  };
}