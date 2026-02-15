use rat_widget::statusline_stacked::StatusLineStacked;
use ratatui::buffer::Buffer;
use ratatui::style::{Style, Stylize};
use ratatui::widgets::Widget;
use ratatui_macros::{line, span};

use crate::app::cli::VERSION_MESSAGE;
use crate::ui::components::DumbComponent;
use crate::ui::layout::Layout;

pub struct TitleBar;

impl TitleBar {
    pub fn render(&mut self, area: Layout, buf: &mut Buffer) {
        let ss = StatusLineStacked::new()
            .start(
                line![span!(" gitv ").style(Style::new().black().on_blue()),],
                " ",
            )
            .end(
                line![
                    span!("Version").magenta(),
                    " ",
                    span!(" {} ", VERSION_MESSAGE).black().on_magenta().bold()
                ],
                " ",
            );
        ss.render(area.title_bar, buf);
    }
}

impl DumbComponent for TitleBar {
    fn render(&mut self, area: Layout, buf: &mut Buffer) {
        self.render(area, buf);
    }
}
