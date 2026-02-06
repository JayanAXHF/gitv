use crate::filters::{LabelFilter, RegexFilter};
use crate::models::{Issue, Repo};

#[derive(Default)]
pub struct AppState {
    pub selected_repo: Option<Repo>,
    pub issues: Vec<Issue>,
    pub regex_filter: RegexFilter,
    pub label_filter: LabelFilter,
}
