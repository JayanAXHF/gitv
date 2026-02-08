use octocrab::Page;

use crate::models::Issue;

#[derive(Debug, Clone)]
pub enum Action {
    FetchIssues,
    ApplyRegexFilter,
    ApplyLabelFilter,
    SelectIssue(usize),
    NewPage(Page<Issue>),
    Quit,
}
