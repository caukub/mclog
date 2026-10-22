use crate::analyzer::dynamic::DynamicAnalyzer;

pub trait PortAnalyzer {
    fn get() -> Option<u16>;
    fn server() -> Option<u16>;
    fn query() -> Option<u16>;
    fn rcon() -> Option<u16>;
}

impl PortAnalyzer for DynamicAnalyzer {
    fn get() -> Option<u16> {
        todo!()
    }

    fn server() -> Option<u16> {
        todo!()
    }

    fn query() -> Option<u16> {
        todo!()
    }

    fn rcon() -> Option<u16> {
        todo!()
    }
}
