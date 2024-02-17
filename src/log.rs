use lazy_static::lazy_static;
use regex::Regex;
use tokio::{
    fs::File,
    io::{BufReader, Lines},
};
use tokio_stream::{wrappers::LinesStream, StreamExt};

use crate::analyzer::static_analyzer::StaticAnalyzer;

pub struct Log {
    lines: Lines<BufReader<File>>,
}

lazy_static! {
    static ref IPV4_REGEX: Regex = Regex::new(r"(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})")
        .unwrap_or_else(|e| { panic!("Failed to create 'IPV4_REGEX': {}", e) });
}

impl Log {
    pub fn new(lines: Lines<BufReader<File>>) -> Self {
        Self { lines }
    }

    pub async fn lines(self) -> Vec<String> {
        let mut lines = Vec::new();

        let mut lines_stream = LinesStream::new(self.lines);

        while let Some(Ok(line)) = lines_stream.next().await {
            lines.push(line);
        }

        lines
    }

    pub async fn first_n_lines(self, limit: usize) -> Vec<String> {
        let mut lines = Vec::new();

        let mut lines_stream = LinesStream::new(self.lines).take(limit);

        while let Some(Ok(line)) = lines_stream.next().await {
            lines.push(line)
        }

        lines
    }

    pub async fn _lines_hideips(self) -> Vec<String> {
        let mut _lines_stream = LinesStream::new(self.lines);
        unimplemented!()
    }

    pub async fn first_n_lines_hideips(self, limit: usize) -> Vec<String> {
        let mut lines = Vec::new();

        let mut lines_stream = LinesStream::new(self.lines).take(limit);

        let mut matched_plugin_versions = Vec::new();

        let lines_to_ignore = [
            "plugins/",
            "Forge Mod Loader version",
            "MinecraftForge v",
            "127.0.0.1",
            "0.0.0.0",
            "openjdk",
            "OpenJDK",
        ];

        //
        let lines_to_replace = [
            "at (", // Shotyr[/{ipv4}:58381] logged in with entity id 675
                   // at ([world]-102.50147912322777, 94.88908505183846, -117.07016565695118)
                   // SetSpawn v4.8
            "logged in with entity id",
        ];

        while let Some(Ok(line)) = lines_stream.next().await {
            if let Some(plugin) = StaticAnalyzer::plugin_bukkit(&line) {
                matched_plugin_versions.push(plugin.version);
                lines.push(line);
            } else if matched_plugin_versions.iter().any(|ver| line.contains(ver))
                || lines_to_ignore.iter().any(|i| line.contains(i))
            {
                if lines_to_replace.iter().any(|&ss| line.contains(ss)) {
                    let cleared = IPV4_REGEX.replace_all(&line, "{ipv4}").to_string();
                    lines.push(cleared);
                } else {
                    lines.push(line);
                }
            } else {
                let cleared_line = IPV4_REGEX.replace_all(&line, "{ipv4}").to_string();
                lines.push(cleared_line);
            }
        }

        lines
    }
}
