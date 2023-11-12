use super::{
    delimiters::{delimiters, detect_delimiter_type, Delimiters},
    log_level::EntryLevel,
};

pub struct Parser {
    lines: Vec<String>,
    delimiters: Delimiters,
}

pub struct LogEntry {
    log_level: EntryLevel,
    prefix: String,
    message: String,
}

impl Parser {
    pub fn new(lines: Vec<String>, custom_delimiters: Vec<String>) -> Self {
        let delimiter_type = detect_delimiter_type(&lines);
        let delimiters = delimiters(&delimiter_type, custom_delimiters);

        Self { lines, delimiters }
    }

    fn split(&self, chunk: String, delimiter: &String) -> (String, String) {
        const EMPTY_TUPLE: (&'static str, &'static str) = ("", "");

        let split = chunk.split_once(delimiter).unwrap_or(EMPTY_TUPLE);

        let prefix = format!("{}{}", split.0, delimiter);
        let message = split.1.to_string();

        (prefix, message)
    }

    fn parse(&self) -> Vec<LogEntry> {
        let mut log_entries = Vec::new();

        let mut last_idx_with_level: usize = 0;
        let mut last_log_level = self.log_level(&self.lines[0]);

        let last_line_idx = self.lines.len() - 1;

        let mut prefix = String::new();
        let mut message = String::new();

        for (index, line) in self.lines.iter().enumerate() {
            let previous_line_idx = match index {
                0 => 0,
                _ => index - 1,
            };

            if self.contain_log_level(line) {
                let chunk = self.lines[last_idx_with_level..=previous_line_idx].join("\n");

                match self.log_level(&chunk) {
                    EntryLevel::Info => {
                        let split = self.split(chunk, &self.delimiters.info.info);

                        prefix = split.0;
                        message = split.1;
                    }
                    EntryLevel::Warn => {
                        if chunk.contains(&self.delimiters.warn.warn) {
                            let split = self.split(chunk, &self.delimiters.warn.warn);

                            prefix = split.0;
                            message = split.1;
                        } else if chunk.contains(&self.delimiters.warn.warning) {
                            let split = self.split(chunk, &self.delimiters.warn.warning);

                            prefix = split.0;
                            message = split.1;
                        }
                    }
                    EntryLevel::Error => {
                        if chunk.contains(&self.delimiters.error.error) {
                            let split = self.split(chunk, &self.delimiters.error.error);

                            prefix = split.0;
                            message = split.1;
                        } else if chunk.contains(&self.delimiters.error.severe) {
                            let split = self.split(chunk, &self.delimiters.error.severe);

                            prefix = split.0;
                            message = split.1;
                        } else if chunk.contains(&self.delimiters.error.fatal) {
                            let split = self.split(chunk, &self.delimiters.error.fatal);

                            prefix = split.0;
                            message = split.1;
                        }
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

                        prefix = format!("{}{}", split.0, delimiter);
                        message = split.1.to_string();
                    }
                    EntryLevel::Unknown => {
                        prefix = String::new();
                        message = chunk;
                    }
                }

                log_entries.push(LogEntry {
                    log_level: last_log_level,
                    prefix: prefix.clone(),
                    message: message.clone(),
                });

                last_idx_with_level = index;
                last_log_level = self.log_level(line);
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
                let level = self.log_level(&self.lines[last_line_idx]);
                let message = &self.lines[last_line_idx];

                match level {
                    EntryLevel::Info => {
                        let split = self.split(message.to_owned(),&self.delimiters.info.info);

                        i_prefix = split.0;
                        i_message = split.1;
                    }
                    EntryLevel::Warn => {
                        if message.contains(&self.delimiters.warn.warn) {
                            let split = self.split(message.to_owned(),&self.delimiters.warn.warn);

                            i_prefix = split.0;
                            i_message = split.1;
                        } else if message.contains(&self.delimiters.warn.warning) {
                            let split = self.split(message.to_owned(), &self.delimiters.warn.warning);

                            i_prefix = split.0;
                            i_message = split.1;
                        }
                    }
                    EntryLevel::Error => {
                        if message.contains(&self.delimiters.error.error) {
                            let split = self.split(message.to_owned(), &self.delimiters.error.error);

                            i_prefix = split.0;
                            i_message = split.1;
                        } else if message.contains(&self.delimiters.error.severe) {
                            let split = self.split(message.to_owned(), &self.delimiters.error.severe);

                            i_prefix = split.0;
                            i_message = split.1;
                        } else if message.contains(&self.delimiters.error.fatal) {
                            let split = self.split(message.to_owned(), &self.delimiters.error.fatal);

                            i_prefix = split.0;
                            i_message = split.1;
                        }
                    }
                    EntryLevel::Custom => {
                        let delimiter: Vec<&String> = self
                            .delimiters
                            .custom
                            .iter()
                            .filter(|&x| message.contains(x))
                            .collect();
                        let delimiter = delimiter[0].as_str();

                        let split = self.split(message.to_owned(), &delimiter.to_string());

                        i_prefix = split.0;
                        i_message = split.1;
                    }
                    _ => {}
                }

                log_entries.push(LogEntry {
                    log_level: level,
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

    pub fn html(&self) -> Vec<String> {
        let parts = self.parse();

        let mut html_parts = Vec::new();

        for part in parts {
            let html_part = match part.log_level {
                EntryLevel::Info => format!(
                    "<span class=\"{}\">{}</span>{}\n",
                    part.log_level,
                    html_escape::encode_text(part.prefix.as_str()),
                    html_escape::encode_text(part.message.as_str())
                ),
                EntryLevel::Custom => format!(
                    "<span class=\"{}\">{}</span>{}\n",
                    part.log_level,
                    html_escape::encode_text(part.prefix.as_str()),
                    html_escape::encode_text(part.message.as_str())
                ),
                _ => format!(
                    "<span class=\"{}\">{}{}</span>\n",
                    part.log_level,
                    html_escape::encode_text(part.prefix.as_str()),
                    html_escape::encode_text(part.message.as_str()),
                ),
            };
            html_parts.push(html_part);
        }

        html_parts
    }

    fn log_level(&self, line: &str) -> EntryLevel {
        let info_delimiters = &self.delimiters.info;
        let warn_delimiters = &self.delimiters.warn;
        let error_delimiters = &self.delimiters.error;

        if line.contains(&info_delimiters.info) {
            return EntryLevel::Info;
        } else if line.contains(&warn_delimiters.warn) || line.contains(&warn_delimiters.warning) {
            return EntryLevel::Warn;
        } else if line.contains(&error_delimiters.error)
            || line.contains(&error_delimiters.severe)
            || line.contains(&error_delimiters.fatal)
        {
            return EntryLevel::Error;
        } else if self.delimiters.custom.iter().any(|dm| line.contains(dm)) {
            return EntryLevel::Custom;
        }
        EntryLevel::Unknown
    }

    fn contain_log_level(&self, line: &str) -> bool {
        !matches!(self.log_level(line), EntryLevel::Unknown)
    }
}