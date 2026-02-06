use ratatui::{buffer::Buffer, layout::Rect};

use crate::ui::{Action, layout::Layout};

pub mod issue_detail;
pub mod issue_list;
pub mod label_filter;
pub mod search_bar;
pub mod status_bar;

pub trait Component {
    fn render(&mut self, area: Layout, buf: &mut Buffer);
    fn register_action_tx(&mut self, action_tx: tokio::sync::mpsc::Sender<Action>) {
        let _ = action_tx;
    }
    fn handle_event(&mut self, event: Action) {
        let _ = event;
    }
}
