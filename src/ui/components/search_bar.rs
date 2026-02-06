use rat_widget::text_input;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Block, widgets::StatefulWidget};

use crate::ui::{Action, components::Component, layout::Layout};

#[derive(Default)]
pub struct TextSearch {
    state: rat_widget::text_input::TextInputState,
    action_tx: Option<tokio::sync::mpsc::Sender<Action>>,
}

impl TextSearch {
    fn render_w(&mut self, area: Rect, buf: &mut Buffer) {
        let text_input = rat_widget::text_input::TextInput::new().block(
            Block::bordered()
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title("Search"),
        );
        text_input.render(area, buf, &mut self.state);
    }
}

impl Component for TextSearch {
    fn render(&mut self, area: Layout, buf: &mut Buffer) {
        let area = area.text_search;
        self.render_w(area, buf);
    }

    fn register_action_tx(&mut self, action_tx: tokio::sync::mpsc::Sender<Action>) {
        self.action_tx = Some(action_tx);
    }
    fn handle_event(&mut self, event: Action) {
        match event {
            Action::AppEvent(ref event) => {
                text_input::handle_events(&mut self.state, true, event);
            }
            _ => {}
        }
    }
}
