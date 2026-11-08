use std::sync::LazyLock;

use regex::Regex;

use super::StaticAnalyzer;

static SERVER_PORT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Starting Minecraft server on .*:(\d+)").unwrap());

static QUERY_PORT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Query running on .*:(\d+)").unwrap());

static RCON_PORT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"RCON running on .*:(\d+)").unwrap());

pub trait StaticPortAnalyzer {
    fn server(&self, line_limit: Option<usize>) -> Option<u16>;
    fn query(&self, line_limit: Option<usize>) -> Option<u16>;
    fn rcon(&self, line_limit: Option<usize>) -> Option<u16>;
}

impl StaticPortAnalyzer for StaticAnalyzer<'_> {
    fn server(&self, line_limit: Option<usize>) -> Option<u16> {
        let line_limit = line_limit.unwrap_or(usize::MAX);

        for line in self.lines.iter().take(line_limit) {
            if let Some(caps) = SERVER_PORT_RE.captures(line) {
                return caps.get(1)?.as_str().parse().ok();
            }
        }
        None
    }

    fn query(&self, line_limit: Option<usize>) -> Option<u16> {
        let line_limit = line_limit.unwrap_or(usize::MAX);

        for line in self.lines.iter().take(line_limit) {
            if let Some(caps) = QUERY_PORT_RE.captures(line) {
                return caps.get(1)?.as_str().parse().ok();
            }
        }
        None
    }

    fn rcon(&self, line_limit: Option<usize>) -> Option<u16> {
        let line_limit = line_limit.unwrap_or(usize::MAX);

        for line in self.lines.iter().take(line_limit) {
            if let Some(caps) = RCON_PORT_RE.captures(line) {
                return caps.get(1)?.as_str().parse().ok();
            }
        }
        None
    }
}

// TODO plugin/mod ports
