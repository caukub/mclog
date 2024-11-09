use regex::Regex;
use std::sync::LazyLock;

// TODO - rewrite

// WARN]:
static BRACKET_COLON_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new("((INFO|WARN|WARNING|ERROR|FATAL|SEVERE)]:)").unwrap_or_else(|err| {
        panic!("Failed to create 'BRACKET_COLON_REGEX': {}", err);
    })
});

// WARN]
static BRACKET_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new("((INFO|WARN|WARNING|ERROR|FATAL|SEVERE)])").unwrap_or_else(|err| {
        panic!("Failed to create 'BRACKET_REGEX': {}", err);
    })
});

// WARN:
static COLON_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new("((INFO|WARN|WARNING|ERROR|FATAL|SEVERE):)").unwrap_or_else(|err| {
        panic!("Failed to create 'COLON_REGEX': {}", err);
    })
});

// WARN
static NOCOLON_NOBRACKET_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new("(INFO|WARN|WARNING|ERROR|FATAL|SEVERE)").unwrap_or_else(|err| {
        panic!("Failed to create 'NOCOLON_NOBRACKET_REGEX': {}", err);
    })
});

#[derive(Clone, Copy)]
pub enum DelimiterType {
    BracketColon,
    Bracket,
    Colon,
    NoColonNoBracket,
}

impl std::fmt::Display for DelimiterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DelimiterType::BracketColon => write!(f, "]:"),
            DelimiterType::Bracket => write!(f, "]"),
            DelimiterType::Colon => write!(f, ":"),
            DelimiterType::NoColonNoBracket => write!(f, ""),
        }
    }
}

pub struct Delimiters {
    pub info: Vec<String>,
    pub warn: Vec<String>,
    pub error: Vec<String>,
    pub custom: Vec<String>,
}

impl Delimiters {
    pub fn new(custom_delimiters: Vec<String>, delimiter_type: DelimiterType) -> Self {
        let info = ["INFO"];
        let warn = ["WARN", "WARNING"];
        let error = ["ERROR", "SEVERE", "FATAL"];
        let custom = custom_delimiters;

        let info = info
            .iter()
            .map(|del| format!("{del}{delimiter_type}"))
            .collect();
        let warn = warn
            .iter()
            .map(|del| format!("{del}{delimiter_type}"))
            .collect();
        let error = error
            .iter()
            .map(|del| format!("{del}{delimiter_type}"))
            .collect();
        let custom = custom
            .iter()
            .map(|del| format!("{del}{delimiter_type}"))
            .collect();

        Self {
            info,
            warn,
            error,
            custom,
        }
    }
}

const DELIMITER_MAX_MATCHES: usize = 125;

fn count_matches(lines: &[String], regex: Regex) -> usize {
    lines
        .iter()
        .take(DELIMITER_MAX_MATCHES)
        .filter(|line| regex.is_match(line))
        .count()
}

pub fn detect_delimiter_type(lines: &[String]) -> DelimiterType {
    let delimiter_regexes: Vec<(&'static Regex, DelimiterType)> = vec![
        (&NOCOLON_NOBRACKET_REGEX, DelimiterType::NoColonNoBracket),
        (&COLON_REGEX, DelimiterType::Colon),
        (&BRACKET_REGEX, DelimiterType::Bracket),
        (&BRACKET_COLON_REGEX, DelimiterType::BracketColon),
    ];

    #[allow(clippy::needless_borrowed_reference)]
    let delimiter_type = delimiter_regexes
        .iter()
        .map(|(&ref regex, delimiter_type)| {
            let count = count_matches(lines, regex.clone());
            (count, delimiter_type)
        })
        .max_by_key(|&(count, _)| count)
        .unwrap_or((0, &DelimiterType::NoColonNoBracket))
        .1
        .to_owned();

    delimiter_type
}
