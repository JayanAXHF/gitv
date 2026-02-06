#[derive(Clone, Debug)]
pub struct Config {
    pub page_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self { page_size: 50 }
    }
}

pub mod defaults;
