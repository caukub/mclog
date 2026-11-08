pub mod plugin;
pub mod port;
pub mod server;

pub struct StaticAnalyzer<'a> {
    lines: Vec<&'a str>,
}

impl<'a> StaticAnalyzer<'a> {
    pub fn new(lines: Vec<&'a str>) -> Self {
        Self { lines }
    }
}
