use tui_textarea::TextArea;


// TODO: make textarea its own struct
// and add the util funcs to it
pub struct App<'a> {
  pub running: bool,
  pub titles: Vec<&'a str>,
  pub tab_index: usize,
  pub textarea: TextArea<'a>,
  pub text_active: bool,
}

impl<'a> Default for App<'a> {
  fn default() -> Self {
    Self {
      running: true,
      titles: vec!["day", "week", "month"],
      tab_index: 0,
      textarea: TextArea::default(),
      text_active: false,
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

}