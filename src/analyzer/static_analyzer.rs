use super::Plugin;
use regex::Regex;
use std::sync::LazyLock;

static SPIGOT_PLUGIN_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[([^]]*)] Loading (.*) v(.*)").unwrap_or_else(|e| {
        panic!("Failed to create 'SPIGOT_PLUGIN_REGEX': {}", e);
    })
});
static PAPER_PLUGIN_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[([^]]*)] Loading server plugin (.*) v(.*)").unwrap_or_else(|e| {
        panic!("Failed to create 'PAPER_PLUGIN_REGEX': {}", e);
    })
});

// This regex matches 4 or more digits because reserved ports shouldn't be used
static PORT_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r":(\d{4,5})").unwrap_or_else(|e| {
        panic!("Failed to create 'PORT_REGEX': {}", e);
    })
});

static NOCOLON_PORT_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(\d{4,5})").unwrap_or_else(|e| {
        panic!("Failed to create 'NOCOLON_PORT_REGEX': {}", e);
    })
});

static MINECRAFT_VERSION_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"Starting minecraft server version (.*)").unwrap_or_else(|e| {
        panic!("Failed to create 'MINECRAFT_VERSION_REGEX': {}", e);
    })
});

pub struct StaticAnalyzer;

impl StaticAnalyzer {
    pub fn plugin_bukkit(line: &str) -> Option<Plugin> {
        let is_spigot_plugin_regex_match = SPIGOT_PLUGIN_REGEX.is_match(line);
        let is_paper_plugin_regex_match = PAPER_PLUGIN_REGEX.is_match(line);

        if is_paper_plugin_regex_match || is_spigot_plugin_regex_match {
            let captures = match is_paper_plugin_regex_match {
                true => PAPER_PLUGIN_REGEX.captures(line)?,
                false => SPIGOT_PLUGIN_REGEX.captures(line)?,
            };

            let plugin_prefix = captures.get(1)?.as_str();
            let plugin_name = captures.get(2)?.as_str();

            let plugin_prefix_cleared = PluginNameNormalizer(plugin_prefix).clear();
            let plugin_name_cleared = PluginNameNormalizer(plugin_name).clear();

            if plugin_prefix_cleared == plugin_name_cleared {
                let plugin_version = captures.get(3)?.as_str();

                return Some(Plugin {
                    name: plugin_name.to_string(),
                    version: plugin_version.to_string(),
                });
            }
        }
        None
    }

    pub fn plugin_bungeecord() {
        todo!()
    }

    pub fn plugin_velocity() {
        todo!()
    }

    pub fn mod_fabric() {
        todo!()
    }

    pub fn mod_forge() {
        todo!()
    }

    pub fn port(name: String, line: &str, must_contain: String) -> Option<(String, u16)> {
        if line.to_lowercase().contains(&must_contain.to_lowercase()) {
            if PORT_REGEX.is_match(line) {
                let captures = PORT_REGEX.captures(line)?;

                let capture = captures.get(1)?;

                let port = capture.as_str().parse::<u16>().ok()?;

                return Some((name, port));
            }

            if NOCOLON_PORT_REGEX.is_match(line) {
                let captures = NOCOLON_PORT_REGEX.captures(line)?;

                let capture = captures.get(1)?;

                let port = capture.as_str().parse::<u16>().ok()?;

                return Some((name, port));
            }
        }
        None
    }

    pub fn vanilla_port(line: &str, must_contain: &str) -> Option<u16> {
        if line.contains(must_contain) && PORT_REGEX.is_match(line) {
            let port = PORT_REGEX.captures(line)?.get(1)?.as_str();
            let port = port.parse::<u16>().ok()?;

            return Some(port);
        }

        None
    }

    pub fn noproxy_server_version(line: &str) -> Option<String> {
        if MINECRAFT_VERSION_REGEX.is_match(line) {
            let captures = MINECRAFT_VERSION_REGEX.captures(line)?;
            let version = captures.get(1)?.as_str().to_string();
            return Some(version);
        }
        None
    }

    pub fn leaked_plugin(line: &str) -> Option<String> {
        let sus = [
            "directleaks",
            "blackspigot",
            "nulled",
            "spigotunlocked",
            "plugin integrity has been compromised",
            "cracked",
        ];

        let allowed = ["crackshot"];

        if sus.iter().any(|s| line.to_lowercase().contains(s))
            && !allowed.iter().any(|allowed| line.contains(allowed))
        {
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
