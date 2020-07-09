use tui::{
    backend::CrosstermBackend,
    Frame
};
use std::io::Stdout;

pub type TUIFrame<'a> = Frame<'a, CrosstermBackend<Stdout>>;

pub trait Route {}

pub trait RenderTUI<T> where T: Route {
    fn render(&self, frame: &mut TUIFrame);
}
