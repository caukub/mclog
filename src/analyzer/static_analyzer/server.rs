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
    fn platform() -> Option<Platform>;
    fn version() -> Option<String>;
}

impl StaticServerAnalyzer for StaticAnalyzer {
    fn platform() -> Option<Platform> {
        todo!()
    }

    fn version() -> Option<String> {
        todo!()
    }
}