pub mod dynamic;
pub mod static_analyzer;

pub struct Analyzer {
    line_limit: Option<usize>,
}

impl Analyzer {
    pub fn new(line_limit: Option<usize>) -> Self {
        Self { line_limit }
    }
}
