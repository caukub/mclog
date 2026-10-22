use crate::analyzer::dynamic::DynamicAnalyzer;

pub trait ServerAnalyzer {
    fn is_proxy() -> bool;
    fn is_modded() -> bool;
    fn is_bukkit_based() -> bool;
    fn has_known_version() -> bool;
    fn version() -> String;
    fn platform() -> String;
    fn java_version() -> String;
}

impl ServerAnalyzer for DynamicAnalyzer {
    fn is_proxy() -> bool {
        todo!()
    }

    fn is_modded() -> bool {
        todo!()
    }

    fn is_bukkit_based() -> bool {
        todo!()
    }

    fn has_known_version() -> bool {
        todo!()
    }

    fn version() -> String {
        todo!()
    }

    fn platform() -> String {
        todo!()
    }

    fn java_version() -> String {
        todo!()
    }
}
