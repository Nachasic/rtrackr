use tui::{
    backend::CrosstermBackend,
    Frame
};
use std::io::Stdout;
use crate::AppState;

pub type TUIFrame<'a> = Frame<'a, CrosstermBackend<Stdout>>;

pub trait Route {}

pub trait RenderTUI<T> where T: Route {
    fn render(&self, frame: &mut TUIFrame);
}

pub trait StatefulTUIComponent: Route{
    fn tick(&mut self, app_state: &AppState);
    fn render(&self, frame: &mut TUIFrame);
}