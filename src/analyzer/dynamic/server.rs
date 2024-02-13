use crate::analyzer::{DynamicAnalyzerDetails, PlatformDetails};

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
        self.dad.platform.name()
    }
}
