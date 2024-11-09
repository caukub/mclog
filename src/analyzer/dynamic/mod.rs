use lazy_static::lazy_static;
use regex::Regex;
use rhai::{ImmutableString, AST};
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use {log::Log, plugins::Plugins, ports::Ports, server::Server};
use crate::analyzer::dynamic::chunks::Chunks;

pub mod plugins;
pub mod ports;
pub mod server;
pub mod chunks;

pub struct ScriptPlatform<P> {
    pub global: P,
    pub noproxy: P,
    pub bukkit: P,
    pub forge: P,
    pub fabric: P,
    pub bungeecord: P,
    pub velocity: P,
}

impl<P: AsRef<Path>> AsRef<Path> for ScriptPlatform<P> {
    fn as_ref(&self) -> &Path {
        self.global.as_ref()
    }
}

pub struct Script {
    pub file: String,
    pub script: String,
    pub detection: HashMap<String, Detection>,
    pub path: PathBuf,
    pub ast: AST,
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
    pub script_directories: ScriptPlatform<PathBuf>,
}

impl DynamicAnalyzer {
    fn script_paths(&self, directory: impl AsRef<Path>) -> Vec<PathBuf> {
        let mut files = vec![];

        for file in std::fs::read_dir(directory).unwrap().flatten() {
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

    fn scripts(&self, paths: Vec<PathBuf>) -> Vec<Script> {
        let mut scripts = Vec::new();

        for file in paths {
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

    fn global_paths(&self) -> Vec<PathBuf> {
        self.script_paths(&self.script_directories.global)
    }

    fn noproxy_paths(&self) -> Vec<PathBuf> {
        self.script_paths(&self.script_directories.noproxy)
    }

    fn bukkit_paths(&self) -> Vec<PathBuf> {
        self.script_paths(&self.script_directories.bukkit)
    }

    fn forge_paths(&self) -> Vec<PathBuf> {
        self.script_paths(&self.script_directories.forge)
    }

    fn fabric_paths(&self) -> Vec<PathBuf> {
        self.script_paths(&self.script_directories.fabric)
    }

    fn bungeecord_paths(&self) -> Vec<PathBuf> {
        self.script_paths(&self.script_directories.bungeecord)
    }

    fn velocity_paths(&self) -> Vec<PathBuf> {
        self.script_paths(&self.script_directories.velocity)
    }

    pub fn global_scripts(&self) -> Vec<Script> {
        self.scripts(self.global_paths())
    }

    pub fn noproxy_scripts(&self) -> Vec<Script> {
        self.scripts(self.noproxy_paths())
    }

    pub fn bukkit_scripts(&self) -> Vec<Script> {
        self.scripts(self.bukkit_paths())
    }

    pub fn forge_scripts(&self) -> Vec<Script> {
        self.scripts(self.forge_paths())
    }

    pub fn fabric_scripts(&self) -> Vec<Script> {
        self.scripts(self.fabric_paths())
    }

    pub fn bungeecord_scripts(&self) -> Vec<Script> {
        self.scripts(self.bungeecord_paths())
    }

    pub fn velocity_scripts(&self) -> Vec<Script> {
        self.scripts(self.velocity_paths())
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
            .register_get("rcon", Ports::rcon);;

        let current_directory = std::env::current_dir().expect("Coudln't get current directory");
        let scripts_directory = current_directory.join("scripts");

        let script_directories = ScriptPlatform {
            global: scripts_directory.join("global"),
            noproxy: scripts_directory.join("noproxy"),
            bukkit: scripts_directory.join("bukkit"),
            forge: scripts_directory.join("forge"),
            fabric: scripts_directory.join("fabric"),
            bungeecord: scripts_directory.join("bungeecord"),
            velocity: scripts_directory.join("velocity"),
        };

        Self {
            engine,
            script_directories,
        }
    }
}

lazy_static! {
    pub static ref SEMVER_REGEX: Regex = Regex::new(r"\d+(\.\d+){0,2}")
        .unwrap_or_else(|e| { panic!("Failed to create 'SEMVER_REGEX' regex: {}", e) });
}

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
