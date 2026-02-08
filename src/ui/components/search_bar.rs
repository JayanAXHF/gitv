use async_trait::async_trait;
use rat_widget::{
    choice::{Choice, ChoiceState},
    event::{HandleEvent, Popup, Regular},
    focus::impl_has_focus,
    popup::Placement,
};
use ratatui::{
    buffer::Buffer,
    style::{Style, Stylize},
    widgets::{Block, StatefulWidget, Widget},
};
use ratatui_macros::span;

use crate::ui::{Action, components::Component, layout::Layout};

#[derive(Default)]
pub struct TextSearch {
    search_state: rat_widget::text_input::TextInputState,
    label_state: rat_widget::text_input::TextInputState,
    cstate: ChoiceState,
    action_tx: Option<tokio::sync::mpsc::Sender<Action>>,
}

impl TextSearch {
    fn render_w(&mut self, layout: Layout, buf: &mut Buffer) {
        let contents = (1..)
            .zip([span!("Open".green()), span!("Close".magenta())])
            .collect::<Vec<_>>();
        let text_input = rat_widget::text_input::TextInput::new().block(
            Block::bordered()
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title("Search"),
        );
        let label = rat_widget::text_input::TextInput::new().block(
            Block::bordered()
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title("Search Labels"),
        );
        let (widget, popup) = Choice::new()
            .items(contents)
            .popup_placement(Placement::Below)
            .popup_style(Style::default())
            .focus_style(Style::default())
            .select_style(Style::default())
            .button_style(Style::default())
            .style(Style::default())
            .select_marker('>')
            .into_widgets();
        let block = Block::bordered().border_type(ratatui::widgets::BorderType::Rounded);
        let binner = block.inner(layout.status_dropdown);
        block.render(layout.status_dropdown, buf);
        popup.render(layout.status_dropdown, buf, &mut self.cstate);
        widget.render(binner, buf, &mut self.cstate);
        text_input.render(layout.text_search, buf, &mut self.search_state);
        label.render(layout.label_search, buf, &mut self.label_state);
    }
}

impl_has_focus!(search_state, label_state, cstate for TextSearch);

#[async_trait(?Send)]
impl Component for TextSearch {
    fn render(&mut self, area: Layout, buf: &mut Buffer) {
        self.render_w(area, buf);
    }

    fn register_action_tx(&mut self, action_tx: tokio::sync::mpsc::Sender<Action>) {
        self.action_tx = Some(action_tx);
    }
    async fn handle_event(&mut self, event: Action) {
        match event {
            Action::AppEvent(ref event) => {
                self.label_state.handle(event, Regular);
                self.search_state.handle(event, Regular);
                self.cstate.handle(event, Popup);
            }
            _ => {}
        }
    }
}
