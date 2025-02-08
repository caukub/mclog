use super::{delimiters::detect_delimiter_type, log_level::EntryLevel};
use crate::parser::delimiters::Delimiters;

pub struct Parser {
    lines: Vec<String>,
    delimiters: Delimiters,
}

#[derive(Debug)]
pub struct LogEntry {
    log_level: EntryLevel,
    prefix: String,
    message: String,
}

impl Parser {
    pub fn new(lines: Vec<String>, custom_delimiters: Vec<String>) -> Self {
        let delimiter_type = detect_delimiter_type(&lines);
        let delimiters = Delimiters::new(custom_delimiters, delimiter_type);
        Self { lines, delimiters }
    }

    fn split(&self, chunk: String, delimiter: &str) -> (String, String) {
        const EMPTY_TUPLE: (&str, &str) = ("", "");

        let split = chunk.split_once(delimiter).unwrap_or(EMPTY_TUPLE);

        let prefix = format!("{}{}", split.0, delimiter);
        let message = split.1.to_string();

        (prefix, message)
    }

    fn parse(&self) -> Vec<LogEntry> {
        let mut log_entries = Vec::new();

        let mut last_idx_with_level: usize = 0;
        let mut last_log_level = self.log_level(&self.lines[0]).0;

        let last_line_idx = self.lines.len() - 1;

        #[allow(unused_assignments)]
        let mut prefix = String::new();
        #[allow(unused_assignments)]
        let mut message = String::new();

        for (index, line) in self.lines.iter().enumerate() {
            let previous_line_idx = match index {
                0 => 0,
                _ => index - 1,
            };

            if self.contain_log_level(line) {
                let chunk = self.lines[last_idx_with_level..=previous_line_idx].join("\n");

                let log_level = self.log_level(&chunk);

                let entry_level = log_level.0;
                let delimiter = log_level.1;

                let split = match entry_level {
                    EntryLevel::Info | EntryLevel::Warn | EntryLevel::Error => {
                        self.split(chunk, &delimiter.unwrap_or_default())
                    }
                    EntryLevel::Custom => {
                        let delimiter: Vec<&String> = self
                            .delimiters
                            .custom
                            .iter()
                            .filter(|&x| chunk.contains(x))
                            .collect();

                        let delimiter = delimiter[0].as_str();

                        let split = chunk.split_once(delimiter).unwrap();

                        (format!("{}{}", split.0, delimiter), split.1.to_string())
                    }
                    EntryLevel::Unknown => (String::new(), chunk),
                };

                prefix = split.0;
                message = split.1;

                log_entries.push(LogEntry {
                    log_level: last_log_level,
                    prefix,
                    message,
                });

                last_idx_with_level = index;
                last_log_level = self.log_level(line).0;
            }
        }

        match self.contain_log_level(&self.lines[last_line_idx]) {
            false => {
                let chunk = self.lines[last_idx_with_level..=last_line_idx].join("\n");
                log_entries.push(LogEntry {
                    log_level: last_log_level,
                    message: chunk,
                    prefix: String::new(),
                })
            }
            true => {
                let mut i_prefix = String::new();
                let mut i_message = String::new();

                let message = &self.lines[last_line_idx];

                let log_level = self.log_level(message);

                let entry_level = log_level.0;
                let delimiter = log_level.1.unwrap_or_default();

                match entry_level {
                    EntryLevel::Info | EntryLevel::Warn | EntryLevel::Error => {
                        let split = self.split(message.to_owned(), &delimiter);

                        i_prefix = split.0;
                        i_message = split.1;
                    }
                    EntryLevel::Custom => {
                        let delimiter: Vec<&String> = self
                            .delimiters
                            .custom
                            .iter()
                            .filter(|&x| message.contains(x))
                            .collect();
                        let delimiter = delimiter[0].as_str();

                        let split = self.split(message.to_owned(), delimiter);

                        i_prefix = split.0;
                        i_message = split.1;
                    }
                    EntryLevel::Unknown => {}
                }

                log_entries.push(LogEntry {
                    log_level: entry_level,
                    prefix: i_prefix,
                    message: i_message,
                });
            }
        }

        if self.contain_log_level(&self.lines[0]) {
            log_entries.remove(0);
        }

        log_entries
    }

    pub fn get_chunks(&self) -> Vec<String> {
        let chunks = self.parse();

        let mut output = Vec::new();

        for chunk in chunks {
            output.push(format!("{}{}", chunk.prefix, chunk.message))
        }

        output
    }

    pub fn html(&self) -> Vec<String> {
        let parts = self.parse();

        let mut html_parts = Vec::new();

        let mut id: usize = 1;

        for part in parts {
            let html_part = match part.log_level {
                EntryLevel::Info => format!(
                    r#"<span class="p" id="L{id}"><span class={}>{}</span>{}</span>"#,
                    part.log_level,
                    html_escape::encode_text(part.prefix.as_str()),
                    html_escape::encode_text(part.message.as_str())
                ),
                EntryLevel::Warn | EntryLevel::Error | EntryLevel::Unknown => format!(
                    r#"<span class="p" id="L{id}"><span class="{}">{}{}</span></span>"#,
                    part.log_level,
                    html_escape::encode_text(part.prefix.as_str()),
                    html_escape::encode_text(part.message.as_str()),
                ),
                EntryLevel::Custom => format!(
                    r#"<span class="p" id="L{id}"><span class="{}">{}</span>{}</span>"#,
                    part.log_level,
                    html_escape::encode_text(part.prefix.as_str()),
                    html_escape::encode_text(part.message.as_str())
                ),
            };
            html_parts.push(html_part);
            id += 1;
        }

        html_parts
    }

    fn log_level(&self, line: &str) -> (EntryLevel, Option<String>) {
        if let Some(del) = self.delimiters.info.iter().find(|&del| line.contains(del)) {
            return (EntryLevel::Info, Some(del.to_owned()));
        }

        let warn_delimiter = self.delimiters.warn.iter().find(|&del| line.contains(del));

        if let Some(del) = warn_delimiter {
            return (EntryLevel::Warn, Some(del.to_owned()));
        }

        let error_delimiter = self.delimiters.error.iter().find(|&del| line.contains(del));

        if let Some(del) = error_delimiter {
            return (EntryLevel::Error, Some(del.to_owned()));
        }

        let custom_delimiter = self
            .delimiters
            .custom
            .iter()
            .find(|&del| line.contains(del));
        if let Some(del) = custom_delimiter {
            return (EntryLevel::Custom, Some(del.to_owned()));
        }

        (EntryLevel::Unknown, None)
    }

    fn contain_log_level(&self, line: &str) -> bool {
        !matches!(self.log_level(line).0, EntryLevel::Unknown)
    }
}
