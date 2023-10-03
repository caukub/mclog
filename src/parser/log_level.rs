#[derive(Clone, Copy)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Custom,
    Unknown,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Info => write!(f, "i"),
            LogLevel::Warn => write!(f, "w"),
            LogLevel::Error => write!(f, "e"),
            LogLevel::Custom => write!(f, "p"),
            LogLevel::Unknown => write!(f, "u"),
        }
    }
}
