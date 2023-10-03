use rhai::ImmutableString;

use crate::analyzer::DynamicAnalyzerDetails;

#[derive(Clone)]
pub struct Plugins {
    dad: DynamicAnalyzerDetails,
}

impl Plugins {
    pub fn new(dad: DynamicAnalyzerDetails) -> Self {
        Self { dad }
    }

    pub fn has(self, name: ImmutableString) -> bool {
        self.dad.plugins.iter().any(|plugin| *plugin.0 == name)
    }

    pub fn has_permissive(self, name: ImmutableString) -> bool {
        self.dad
            .plugins
            .iter()
            .any(|plugin| plugin.0.to_lowercase() == name.to_lowercase())
    }

    pub fn version(self, name: ImmutableString) -> Option<String> {
        let plugin = self
            .dad
            .plugins
            .iter()
            .find(|plugin| plugin.0.to_lowercase() == name.to_lowercase());

        plugin.map(|plugin| plugin.1.to_owned())
    }
}
