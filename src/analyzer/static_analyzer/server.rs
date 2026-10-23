use std::sync::LazyLock;

use regex::Regex;

use super::StaticAnalyzer;

#[derive(Debug)]
pub enum Platform {
    Vanilla,
    Plugin(PluginPlatform),
    Modloader(ModloaderPlatform),
    Proxy(ProxyPlatform),
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Platform::Vanilla => write!(f, "Vanilla"),
            Platform::Plugin(p) => write!(f, "{p}"),
            Platform::Modloader(m) => write!(f, "{m}"),
            Platform::Proxy(p) => write!(f, "{p}"),
        }
    }
}

#[derive(Debug)]
pub enum PluginPlatform {
    CraftBukkit,
    Spigot,
    Paper,
    Purpur,
    Pufferfish,
    Folia,
}

impl std::fmt::Display for PluginPlatform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            PluginPlatform::CraftBukkit => "CraftBukkit",
            PluginPlatform::Spigot => "Spigot",
            PluginPlatform::Paper => "Paper",
            PluginPlatform::Purpur => "Purpur",
            PluginPlatform::Pufferfish => "Pufferfish",
            PluginPlatform::Folia => "Folia",
        };
        f.write_str(name)
    }
}

#[derive(Debug)]
pub enum ModloaderPlatform {
    Forge,
    Fabric,
    NeoForge,
}

impl std::fmt::Display for ModloaderPlatform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            ModloaderPlatform::Forge => "Forge",
            ModloaderPlatform::Fabric => "Fabric",
            ModloaderPlatform::NeoForge => "NeoForge",
        };
        f.write_str(name)
    }
}

#[derive(Debug)]
pub enum ProxyPlatform {
    BungeeCord,
    Waterfall,
    Velocity,
}

impl std::fmt::Display for ProxyPlatform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            ProxyPlatform::BungeeCord => "BungeeCord",
            ProxyPlatform::Waterfall => "Waterfall",
            ProxyPlatform::Velocity => "Velocity",
        };
        f.write_str(name)
    }
}

pub trait StaticServerAnalyzer {
    fn version_platform(&self, line_limit: Option<usize>) -> ServerInfo;
}

static MINECRAFT_VERSION_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Starting minecraft server version (\S+)").unwrap());

static BUKKIT_PLATFORM_VERSION_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"This server is running (\S+) version (\S+)").unwrap());

static NEO_FORGE_PLATFORM_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?:Neo)?Forge mod loading, version ([^\s,]+)").unwrap());

static FORGE_OLD_PLATFORM_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Forge Mod Loader version (\S+)").unwrap());

static FABRIC_PLATFORM_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Loading Minecraft \S+ with Fabric Loader (\S+)").unwrap());

static BUNGEECORD_WATERFALL_PLATFORM_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Enabled \S+ version (\S+)").unwrap());

static VELOCITY_PLATFORM_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Booting up Velocity (\S+)").unwrap());

static FOLIA_PLATFORM_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Loading Folia (\S+)").unwrap());

#[derive(Debug)]
pub struct ServerInfo {
    pub minecraft_version: Option<String>,
    pub platform: Platform,
    pub platform_version: Option<String>,
}

impl StaticServerAnalyzer for StaticAnalyzer {
    fn version_platform(&self, line_limit: Option<usize>) -> ServerInfo {
        let line_limit = line_limit.unwrap_or(usize::MAX);

        let mut minecraft_version = None;
        let mut platform = Platform::Vanilla;
        let mut platform_version = None;

        for line in self.lines.iter().take(line_limit) {
            // Minecraft version
            if let Some(caps) = MINECRAFT_VERSION_RE.captures(line) {
                let version = caps.get(1).map(|p| p.as_str().to_string());

                minecraft_version = version;

                if platform_version.is_some() {
                    break;
                }
            }

            // Plugin platform
            if let Some(caps) = BUKKIT_PLATFORM_VERSION_RE.captures(line) {
                let plat = caps.get(1).map(|p| p.as_str());
                let plat_version = caps.get(2).map(|v| v.as_str());

                if let (Some(plat), Some(plat_version)) = (plat, plat_version) {
                    let plat = if plat.contains("Paper") {
                        Platform::Plugin(PluginPlatform::Paper)
                    } else if plat.contains("Purpur") {
                        Platform::Plugin(PluginPlatform::Purpur)
                    } else if plat.contains("Spigot") {
                        Platform::Plugin(PluginPlatform::Spigot)
                    } else if plat.contains("Pufferfish") {
                        Platform::Plugin(PluginPlatform::Pufferfish)
                    } else if plat.contains("CraftBukkit") {
                        if plat.contains("Spigot") {
                            Platform::Plugin(PluginPlatform::Spigot)
                        } else if plat.contains("Paper") {
                            Platform::Plugin(PluginPlatform::Paper)
                        } else {
                            Platform::Plugin(PluginPlatform::CraftBukkit)
                        }
                    } else {
                        Platform::Plugin(PluginPlatform::CraftBukkit)
                    };

                    platform = plat;
                    platform_version = Some(plat_version.to_string());

                    if minecraft_version.is_some() {
                        break;
                    }
                }
            }

            // Forge
            if let Some(caps) = NEO_FORGE_PLATFORM_RE.captures(line) {
                let version = caps.get(1).map(|v| v.as_str().to_string());

                let plat = if line.contains("NeoForge") {
                    Platform::Modloader(ModloaderPlatform::NeoForge)
                } else {
                    Platform::Modloader(ModloaderPlatform::Forge)
                };

                platform = plat;
                platform_version = version;

                if minecraft_version.is_some() {
                    break;
                }
            }

            // Fabric
            if let Some(caps) = FABRIC_PLATFORM_RE.captures(line) {
                let version = caps.get(1).map(|v| v.as_str().to_string());

                platform = Platform::Modloader(ModloaderPlatform::Fabric);
                platform_version = version;

                if minecraft_version.is_some() {
                    break;
                }
            }

            // Velocity
            if let Some(caps) = VELOCITY_PLATFORM_RE.captures(line) {
                let version = caps.get(1).map(|v| v.as_str().to_string());

                platform = Platform::Proxy(ProxyPlatform::Velocity);
                platform_version = version;

                if minecraft_version.is_some() {
                    break;
                }
            }

            // Forge old
            if let Some(caps) = FORGE_OLD_PLATFORM_RE.captures(line) {
                let version = caps.get(1).map(|v| v.as_str().to_string());

                platform = Platform::Modloader(ModloaderPlatform::Forge);
                platform_version = version;

                if minecraft_version.is_some() {
                    break;
                }
            }

            // BungeeCord
            if let Some(caps) = BUNGEECORD_WATERFALL_PLATFORM_RE.captures(line) {
                let version = caps.get(1).map(|v| v.as_str().to_string());

                let plat = if line.contains("Waterfall") {
                    Platform::Proxy(ProxyPlatform::Waterfall)
                } else {
                    Platform::Proxy(ProxyPlatform::BungeeCord)
                };

                platform = plat;
                platform_version = version;

                if minecraft_version.is_some() {
                    break;
                }
            }

            // Folia
            if let Some(caps) = FOLIA_PLATFORM_RE.captures(line) {
                let version = caps.get(1).map(|v| v.as_str().to_string());

                platform = Platform::Plugin(PluginPlatform::Folia);
                platform_version = version;

                if minecraft_version.is_some() {
                    break;
                }
            }
        }

        ServerInfo {
            minecraft_version,
            platform,
            platform_version,
        }
    }
}
