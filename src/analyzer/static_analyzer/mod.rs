pub mod plugin;
pub mod port;
pub mod server;

pub struct StaticAnalyzer<'a> {
    lines: &'a [&'a str],
}

impl<'a> StaticAnalyzer<'a> {
    pub fn new(lines: &'a [&'a str]) -> Self {
        Self { lines }
    }
}
