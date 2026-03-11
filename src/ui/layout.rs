use ratatui::layout::{Rect, Spacing};
use ratatui_macros::{horizontal, vertical};

#[derive(Debug, Clone, Copy)]
pub struct Layout {
    pub status_bar: Rect,
    pub mini_convo_preview: Rect,
    pub main_content: Rect,
    pub label_list: Rect,
    pub text_search: Rect,
    pub status_dropdown: Rect,
    pub issue_preview: Rect,
    pub label_search: Rect,
    pub title_bar: Rect,
}

impl Layout {
    pub fn new(area: Rect) -> Self {
        let [title_bar, main, status_bar] = vertical![==1, *=1, ==1].areas(area);
        let [left, right] = horizontal![==66%, *=1].areas(main);
        let [mini_convo_preview, label_list, issue_preview] = vertical![==60%, *=1, *=1]
            .spacing(Spacing::Overlap(1))
            .areas(right);
        let [text_search, bottom_search, main_content] = vertical![==3, ==3, *=1].areas(left);
        let [label_search, status_dropdown] = horizontal![*=1, ==30%].areas(bottom_search);
        Self {
            status_dropdown,
            mini_convo_preview,
            title_bar,
            status_bar,
            main_content,
            label_list,
            label_search,
            text_search,
            issue_preview,
        }
    }

    pub fn fullscreen(area: Rect) -> Self {
        Self {
            mini_convo_preview: area,
            status_bar: area,
            main_content: area,
            label_list: area,
            text_search: area,
            status_dropdown: area,
            issue_preview: area,
            label_search: area,
            title_bar: area,
        }
    }

    pub fn areas(&self) -> [Rect; 4] {
        [
            self.title_bar,
            self.main_content,
            self.label_list,
            self.issue_preview,
        ]
    }
}
