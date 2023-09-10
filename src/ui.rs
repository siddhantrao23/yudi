use ratatui::{
  Frame, text::Line, symbols::DOT,
  layout::{Layout, Direction, Constraint}, 
  widgets::{Block, Tabs}, 
  style::{Style, Modifier, Color},
  backend::Backend,
};

use crate::app::App;

pub fn render<B: Backend>(f: &mut Frame<B>, app: &App) {
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
