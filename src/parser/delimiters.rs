use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    // WARN]:
    static ref BRACKET_COLON_REGEX: Regex = Regex::new("((INFO|WARN|WARNING|ERROR|FATAL|SEVERE)]:)").unwrap_or_else(|e| {
        panic!("Failed to create 'BRACKET_COLON_REGEX': {}", e);
    });

    // WARN]
    static ref BRACKET_REGEX: Regex = Regex::new("((INFO|WARN|WARNING|ERROR|FATAL|SEVERE)])").unwrap_or_else(|e| {
        panic!("Failed to create 'BRACKET_REGEX': {}", e);
    });


    // WARN:
    static ref COLON_REGEX: Regex = Regex::new("((INFO|WARN|WARNING|ERROR|FATAL|SEVERE):)").unwrap_or_else(|e| {
        panic!("Failed to create 'COLON_REGEX': {}", e);
    });

    // WARN
    static ref NOCOLON_NOBRACKET_REGEX: Regex = Regex::new("(INFO|WARN|WARNING|ERROR|FATAL|SEVERE)").unwrap_or_else(|e| {
        panic!("Failed to create 'NOCOLON_NOBRACKET_REGEX': {}", e);
    });
}

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
        let info = vec!["INFO"];
        let warn = vec!["WARN", "WARNING"];
        let error = vec!["ERROR", "SEVERE", "FATAL"];
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
