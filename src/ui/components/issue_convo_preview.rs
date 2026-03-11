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

use crate::{
    errors::AppError,
    ui::{
        Action,
        components::{Component, help::HelpElementKind, issue_conversation::render_markdown},
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

#[derive(Default)]
pub struct IssueConvoPreview {
    action_tx: Option<tokio::sync::mpsc::Sender<Action>>,
    body: Option<Arc<str>>,
    area: Rect,
    paragraph_state: ParagraphState,
    index: usize,
    focus: FocusFlag,
}

impl IssueConvoPreview {
    pub fn new() -> Self {
        Self::default()
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
        let rendered = render_markdown(body, area.width.saturating_sub(2).into(), 2).lines;
        let para = rat_widget::paragraph::Paragraph::new(rendered).block(
            Block::default()
                .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM)
                .title(format!("[{}] Issue Body", self.index))
                .merge_borders(ratatui::symbols::merge::MergeStrategy::Exact)
                .border_style(get_border_style(&self.paragraph_state)),
        );
        para.render(area, buf, &mut self.paragraph_state);
    }

    fn render_issue_list_preview(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::LEFT | Borders::BOTTOM)
            .padding(Padding::horizontal(1))
            .title(format!("[{}] Nearby Issues", self.index))
            .merge_borders(ratatui::symbols::merge::MergeStrategy::Exact)
            .border_style(get_border_style(&self.paragraph_state));

        if self.issue_ids.is_empty() {
            let para = ratatui::widgets::Paragraph::new("No nearby issues available.").block(block);
            para.render(area, buf);
            return;
        }

        let items = {
            let pool = self.issue_pool.read().expect("issue pool lock poisoned");
            self.issue_ids
                .iter()
                .map(|issue_id| {
                    let issue = pool.get_issue(*issue_id);
                    if Some(issue.number) == self.open_number {
                        let mut lines = build_issue_list_lines(issue, &pool, false, false);
                        if let Some(first_line) = lines.first_mut() {
                            first_line.spans.insert(
                                0,
                                Span::styled(
                                    "* ",
                                    Style::new().fg(Color::Green).add_modifier(Modifier::BOLD),
                                ),
                            );
                        }
                        ListItem::new(lines)
                    } else {
                        build_issue_list_item(issue, &pool, false, false)
                    }
                })
                .collect::<Vec<_>>()
        };

        self.sync_selected_issue();

        let list = TuiList::new(items)
            .block(block)
            .highlight_style(Style::new().add_modifier(Modifier::BOLD | Modifier::REVERSED));
        StatefulWidget::render(list, area, buf, &mut self.list_state);
    }

    fn selected_issue_id(&self) -> Option<IssueId> {
        let selected = self.list_state.selected()?;
        self.issue_ids.get(selected).copied()
    }

    fn sync_selected_issue(&mut self) {
        let selected = self.selected_number.and_then(|number| {
            let pool = self.issue_pool.read().expect("issue pool lock poisoned");
            self.issue_ids
                .iter()
                .position(|issue_id| pool.get_issue(*issue_id).number == number)
        });
        self.list_state.select(selected);
    }

    async fn open_selected_issue(&mut self) -> Result<(), AppError> {
        let Some(issue_id) = self.selected_issue_id() else {
            return Ok(());
        };
        let Some(action_tx) = self.action_tx.clone() else {
            return Ok(());
        };

        let (number, labels, preview_seed, conversation_seed) = {
            let pool = self.issue_pool.read().expect("issue pool lock poisoned");
            let issue = pool.get_issue(issue_id);
            (
                issue.number,
                issue.labels.clone(),
                IssuePreviewSeed::from_ui_issue(issue, &pool),
                crate::ui::components::issue_conversation::IssueConversationSeed::from_ui_issue(
                    issue, &pool,
                ),
            )
        };

        self.open_number = Some(number);
        self.selected_number = Some(number);
        self.sync_selected_issue();
        action_tx
            .send(Action::SelectedIssue { number, labels })
            .await?;
        action_tx
            .send(Action::SelectedIssuePreview { seed: preview_seed })
            .await?;
        action_tx
            .send(Action::IssueListPreviewUpdated {
                issue_ids: self.issue_ids.clone(),
                selected_number: number,
            })
            .await?;
        action_tx
            .send(Action::EnterIssueDetails {
                seed: conversation_seed,
            })
            .await?;
        Ok(())
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
