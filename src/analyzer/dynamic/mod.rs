use crate::analyzer::dynamic::chunks::Chunks;
use regex::Regex;
use rhai::{ImmutableString, AST};
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use {log::Log, plugins::Plugins, ports::Ports, server::Server};

pub mod chunks;
pub mod plugins;
pub mod ports;
pub mod server;

pub static SCRIPTS_DIRECTORY: LazyLock<PathBuf> = LazyLock::new(|| {
    let current_directory = std::env::current_dir().unwrap();
    current_directory.join("scripts")
});

pub struct Script {
    pub file: String,
    pub script: String,
    pub detection: HashMap<String, Detection>,
    pub path: PathBuf,
    pub ast: AST,
}

pub enum ScriptPlatform {
    Global,
    NoProxy,
    Bukkit,
    Forge,
    Fabric,
    BungeeCord,
    Velocity,
    Folia,
}

impl ScriptPlatform {
    fn directory(&self) -> impl AsRef<Path> {
        let dir = match self {
            ScriptPlatform::Global => "global",
            ScriptPlatform::NoProxy => "noproxy",
            ScriptPlatform::Bukkit => "bukkit",
            ScriptPlatform::Forge => "forge",
            ScriptPlatform::Fabric => "fabric",
            ScriptPlatform::BungeeCord => "bungeecord",
            ScriptPlatform::Velocity => "velocity",
            ScriptPlatform::Folia => "folia",
        };

        SCRIPTS_DIRECTORY.join(dir)
    }

    pub fn script_paths(&self) -> Vec<PathBuf> {
        let mut files = vec![];

        for file in std::fs::read_dir(self.directory()).unwrap().flatten() {
            if file
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with('_')
            {
                continue;
            }
            files.push(file.path());
        }
        files
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Detection {
    pub header: String,
    pub solutions: Vec<String>,
    pub private: Option<bool>,
    pub detail: Option<String>,
    pub level: Option<DetectionLevel>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Ord, Eq, PartialEq, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum DetectionLevel {
    Critical,
    Error,
    Warn,
    Info,
}

pub struct DynamicAnalyzer {
    pub engine: rhai::Engine,
}

impl DynamicAnalyzer {
    pub fn scripts(&self, script_platform: ScriptPlatform) -> Vec<Script> {
        let mut scripts = Vec::new();

        for file in script_platform.script_paths() {
            let file_name = file.file_name().unwrap().to_str().unwrap().to_string();
            let script_file_content = std::fs::read_to_string(&file).unwrap();
            let script_split = script_file_content.split_once("///").unwrap();

            let script_content = script_split.0.to_string();
            let toml = script_split.1.to_string();

            let detection: HashMap<String, Detection> = match toml::from_str(&toml) {
                Ok(hm) => hm,
                Err(_) => continue,
            };

            let ast = self.ast(&script_content);

            scripts.push(Script {
                file: file_name,
                script: script_content,
                detection,
                path: file,
                ast,
            })
        }

        scripts
    }

    fn ast(&self, content: &String) -> AST {
        let content = format!(
            "
            let ports = new_ports(dad);
            let plugins = new_plugins(dad);
            let server = new_server(dad);
            let chunks = new_chunks(dad);
            {}
            return ();
            ",
            content
        );

        self.engine.compile(content).unwrap()
    }
}

impl Default for DynamicAnalyzer {
    fn default() -> Self {
        let mut engine = rhai::Engine::new();

        engine
            .register_custom_operator("matchver", 160)
            .unwrap()
            .register_fn("matchver", matches_version);

        engine
            .register_custom_operator("matchserver", 160)
            .unwrap()
            .register_fn("matchserver", matches_server_version);

        engine
            .register_type::<i32>()
            .register_fn("to_string", i32::to_string);

        engine
            .register_type::<Plugins>()
            .register_fn("new_plugins", Plugins::new)
            .register_fn("version", Plugins::version)
            .register_fn("has", Plugins::has)
            .register_fn("has_permissive", Plugins::has_permissive);

        engine
            .register_type::<Server>()
            .register_fn("new_server", Server::new)
            .register_fn("is_proxy", Server::is_proxy)
            .register_fn("is_modded", Server::is_modded)
            .register_fn("is_known_version", Server::is_known_version)
            .register_fn("is_bukkit_based", Server::is_bukkit_based)
            .register_get("version", Server::version)
            .register_get("platform", Server::platform);

        engine
            .register_type::<Chunks>()
            .register_fn("new_chunks", Chunks::new)
            .register_fn("has_line", Chunks::has_line)
            .register_fn("has_line", Chunks::has_line2)
            .register_fn("has_line_permissive", Chunks::has_line_permissive);

        engine
            .register_type::<Ports>()
            .register_fn("new_ports", Ports::new)
            .register_fn("get", Ports::get)
            .register_get("server", Ports::server)
            .register_get("query", Ports::query)
            .register_get("rcon", Ports::rcon);

        Self { engine }
    }
}

pub static SEMVER_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\d+(\.\d+){0,2}")
        .unwrap_or_else(|e| panic!("Failed to create 'SEMVER_REGEX' regex: {}", e))
});

fn matches_version(version: Option<String>, version_requirements: ImmutableString) -> bool {
    let version = match version {
        Some(ver) => ver,
        None => return false,
    };

    let version = match SEMVER_REGEX.is_match(&version) {
        true => SEMVER_REGEX
            .captures(&version)
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .to_string(),
        false => return false,
    };

    if !SEMVER_REGEX.is_match(&version) {
        return false;
    }

    if !SEMVER_REGEX.is_match(&version_requirements) {
        return false;
    }

    let version = match Version::parse(&version) {
        Ok(version) => version,
        Err(_) => {
            let dot_count = version.chars().filter(|&c| c == '.').count();

            let semver_fixer = match dot_count {
                0 => ".0.0",
                1 => ".0",
                _ => return false,
            };

            let fixed_version = format!("{}{}", version, semver_fixer);
            match Version::parse(&fixed_version) {
                Ok(version) => version,
                Err(_) => return false,
            }
        }
    };
    let version_requirements = match VersionReq::parse(&version_requirements) {
        Ok(ver) => ver,
        Err(_) => return false,
    };

    version_requirements.matches(&version)
}

fn matches_server_version(server_version: String, version_requirements: String) -> bool {
    if !SEMVER_REGEX.is_match(&server_version) {
        return false;
    }

    if !SEMVER_REGEX.is_match(&version_requirements) {
        return false;
    }

    let server_version = Version::parse(&server_version).unwrap();

    let version_requirements = match VersionReq::parse(&version_requirements) {
        Ok(vr) => vr,
        Err(_) => return false,
    };

    version_requirements.matches(&server_version)
}
