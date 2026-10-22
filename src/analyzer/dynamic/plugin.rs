use crate::analyzer::dynamic::DynamicAnalyzer;

pub trait PluginAnalyzer {
    fn has(&self, plugin_name: &str) -> bool;
    fn has_nocase(&self, plugin_name: &str) -> bool;
    fn has_loaded(&self, plugin_name: &str) -> bool;
    fn has_loaded_nocase(&self, plugin_name: &str) -> bool;
    fn version(&self, plugin_name: &str) -> Option<String>;
}

impl PluginAnalyzer for DynamicAnalyzer {
    fn has(&self, plugin_name: &str) -> bool {
        todo!()
    }

    fn has_nocase(&self, plugin_name: &str) -> bool {
        todo!()
    }

    fn version(&self, plugin_name: &str) -> Option<String> {
        todo!()
    }

    fn has_loaded(&self, plugin_name: &str) -> bool {
        todo!()
    }

    fn has_loaded_nocase(&self, plugin_name: &str) -> bool {
        todo!()
    }
}
