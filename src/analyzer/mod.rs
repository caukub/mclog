use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use self::static_analyzer::StaticAnalyzer;

pub mod dynamic;
pub mod static_analyzer;
pub mod template;

#[derive(Serialize, Debug, Copy, Clone)]
pub enum Platform {
    Vanilla,
    CraftBukkit,
    Spigot,
    Paper,
    Pufferfish,
    Purpur,
    Fabric,
    Forge,
    BungeeCord,
    Waterfall,
    Velocity,
}

impl PlatformDetails for Platform {
    fn name(&self) -> &'static str {
        match self {
            Platform::Vanilla => "vanilla",
            Platform::CraftBukkit => "craftbukkit",
            Platform::Spigot => "spigot",
            Platform::Paper => "paper",
            Platform::Pufferfish => "pufferfish",
            Platform::Purpur => "purpur",
            Platform::Fabric => "fabric",
            Platform::Forge => "forge",
            Platform::BungeeCord => "bungeecord",
            Platform::Waterfall => "waterfall",
            Platform::Velocity => "velocity",
        }
    }

    fn display_name(&self) -> &'static str {
        match self {
            Platform::Vanilla => "Vanilla",
            Platform::CraftBukkit => "CraftBukkit",
            Platform::Spigot => "Spigot",
            Platform::Paper => "Paper",
            Platform::Pufferfish => "Puferfish",
            Platform::Purpur => "Purpur",
            Platform::Fabric => "Fabric",
            Platform::Forge => "Forge",
            Platform::BungeeCord => "BungeeCord",
            Platform::Waterfall => "Waterfall",
            Platform::Velocity => "Velocity",
        }
    }
}

pub trait PlatformDetails {
    fn name(&self) -> &'static str;
    fn display_name(&self) -> &'static str;
}

#[derive(Serialize, Debug, Clone)]
pub struct Ports {
    pub vanilla: VanillaPorts,
    pub plugins: HashMap<String, u16>,
    pub mods: HashMap<String, u16>,
}

#[derive(Serialize, Debug, Clone)]
pub struct VanillaPorts {
    pub server: Option<u16>,
    pub query: Option<u16>,
    pub rcon: Option<u16>,
}

#[derive(Clone, Debug)]
pub struct Analyzer {
    pub lines: Vec<String>,
    pub platform: Platform,
}

impl Analyzer {
    pub fn new(lines: &[String]) -> Self {
        let platform = determine_platform(lines);

        Self {
            lines: lines.to_vec(),
            platform,
        }
    }

    fn is_proxy(&self) -> bool {
        matches!(
            self.platform,
            Platform::BungeeCord | Platform::Waterfall | Platform::Velocity
        )
    }

    fn is_modded(&self) -> bool {
        matches!(self.platform, Platform::Forge | Platform::Fabric)
    }

    fn is_bukkit_based(&self) -> bool {
        matches!(
            self.platform,
            Platform::CraftBukkit
                | Platform::Spigot
                | Platform::Paper
                | Platform::Pufferfish
                | Platform::Purpur
        )
    }

    fn plugins(&self, line_limit: usize) -> HashMap<String, String> {
        let mut plugins = HashMap::new();

        if self.is_bukkit_based() {
            for line in self.lines.iter().take(line_limit) {
                let xd = StaticAnalyzer::plugin_bukkit(line);

                if xd.clone().is_some() {
                    plugins.insert(xd.clone().unwrap().name, xd.unwrap().version);
                }
            }
        } else if self.is_proxy() {
            todo!()
        } else if self.is_modded() {
            todo!()
        }

        plugins
    }

    fn version(&self) -> Option<String> {
        if self.is_proxy() {
            match self.platform {
                Platform::BungeeCord => {}
                Platform::Waterfall => {}
                Platform::Velocity => {}
                _ => {}
            }
        } else {
            for line in &self.lines {
                match StaticAnalyzer::noproxy_server_version(line) {
                    None => continue,
                    Some(ver) => {
                        return Some(ver);
                    }
                }
            }
        }
        None
    }

    fn vanilla_ports(&self) -> VanillaPorts {
        const SERVER_PORT_MESSAGE: &str = "Starting Minecraft server on";
        const QUERY_PORT_MESSAGE: &str = "Query running on";
        const RCON_PORT_MESSAGE: &str = "RCON running on";

        let mut vanilla_ports = VanillaPorts {
            server: None,
            query: None,
            rcon: None,
        };

        for line in &self.lines {
            match StaticAnalyzer::vanilla_port(line, SERVER_PORT_MESSAGE) {
                None => {}
                Some(port) => vanilla_ports.server = Some(port),
            }

            match StaticAnalyzer::vanilla_port(line, QUERY_PORT_MESSAGE) {
                None => {}
                Some(port) => vanilla_ports.query = Some(port),
            }

            match StaticAnalyzer::vanilla_port(line, RCON_PORT_MESSAGE) {
                None => {}
                Some(port) => vanilla_ports.rcon = Some(port),
            }
        }

        vanilla_ports
    }

    fn ports(&self, ports: &HashMap<String, Vec<String>>, limit: usize) -> HashMap<String, u16> {
        let mut port_list = HashMap::new();
        for line in self.lines.iter().take(limit) {
            for port in ports {
                for must_contain in port.1 {
                    if let Some(result) = StaticAnalyzer::port(
                        port.0.to_owned(),
                        line.as_str(),
                        must_contain.to_owned(),
                    ) {
                        port_list.insert(result.0, result.1);
                    }
                }
            }
        }

        port_list
    }

    fn plugin_ports(
        &self,
        ports_root: &PortsRoot,
        ports_lines_limit: usize,
    ) -> HashMap<String, u16> {
        if !self.is_bukkit_based() {
            return HashMap::new();
        }

        self.ports(&ports_root.ports.plugins, ports_lines_limit)
    }

    fn mod_ports(&self, ports_root: &PortsRoot, ports_lines_limit: usize) -> HashMap<String, u16> {
        if !self.is_modded() {
            return HashMap::new();
        }

        self.ports(&ports_root.ports.mods, ports_lines_limit)
    }

    pub fn build(self, plugins_limit: usize, ports_limit: usize) -> DynamicAnalyzerDetails {
        let current_directory = std::env::current_dir().unwrap();
        let ports_file_dir = current_directory.join("configuration").join("ports.toml");

        let ports_file = std::fs::read_to_string(ports_file_dir.as_path()).unwrap();
        let ports_root: PortsRoot = toml::from_str(ports_file.as_str()).unwrap();

        DynamicAnalyzerDetails {
            lines: self.lines.clone(),
            plugins: self.plugins(plugins_limit),
            platform: self.platform,
            version: self.version(),
            is_modded: self.is_modded(),
            is_proxy: self.is_proxy(),
            is_bukkit_based: self.is_bukkit_based(),
            ports: Ports {
                vanilla: self.vanilla_ports(),
                plugins: self.plugin_ports(&ports_root, ports_limit),
                mods: self.mod_ports(&ports_root, ports_limit),
            },
        }
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DynamicAnalyzerDetails {
    #[serde(skip_serializing)]
    pub lines: Vec<String>,
    pub plugins: HashMap<String, String>,
    pub platform: Platform,
    pub version: Option<String>,
    pub is_modded: bool,
    pub is_proxy: bool,
    pub is_bukkit_based: bool,
    pub ports: Ports,
}

fn determine_platform(lines: &[String]) -> Platform {
    const CRAFTBUKKIT: &str = "This server is running CraftBukkit version";
    const PAPER: &str = "This server is running Paper version";
    const PUFFERFISH: &str = "This server is running Pufferfish version";
    const PURPUR: &str = "This server is running Purpur version";
    const FABRIC: &str = "with Fabric Loader";
    const FORGE: &str = "Forge mod loading, version";
    const BUNGEECORD: &str = "Enabled BungeeCord version";
    const WATERFALL: &str = "Enabled Waterfall version";
    const VELOCITY: &str = "Booting up Velocity";

    let craftbukkit_option = lines.iter().find(|line| line.contains(CRAFTBUKKIT));

    if lines.iter().any(|line| line.contains(PAPER)) {
        return Platform::Paper;
    }

    match craftbukkit_option {
        Some(line) => {
            if line.contains("-Spigot") {
                return Platform::Spigot;
            } else if line.contains("Paper") {
                return Platform::Paper;
            } else {
                return Platform::CraftBukkit;
            }
        }
        None => {
            if lines.iter().any(|line| line.contains(PAPER)) {
            } else if lines.iter().any(|line| line.contains(PURPUR)) {
                return Platform::Purpur;
            } else if lines.iter().any(|line| line.contains(PUFFERFISH)) {
                return Platform::Pufferfish;
            } else if lines.iter().any(|line| line.contains(BUNGEECORD)) {
                return Platform::BungeeCord;
            } else if lines.iter().any(|line| line.contains(WATERFALL)) {
                return Platform::Waterfall;
            } else if lines.iter().any(|line| line.contains(VELOCITY)) {
                return Platform::Velocity;
            } else if lines.iter().any(|line| line.contains(FORGE)) {
                return Platform::Forge;
            } else if lines.iter().any(|line| line.contains(FABRIC)) {
                return Platform::Fabric;
            }
        }
    }
    Platform::Vanilla
}

#[derive(Deserialize, Debug)]
struct PortsRoot {
    ports: PluginModPorts,
}

#[derive(Deserialize, Debug)]
struct PluginModPorts {
    plugins: HashMap<String, Vec<String>>,
    mods: HashMap<String, Vec<String>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Plugin {
    pub name: String,
    pub version: String,
}
