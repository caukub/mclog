use std::sync::LazyLock;

use regex::Regex;

use super::StaticAnalyzer;

pub enum Platform {
    Vanilla,
    Plugin(PluginPlatform),
    Modloader(ModloaderPlatform),
    Proxy(ProxyPlatform),
}

pub enum PluginPlatform {
    CraftBukkit,
    Spigot,
    Paper,
    Purpur,
    Folia,
}

pub enum ModloaderPlatform {
    Forge,
    Fabric,
    NeoForge,
}

pub enum ProxyPlatform {
    BungeeCord,
    Waterfall,
    Velocity,
}

pub trait StaticServerAnalyzer {
    fn data(&self) -> ServerData;
}

static BUKKIT_BASED_PLATFORM_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"This server is running\s+(.*?)\s+version\s+(\S+)").unwrap());

pub struct ServerData {
    platform: Platform,
    platform_version: Option<String>,
    minecraft_version: Option<String>,
}

impl StaticServerAnalyzer for StaticAnalyzer {
    fn data(&self) -> ServerData {
        for line in self.lines.iter().take(100) {

        }
        ServerData { platform: Platform::Vanilla, platform_version: None, minecraft_version: None }
    }
}
