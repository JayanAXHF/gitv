#[derive(Clone, Debug, Default)]
pub struct LabelFilter {
    pub labels: Vec<String>,
}

impl LabelFilter {
    pub fn is_active(&self) -> bool {
        !self.labels.is_empty()
    }
}
