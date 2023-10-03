use rhai::ImmutableString;

use crate::analyzer::DynamicAnalyzerDetails;

#[derive(Clone)]
pub struct Ports {
    dad: DynamicAnalyzerDetails,
}

impl Ports {
    pub fn new(dad: DynamicAnalyzerDetails) -> Self {
        Self { dad }
    }

    pub fn get(self, name: ImmutableString) -> i32 {
        let port = if self.dad.is_modded {
            self.dad.ports.mods.get(&name.to_string())
        } else {
            self.dad.ports.plugins.get(&name.to_string())
        };

        match port {
            None => 0,
            Some(port) => *port as i32,
        }
    }

    pub fn server(&mut self) -> i32 {
        self.dad.ports.vanilla.server.unwrap_or(0) as i32
    }

    pub fn query(&mut self) -> i32 {
        self.dad.ports.vanilla.query.unwrap_or(0) as i32
    }

    pub fn rcon(&mut self) -> i32 {
        self.dad.ports.vanilla.rcon.unwrap_or(0) as i32
    }
}
