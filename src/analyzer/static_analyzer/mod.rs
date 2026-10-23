pub mod plugin;
pub mod port;
pub mod server;

pub struct StaticAnalyzer {
    lines: Vec<String>,
}

impl StaticAnalyzer {
    pub fn new(lines: Vec<String>) -> Self {
        Self { lines }
    }
}
