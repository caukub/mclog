pub mod log;
pub mod plugin;
pub mod port;
pub mod server;

pub struct DynamicAnalyzer {
    lines: Vec<String>,
    line_limit: usize,
}

impl DynamicAnalyzer {
    pub fn new(lines: Vec<String>, line_limit: Option<usize>) -> Self {
        let line_limit = line_limit.unwrap_or(usize::MAX);

        Self { lines, line_limit }
    }
}
