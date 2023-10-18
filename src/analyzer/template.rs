use serde::Serialize;
use std::collections::HashMap;

use super::{Platform, Plugin, VanillaPorts};

pub struct TemplateInfo {
    pub platform: Platform,
    pub version: Option<String>,
    pub is_proxy: bool,
    pub is_modded: bool,
    pub ports: Ports,
    pub plugins: Vec<Plugin>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Ports {
    pub vanilla: VanillaPorts,
    pub plugins: HashMap<String, u16>,
    pub mods: HashMap<String, u16>,
}
