use crate::models::{Label, User};

#[derive(Clone, Debug, Default)]
pub struct Issue {
    pub id: u64,
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub labels: Vec<Label>,
    pub author: Option<User>,
    pub state: IssueState,
}

#[derive(Clone, Debug, Default)]
pub enum IssueState {
    #[default]
    Open,
    Closed,
}
