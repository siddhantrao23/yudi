mod app;

use std::{error::Error, io};

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture, Event, self, KeyCode, KeyEventKind, KeyModifiers}};
use tui::{
  Terminal, Frame, text::Line, symbols::DOT,
  layout::{Layout, Direction, Constraint}, 
  widgets::{Block, Tabs, Borders}, 
  style::{Style, Modifier, Color},
  backend::{CrosstermBackend, Backend},
};

use crate::app::App;

fn main() -> Result<(), Box<dyn Error>> {
  enable_raw_mode()?;

  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let app = App::new();
  let res = run_app(&mut terminal, app);

  disable_raw_mode()?;
  execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
  terminal.show_cursor()?;

  if let Err(err) = res {
    println!("{err:?}");
  }

  Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
  loop {
    terminal.draw(|f| render_ui(f, &app))?;

    if let Event::Key(key) = event::read()? {
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
      }
    }
  }
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
    .select(app.index)
    .block(Block::default().title("Tab Example")
  );
  f.render_widget(tabs, chunks[0]);

  let body = match app.index {
    0 => Block::default().title("Inner 1"),
    1 => Block::default().title("Inner 2").borders(Borders::ALL),
    2 => Block::default().title("Inner 3"),
    _ => unreachable!()
  };

  f.render_widget(body, chunks[1])
}