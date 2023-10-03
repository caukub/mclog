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
    pub info: InfoDelimiters,
    pub warn: WarnDelimiters,
    pub error: ErrorDelimiters,
    pub custom: Vec<String>,
}

pub struct InfoDelimiters {
    pub info: String,
}

pub struct WarnDelimiters {
    pub warn: String,
    pub warning: String,
}

pub struct ErrorDelimiters {
    pub error: String,
    pub severe: String,
    pub fatal: String,
}

pub fn delimiters(delimiter_type: &DelimiterType, custom_delimiters: Vec<String>) -> Delimiters {
    let info = InfoDelimiters {
        info: format!("INFO{}", delimiter_type),
    };

    let warn = WarnDelimiters {
        warn: format!("WARN{}", delimiter_type),
        warning: format!("WARNING{}", delimiter_type),
    };

    let error = ErrorDelimiters {
        error: format!("ERROR{}", delimiter_type),
        fatal: format!("FATAL{}", delimiter_type),
        severe: format!("SEVERE{}", delimiter_type),
    };

    Delimiters {
        info,
        warn,
        error,
        custom: custom_delimiters,
    }
}

pub fn detect_delimiter_type(lines: &[String]) -> DelimiterType {
    let mut bracket_colon_matches: (DelimiterType, u16) = (DelimiterType::BracketColon, 0);
    let mut bracket_matches: (DelimiterType, u16) = (DelimiterType::Bracket, 0);
    let mut colon_matches: (DelimiterType, u16) = (DelimiterType::Colon, 0);
    let mut nocolon_nobracket_matches: (DelimiterType, u16) = (DelimiterType::NoColonNoBracket, 0);

    for (index, entry) in lines.iter().enumerate() {
        if index > 300
            || bracket_colon_matches.1 > 15
            || bracket_matches.1 > 15
            || colon_matches.1 > 15
            || nocolon_nobracket_matches.1 > 15
        {
            break;
        }
        let entry = entry.as_str();

        if BRACKET_COLON_REGEX.is_match(entry) {
            bracket_colon_matches.1 += 1;
        } else if BRACKET_REGEX.is_match(entry) {
            bracket_matches.1 += 1;
        } else if COLON_REGEX.is_match(entry) {
            colon_matches.1 += 1;
        } else if NOCOLON_NOBRACKET_REGEX.is_match(entry) {
            nocolon_nobracket_matches.1 += 1;
        }
    }

    let (bracket_colon, bracket, colon, nocolon_nobracket) = (
        bracket_colon_matches.1,
        bracket_matches.1,
        colon_matches.1,
        nocolon_nobracket_matches.1,
    );

    if bracket_colon >= bracket && bracket_colon >= colon && bracket_colon >= nocolon_nobracket {
        bracket_colon_matches.0
    } else if bracket >= colon && bracket >= nocolon_nobracket {
        bracket_matches.0
    } else if colon >= nocolon_nobracket {
        colon_matches.0
    } else {
        nocolon_nobracket_matches.0
    }
}
