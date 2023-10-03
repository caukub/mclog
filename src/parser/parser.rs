use super::{
    delimiters::{delimiters, detect_delimiter_type, Delimiters},
    log_level::LogLevel,
};

pub struct Parser {
    lines: Vec<String>,
    delimiters: Delimiters,
}

pub struct LogEntry {
    log_level: LogLevel,
    prefix: String,
    message: String,
}

impl Parser {
    pub fn new(lines: Vec<String>, custom_delimiters: Vec<String>) -> Self {
        let delimiter_type = detect_delimiter_type(&lines);
        let delimiters = delimiters(&delimiter_type, custom_delimiters);

        Self { lines, delimiters }
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
                    LogLevel::Info => {
                        let splitted = chunk.split_once(&self.delimiters.info.info).unwrap();

                        prefix = format!("{}{}", splitted.0, self.delimiters.info.info);
                        message = splitted.1.to_string();
                    }
                    LogLevel::Warn => {
                        let splitted: (&str, &str);

                        if chunk.contains(&self.delimiters.warn.warn) {
                            splitted = chunk.split_once(&self.delimiters.warn.warn).unwrap();

                            prefix = format!("{}{}", splitted.0, self.delimiters.warn.warn);
                            message = splitted.1.to_string();
                        } else if chunk.contains(&self.delimiters.warn.warning) {
                            splitted = chunk.split_once(&self.delimiters.warn.warning).unwrap();

                            prefix = format!("{}{}", splitted.0, self.delimiters.warn.warn);
                            message = splitted.1.to_string();
                        }
                    }
                    LogLevel::Error => {
                        let splitted: (&str, &str);

                        if chunk.contains(&self.delimiters.error.error) {
                            splitted = chunk.split_once(&self.delimiters.error.error).unwrap();

                            prefix = format!("{}{}", splitted.0, self.delimiters.error.error);
                            message = splitted.1.to_string();
                        } else if chunk.contains(&self.delimiters.error.severe) {
                            splitted = chunk.split_once(&self.delimiters.error.severe).unwrap();

                            prefix = format!("{}{}", splitted.0, self.delimiters.error.severe);
                            message = splitted.1.to_string();
                        } else if chunk.contains(&self.delimiters.error.fatal) {
                            splitted = chunk.split_once(&self.delimiters.error.fatal).unwrap();

                            prefix = format!("{}{}", splitted.0, self.delimiters.error.fatal);
                            message = splitted.1.to_string();
                        }
                    }
                    LogLevel::Custom => {
                        let delimiter: Vec<&String> = self
                            .delimiters
                            .custom
                            .iter()
                            .filter(|&x| chunk.contains(x))
                            .collect();
                        let delimiter = delimiter[0].as_str();

                        let splitted = chunk.split_once(delimiter).unwrap();

                        prefix = format!("{}{}", splitted.0, delimiter);
                        message = splitted.1.to_string();
                    }
                    LogLevel::Unknown => {
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
                    LogLevel::Info => {
                        let splitted = message.split_once(&self.delimiters.info.info).unwrap();

                        i_prefix = format!("{}{}", splitted.0, self.delimiters.info.info);
                        i_message = splitted.1.to_string();
                    }
                    LogLevel::Warn => {
                        if message.contains(&self.delimiters.warn.warn) {
                            let splitted = message.split_once(&self.delimiters.warn.warn).unwrap();

                            i_prefix = format!("{}{}", splitted.0, self.delimiters.warn.warn);
                            i_message = splitted.1.to_string();
                        } else if message.contains(&self.delimiters.warn.warning) {
                            let splitted =
                                message.split_once(&self.delimiters.warn.warning).unwrap();

                            i_prefix = format!("{}{}", splitted.0, self.delimiters.warn.warning);
                            i_message = splitted.1.to_string();
                        }
                    }
                    LogLevel::Error => {
                        if message.contains(&self.delimiters.error.error) {
                            let splitted =
                                message.split_once(&self.delimiters.error.error).unwrap();

                            i_prefix = format!("{}{}", splitted.0, self.delimiters.error.error);
                            i_message = splitted.1.to_string();
                        } else if message.contains(&self.delimiters.error.severe) {
                            let splitted =
                                message.split_once(&self.delimiters.error.severe).unwrap();

                            i_prefix = format!("{}{}", splitted.0, self.delimiters.error.severe);
                            i_message = splitted.1.to_string();
                        } else if message.contains(&self.delimiters.error.fatal) {
                            let splitted =
                                message.split_once(&self.delimiters.error.fatal).unwrap();

                            i_prefix = format!("{}{}", splitted.0, self.delimiters.error.fatal);
                            i_message = splitted.1.to_string();
                        }
                    }
                    LogLevel::Custom => {
                        let delimiter: Vec<&String> = self
                            .delimiters
                            .custom
                            .iter()
                            .filter(|&x| message.contains(x))
                            .collect();
                        let delimiter = delimiter[0].as_str();

                        let splitted = message.split_once(delimiter).unwrap();

                        i_prefix = format!("{}{}", splitted.0, delimiter);
                        i_message = splitted.1.to_string();
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
                LogLevel::Info => format!(
                    "<span class=\"{}\">{}</span>{}\n",
                    part.log_level,
                    html_escape::encode_text(part.prefix.as_str()),
                    html_escape::encode_text(part.message.as_str())
                ),
                LogLevel::Custom => format!(
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

    fn log_level(&self, line: &str) -> LogLevel {
        let info_delimiters = &self.delimiters.info;
        let warn_delimiters = &self.delimiters.warn;
        let error_delimiters = &self.delimiters.error;

        if line.contains(&info_delimiters.info) {
            return LogLevel::Info;
        } else if line.contains(&warn_delimiters.warn) || line.contains(&warn_delimiters.warning) {
            return LogLevel::Warn;
        } else if line.contains(&error_delimiters.error)
            || line.contains(&error_delimiters.severe)
            || line.contains(&error_delimiters.fatal)
        {
            return LogLevel::Error;
        } else if self.delimiters.custom.iter().any(|dm| line.contains(dm)) {
            return LogLevel::Custom;
        }
        LogLevel::Unknown
    }

    fn contain_log_level(&self, line: &str) -> bool {
        !matches!(self.log_level(line), LogLevel::Unknown)
    }
}
