use crate::{
    record_store::{ Archetype, ProductivityStatus },
    state::{ AppState },
    classifier::Classifiable,
    tui::{
        components::*,
        routes::{ Route, RenderTUI, TUIFrame },
    }
};
use tui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph },
};

#[derive(Debug, Clone)]
struct DisplayArchetype {
    archetype: Archetype,
    productivity: ProductivityStatus,
}

impl Classifiable for DisplayArchetype {
    fn get_archetype(&self) -> &Archetype {
        &self.archetype
    }

    fn assign_productivity(&mut self, productivity: ProductivityStatus) {
        self.productivity = productivity;
    }
}

impl From<&AppState> for Option<DisplayArchetype> {
    fn from(app: &AppState) -> Self {
        app.get_current_archetype()
            .clone()
            .map(|arch| {
                let mut display = DisplayArchetype {
                    archetype: arch,
                    productivity: ProductivityStatus::Neutral
                };
                app.classifier().classify(&mut display);
                display
            })
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct RouteMain;
impl Route for RouteMain {}

impl RenderTUI<RouteMain> for AppState {
    fn render(&self, frame: &mut TUIFrame) {
        let display = Option::<DisplayArchetype>::from(self);
        let arch_opt = self.get_current_archetype();
        let arch_widget = TUIWindowInfo {
            archetype: arch_opt
        };
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
            .split(frame.size());

        let window_info_text = arch_widget.to_widgets();
        let block = Block::default()
            .title(" Active window info ")
            .borders(Borders::ALL);
        let window_notification = Paragraph::new(window_info_text.iter())
            .block(block.clone())
            .alignment(Alignment::Left);
        
        frame.render_widget(window_notification, chunks[0]);

        let block = Block::default().title(" Block 2 ").borders(Borders::ALL);
        frame.render_widget(block, chunks[1]);
    }
}
