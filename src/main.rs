mod app;
mod ui;
mod textwidget;
mod weather;

use crossterm_026 as crossterm;
use weather::fetch_weather;

use std::{error::Error, io};

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture}};
use ratatui::{Terminal, backend::CrosstermBackend};
use crate::app::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let stdout = io::stdout();
  let mut stdout = stdout.lock(); 

  enable_raw_mode()?;

  execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let mut app = App::new(fetch_weather().await?);
  
  app.text_widget.inactivate();
  let res = app.run(&mut terminal);

  disable_raw_mode()?;
  execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
  terminal.show_cursor()?;
  
  if let Err(err) = res {
    println!("{err:?}");
  }

  Ok(())
}