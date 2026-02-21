use ratatui::{
    style::Style,
    symbols::{self},
    widgets::{Block, Borders, Padding, Paragraph, Widget, WidgetRef},
};

use crate::ui::components::toast::ToastType;

#[derive(Debug, Clone)]
pub struct Toast {
    pub message: String,
    pub type_: ToastType,
}

impl Toast {
    pub fn new(message: &str, type_: ToastType) -> Self {
        Self {
            message: message.to_string(),
            type_,
        }
    }
}

impl WidgetRef for Toast {
    fn render_ref(&self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        const PADDING: u16 = 1;
        let paragraph = Paragraph::new(self.message.as_str()).block(
            Block::default()
                .borders(Borders::LEFT | Borders::RIGHT)
                .border_set(symbols::border::QUADRANT_OUTSIDE)
                .padding(Padding::uniform(PADDING))
                .border_style(Style::default().fg(self.type_.into())),
        );
        paragraph.render(area, buf);
    }
}
