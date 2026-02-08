use std::sync::OnceLock;

use rat_widget::statusline::{StatusLine, StatusLineState};
use ratatui::style::Stylize;
use ratatui::widgets::StatefulWidget;
use ratatui::{buffer::Buffer, layout::Constraint, style::Style};

use crate::ui::components::DumbComponent;
use crate::ui::{AppState, layout::Layout};

static LENGTHS: OnceLock<[u16; 2]> = OnceLock::new();
pub struct StatusBar(StatusLineState);

impl StatusBar {
    pub fn render(&mut self, area: Layout, buf: &mut Buffer) {
        StatusLine::new()
            .layout([
                Constraint::Length(0),
                Constraint::Length(LENGTHS.get().unwrap_or(&[8, 8])[0]),
                Constraint::Length(LENGTHS.get().unwrap_or(&[8, 8])[1]),
            ])
            .section_styles([
                Style::new().white().on_blue(),
                Style::new().white().on_black(),
            ])
            .render(area.status_bar, buf, &mut self.0);
    }
    pub fn new(app_state: AppState) -> Self {
        let mut status_line_state = StatusLineState::new();
        let status_line_1 = format!(" {}/{} ", app_state.owner, app_state.repo);
        let status_line_2 = format!("Logged in as {}", app_state.current_user.green());
        LENGTHS
            .set([status_line_2.len() as u16, status_line_1.len() as u16])
            .unwrap();
        status_line_state.status(1, &status_line_2);
        status_line_state.status(2, status_line_1.as_str());
        status_line_state.status(3, status_line_1.as_str());
        Self(status_line_state)
    }
}

impl DumbComponent for StatusBar {
    fn render(&mut self, area: Layout, buf: &mut Buffer) {
        self.render(area, buf);
    }
}
