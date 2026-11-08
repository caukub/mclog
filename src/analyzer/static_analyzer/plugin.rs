use std::sync::LazyLock;

use regex::Regex;

use super::StaticAnalyzer;

pub trait StaticPluginAnalyzer<'a> {
    fn plugins(&self, line_limit: Option<usize>) -> Vec<Plugin<'a>>;
}

static PLUGIN_LOAD_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[(?P<plugin>[^\]]+)\] Loading (?:server plugin )?(?P<name>\S+) v(?P<version>\S+)")
        .unwrap()
});

static PLUGIN_ENABLE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[(?P<plugin>[^\]]+)\] Enabling (?P<name>\S+) v(?P<version>\S+)").unwrap()
});

#[derive(Debug)]
pub struct Plugin<'a> {
    pub plugin: &'a str,
    pub name: &'a str,
    pub version: Option<&'a str>,
    pub enabled: bool,
}

impl<'a> StaticPluginAnalyzer<'a> for StaticAnalyzer<'a> {
    fn plugins(&self, line_limit: Option<usize>) -> Vec<Plugin<'a>> {
        let line_limit = line_limit.unwrap_or(usize::MAX);

        let mut plugins = Vec::new();

        for line in self.lines.iter().copied().take(line_limit) {
            if let Some(caps) = PLUGIN_LOAD_RE.captures(line) {
                let (Some(plugin), Some(name)) = (caps.name("plugin"), caps.name("name")) else {
                    continue;
                };

                plugins.push(Plugin {
                    plugin: plugin.as_str(),
                    name: name.as_str(),
                    version: caps.name("version").map(|v| v.as_str()),
                    enabled: false,
                });
            } else if let Some(caps) = PLUGIN_ENABLE_RE.captures(line) {
                let Some(name) = caps.name("name") else {
                    continue;
                };

                if let Some(p) = plugins.iter_mut().find(|p| p.name == name.as_str()) {
                    p.enabled = true;
                }
            }
        }

        plugins
    }
}

// TODO jary co se nezaply
// TODO Velocity, Folia
