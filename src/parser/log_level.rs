use std::fmt::Formatter;

#[derive(Clone, Copy)]
pub enum EntryLevel {
    Info,
    Warn,
    Error,
    Custom,
    Unknown,
}

impl std::fmt::Display for EntryLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EntryLevel::Info => write!(f, "i"),
            EntryLevel::Warn => write!(f, "w"),
            EntryLevel::Error => write!(f, "e"),
            EntryLevel::Custom => write!(f, "c"),
            EntryLevel::Unknown => write!(f, "u"),
        }
    }
}
