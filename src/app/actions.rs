#[derive(Debug, Clone)]
pub enum Action {
    FetchIssues,
    ApplyRegexFilter,
    ApplyLabelFilter,
    SelectIssue(usize),
    Quit,
}
