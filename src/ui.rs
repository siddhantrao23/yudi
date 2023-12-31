
use ratatui::{
  Frame, text::Line,
  layout::{Layout, Direction, Constraint}, 
  widgets::{Block, List, ListItem, Padding}, 
  style::{Style, Modifier},
  backend::Backend,
};


use crate::app::App;

pub fn render<B: Backend>(f: &mut Frame<B>, app: &mut App) {
  let size = f.size();
  let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
    .split(size);

  let items: Vec<ListItem> = app.weather_widget.items
    .iter()
    .map(|i| {
      let mut lines = vec![Line::from(i.0.clone())];
      lines.push(Line::from("\n"));
      ListItem::new(lines).style(Style::default().add_modifier(Modifier::DIM))
  })
  .collect();

  let items = List::new(items)
    .block(Block::default().padding(Padding::new(10, 10, 2, 2)))
    .highlight_style(
      Style::default().add_modifier(Modifier::BOLD).remove_modifier(Modifier::DIM)
    )
    .highlight_symbol("•  ");

  f.render_stateful_widget(items, chunks[0], &mut app.weather_widget.state);
  f.render_widget(app.text_widget.textarea.widget(), chunks[1]);
}

/*
pub fn render<B: Backend>(f: &mut Frame<B>, app: &App) {
  // TODO: add the call to weather here and render the weather for week and day
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
    .margin(2)
    .split(chunks[1]);
  
  let curr_date = OffsetDateTime::now_local()
    .unwrap();


  let events = CalendarEventStore::today(
        Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Blue),
  );    

  let y = curr_date.year();
  let m = curr_date.month();
  let d = curr_date.date();

  match app.tab_index {
    0 => {
      f.render_widget(Monthly::new(d, events),chunks[0]);
      f.render_widget(app.text_widget.textarea.widget(), chunks[1]);
    },
    1 => {
      let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
      f.render_widget(Paragraph::new(text).wrap(Wrap { trim: true }), chunks[0]);
      f.render_widget(Paragraph::new("☀  🌤  ☁  ☁"),chunks[1]);
    },
    2 => {
      f.render_widget(Monthly::new(d, &events),chunks[0]);
      f.render_widget(Monthly::new(Date::from_calendar_date(y, m.next(), 1).unwrap(), &events), chunks[1]);
    },
    _ => unreachable!()
  };
}
 */