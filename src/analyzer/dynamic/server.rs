use crate::analyzer::{DynamicAnalyzerDetails, Platform};

#[derive(Clone)]
pub struct Server {
    dad: DynamicAnalyzerDetails,
}

impl Server {
    pub fn new(dad: DynamicAnalyzerDetails) -> Self {
        Self { dad }
    }

    pub fn is_proxy(self) -> bool {
        self.dad.is_proxy
    }

    pub fn is_modded(self) -> bool {
        self.dad.is_modded
    }

    pub fn is_bukkit_based(self) -> bool {
        self.dad.is_bukkit_based
    }

    pub fn is_known_version(&mut self) -> bool {
        self.dad.version.is_some()
    }

    pub fn version(&mut self) -> String {
        match &self.dad.version {
            None => "Unknown".to_string(),
            Some(ver) => ver.to_owned(),
        }
    }

    pub fn platform(&mut self) -> &'static str {
        match self.dad.platform {
            Platform::Vanilla => "vanilla",
            Platform::CraftBukkit => "bukkit",
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
}
