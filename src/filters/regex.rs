#[derive(Clone, Debug, Default)]
pub struct RegexFilter {
    pub query: String,
}

impl RegexFilter {
    pub fn is_active(&self) -> bool {
        !self.query.trim().is_empty()
    }
}
