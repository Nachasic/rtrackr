use crate::{
    AppState,
    tui::{
        style as STYLE,
        components::{ StatefulTUIComponent, TUIFrame, ToWidgets },
        utils::*
    }
};
use tui::{
    layout::Rect,
    widgets::{ Paragraph, Block, Borders },
};

pub struct ActiveWindowInfo {
    display: Option<DisplayArchetype>
}

impl From<&AppState> for ActiveWindowInfo {
    fn from(state: &AppState) -> Self {
        Self {
            display: Option::<DisplayArchetype>::from(state)
        }
    }
}

impl StatefulTUIComponent for ActiveWindowInfo {
    fn tick(&mut self, app_state: &AppState) {
        self.display = Option::<DisplayArchetype>::from(app_state);
    }

    fn render(&self, frame: &mut TUIFrame, chunk: Rect) {
        let window_info_text = (&self.display).to_widgets();
        let block = Block::default()
            .title(" Active window info ")
            .title_style(*STYLE::STYLE_TEXT_HEADER)
            .borders(Borders::TOP);
        let widget = Paragraph::new(window_info_text.iter())
            .block(block);

        frame.render_widget(widget, chunk)
    }
}