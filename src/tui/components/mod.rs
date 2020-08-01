pub mod active_window_info;
use tui::{
    backend::CrosstermBackend,
    Frame,
    layout::Rect,
};
use std::io::Stdout;
use crate::AppState;
use crate::event::Key;

pub type TUIFrame<'a> = Frame<'a, CrosstermBackend<Stdout>>;

pub trait StatefulTUIComponent {
    fn handle_key(&mut self, event: Key) {}
    fn tick(&mut self, app_state: &AppState);
    fn render(&self, frame: &mut TUIFrame, chunk: Rect);
}

pub trait ToWidgets {
    type Res: std::iter::IntoIterator;
    fn to_widgets(&self) -> Self::Res;
}
