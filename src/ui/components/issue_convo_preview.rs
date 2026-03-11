use async_trait::async_trait;
use rat_widget::{
    event::{HandleEvent, Regular},
    focus::{FocusBuilder, FocusFlag, HasFocus},
    paragraph::ParagraphState,
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, StatefulWidget, Widget},
};
use std::sync::{Arc, RwLock};
use textwrap::wrap;

use crate::{
    errors::AppError,
    ui::{
        Action,
        components::{Component, help::HelpElementKind, issue_conversation::render_markdown},
        issue_data::{IssueId, UiIssuePool},
        layout::Layout,
        utils::get_border_style,
    },
};

pub const HELP: &[HelpElementKind] = &[
    crate::help_text!("Issue Conversation Help"),
    crate::help_keybind!("Up/Down", "select issue body/comment entry"),
    crate::help_keybind!("PageUp/PageDown/Home/End", "scroll message body pane"),
    crate::help_keybind!("t", "toggle timeline events"),
    crate::help_keybind!("f", "toggle fullscreen body view"),
    crate::help_keybind!("C", "close selected issue"),
    crate::help_keybind!("l", "copy link to selected message"),
    crate::help_keybind!("Enter (popup)", "confirm close reason"),
    crate::help_keybind!("Ctrl+P", "toggle comment input/preview"),
    crate::help_keybind!("e", "edit selected comment in external editor"),
    crate::help_keybind!("r", "add reaction to selected comment"),
    crate::help_keybind!("R", "remove reaction from selected comment"),
    crate::help_keybind!("Ctrl+Enter / Alt+Enter", "send comment"),
    crate::help_keybind!("Esc", "exit fullscreen / return to issue list"),
];

pub struct IssueConvoPreview {
    action_tx: Option<tokio::sync::mpsc::Sender<Action>>,
    body: Option<Arc<str>>,
    area: Rect,
    current: Option<IssueId>,
    paragraph_state: ParagraphState,
    index: usize,
    focus: FocusFlag,
}

impl IssueConvoPreview {
    pub fn new() -> Self {
        Self {
            action_tx: None,
            current: None,
            body: None,
            index: 0,
            focus: FocusFlag::default(),
            area: Rect::default(),
            paragraph_state: ParagraphState::default(),
        }
    }

    pub fn render(&mut self, area: Layout, buf: &mut Buffer) {
        let block_template = Block::default()
            .borders(Borders::LEFT | Borders::BOTTOM)
            .border_style(get_border_style(&self.paragraph_state));

        self.area = area.mini_convo_preview;
        let Some(ref body) = self.body else {
            let para =
                ratatui::widgets::Paragraph::new("Select an issue to preview the conversation")
                    .block(
                        block_template
                            .title(format!("[{}] Issue Conversation]", self.index))
                            .merge_borders(ratatui::symbols::merge::MergeStrategy::Exact),
                    );
            para.render(area.mini_convo_preview, buf);
            return;
        };
        let body_str = wrap(
            &body,
            area.mini_convo_preview.width.saturating_sub(2) as usize,
        )
        .join("\n");
        let rendered = render_markdown(
            &body_str,
            area.mini_convo_preview.width.saturating_sub(2).into(),
            2,
        )
        .lines;
        let para = rat_widget::paragraph::Paragraph::new(rendered).block(
            Block::default()
                .borders(Borders::LEFT | Borders::BOTTOM)
                .title(format!("[{}] Issue Body", self.index))
                .merge_borders(ratatui::symbols::merge::MergeStrategy::Exact)
                .border_style(get_border_style(&self.paragraph_state)),
        );
        para.render(area.mini_convo_preview, buf, &mut self.paragraph_state);
    }
}

#[async_trait(?Send)]
impl Component for IssueConvoPreview {
    fn render(&mut self, area: Layout, buf: &mut Buffer) {
        self.render(area, buf);
    }

    fn register_action_tx(&mut self, action_tx: tokio::sync::mpsc::Sender<Action>) {
        self.action_tx = Some(action_tx);
    }

    async fn handle_event(&mut self, event: Action) -> Result<(), AppError> {
        match event {
            Action::AppEvent(ref event) => {
                self.paragraph_state.handle(event, Regular);
            }
            Action::ChangeIssueBodyPreview(body) => {
                self.body = Some(body);
            }
            _ => {}
        }
        Ok(())
    }

    fn should_render(&self) -> bool {
        true
    }

    fn is_animating(&self) -> bool {
        false
    }

    fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    fn set_global_help(&self) {
        if let Some(action_tx) = &self.action_tx {
            let _ = action_tx.try_send(Action::SetHelp(HELP));
        }
    }
}

impl HasFocus for IssueConvoPreview {
    fn build(&self, builder: &mut FocusBuilder) {
        let tag = builder.start(self);
        builder.widget(&self.paragraph_state);
        builder.end(tag);
    }

    fn focus(&self) -> FocusFlag {
        self.focus.clone()
    }

    fn area(&self) -> Rect {
        self.area
    }
}
