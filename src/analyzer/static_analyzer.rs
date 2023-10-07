use super::Plugin;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref SPIGOT_PLUGIN_REGEX: Regex = Regex::new(r"\[([^\]]*)\] Loading (.*) v(.*)")
        .unwrap_or_else(|e| {
            panic!("Failed to create 'SPIGOT_PLUGIN_REGEX': {}", e);
        });
    static ref PAPER_PLUGIN_REGEX: Regex =
        Regex::new(r"\[([^\]]*)\] Loading server plugin (.*) v(.*)").unwrap_or_else(|e| {
            panic!("Failed to create 'PAPER_PLUGIN_REGEX': {}", e);
        });
    static ref PORT_REGEX: Regex = Regex::new(r".*:(\d+)").unwrap_or_else(|e| {
        panic!("Failed to create 'PORT_REGEX': {}", e);
    });
    static ref MINECRAFT_VERSION_REGEX: Regex =
        Regex::new(r"Starting minecraft server version (.*)").unwrap_or_else(|e| {
            panic!("Failed to create 'MINECRAFT_VERSION_REGEX': {}", e);
        });
}

pub struct StaticAnalyzer;

impl StaticAnalyzer {
    pub fn plugin_bukkit(line: &str) -> Option<Plugin> {
        let is_spigot_plugin_regex_match = SPIGOT_PLUGIN_REGEX.is_match(line);
        let is_paper_plugin_regex_match = PAPER_PLUGIN_REGEX.is_match(line);

        if is_paper_plugin_regex_match || is_spigot_plugin_regex_match {
            let captures = match is_paper_plugin_regex_match {
                true => PAPER_PLUGIN_REGEX.captures(line).unwrap(),
                false => SPIGOT_PLUGIN_REGEX.captures(line).unwrap(),
            };

            let plugin_prefix = captures.get(1).unwrap().as_str();
            let plugin_name = captures.get(2).unwrap().as_str();

            let plugin_prefix_cleared = PluginNameNormalizer(plugin_prefix).clear();
            let plugin_name_cleared = PluginNameNormalizer(plugin_name).clear();

            if plugin_prefix_cleared == plugin_name_cleared {
                let plugin_version = captures.get(3).unwrap().as_str();

                return Some(Plugin {
                    name: plugin_name.to_string(),
                    version: plugin_version.to_string(),
                });
            }
        }
        None
    }

    pub fn port(name: String, mut line: &str, must_contain: String) -> Option<(String, u16)> {
        if let Some(split) = line.split_once("]") {
            line = split.1;
        };

        if line.to_lowercase().contains(&must_contain.to_lowercase()) && PORT_REGEX.is_match(line) {
            let captures = PORT_REGEX.captures(line).unwrap();
            let port = captures.get(1).unwrap().as_str().parse::<u16>().unwrap();

            return Some((name, port));
        }
        None
    }

    pub fn vanilla_port(line: &str, must_contain: &str) -> Option<u16> {
        if line.contains(must_contain) && PORT_REGEX.is_match(line) {
            let captures = PORT_REGEX.captures(line).unwrap();
            let port = captures.get(1).unwrap().as_str().parse::<u16>().unwrap();

            return Some(port);
        }

        None
    }

    pub fn noproxy_server_version(line: &str) -> Option<String> {
        if MINECRAFT_VERSION_REGEX.is_match(line) {
            let captures = MINECRAFT_VERSION_REGEX.captures(line).unwrap();
            let version = captures.get(1).unwrap().as_str().to_string();
            return Some(version);
        }
        None
    }

    pub fn leaked_plugin(line: &str) -> Option<String> {
        let sus = [
            "directleaks",
            "blackspigot",
            "nulled",
            "plugin integrity has been compromised",
        ]; // ඞඞඞ

        if sus.iter().any(|s| line.to_lowercase().contains(s)) {
            return Some(line.to_string());
        }

        None
    }
}

struct PluginNameNormalizer<'a>(&'a str);

impl PluginNameNormalizer<'_> {
    fn clear(&self) -> String {
        self.0.to_lowercase().replace(['-', '_', ' '], "")
    }
}
