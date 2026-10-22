use crate::analyzer::dynamic::DynamicAnalyzer;

pub trait LogAnalyzer {
    fn has_line(&self, s: &str) -> bool;
    fn has_line_nocase(&self, s: &str) -> bool;
}

impl LogAnalyzer for DynamicAnalyzer {
    fn has_line(&self, s: &str) -> bool {
        for line in self.lines.iter().take(self.line_limit) {
            if line.contains(s) {
                return true;
            }
        }
        false
    }

    fn has_line_nocase(&self, s: &str) -> bool {
        for line in self.lines.iter().take(self.line_limit) {
            if line.to_lowercase().contains(&s.to_lowercase()) {
                return true;
            }
        }
        false
    }
}

// TODO callback!!!!!
