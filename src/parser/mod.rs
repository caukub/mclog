use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Fatal,
    Severe,
    Debug,
    Trace,
    Warning,
    Custom,
    Unknown,
}

impl LogLevel {
    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error | Self::Fatal | Self::Severe)
    }

    pub fn is_warning(&self) -> bool {
        matches!(self, Self::Warn | Self::Warning)
    }
}

#[derive(Debug)]
pub struct LogEntry {
    level: LogLevel,
    content: String,
}

#[derive(Debug)]
pub struct StructuredLogEntry {
    level: LogLevel,
    prefix: Option<String>,
    message: String,
}

#[derive(Debug)]
pub struct Delimiters {
    info: String,
    warn: String,
    error: String,
    fatal: String,
    severe: String,
    warning: String,
    debug: String,
    trace: String,
}

pub struct Parser<'a, S: AsRef<str>> {
    lines: &'a [S],
    line_limit: Option<usize>,
    analyzer_line_limit: Option<usize>,
    delimiter_match_limit: Option<usize>,
    delimiters: Delimiters,
}

impl<'a, S: AsRef<str>> Parser<'a, S> {
    pub fn new(
        lines: &'a [S],
        line_limit: Option<usize>,
        analyzer_line_limit: Option<usize>,
        delimiter_match_limit: Option<usize>,
    ) -> Self {
        let log_level_suffix =
            find_log_level_suffix(lines, analyzer_line_limit, delimiter_match_limit);
        let delimiters = get_delimiters(log_level_suffix);

        Self {
            lines,
            line_limit,
            analyzer_line_limit,
            delimiter_match_limit,
            delimiters,
        }
    }

    pub fn structure_lines(&self) -> Vec<StructuredLogEntry> {
        let mut structured_lines = Vec::new();

        let mut last_log_level = LogLevel::Unknown;

        for line in self.lines {
            let line = line.as_ref();
            let mut contains_delimiter = false;

            if line.contains(&self.delimiters.info) {
                last_log_level = LogLevel::Info;
                contains_delimiter = true;
            } else if line.contains(&self.delimiters.warn) {
                last_log_level = LogLevel::Warn;
                contains_delimiter = true;
            } else if line.contains(&self.delimiters.error) {
                last_log_level = LogLevel::Error;
                contains_delimiter = true;
            } else if line.contains(&self.delimiters.fatal) {
                last_log_level = LogLevel::Fatal;
                contains_delimiter = true;
            } else if line.contains(&self.delimiters.severe) {
                last_log_level = LogLevel::Severe;
                contains_delimiter = true;
            } else if line.contains(&self.delimiters.debug) {
                last_log_level = LogLevel::Debug;
                contains_delimiter = true;
            } else if line.contains(&self.delimiters.trace) {
                last_log_level = LogLevel::Trace;
                contains_delimiter = true;
            } else if line.contains(&self.delimiters.warning) {
                last_log_level = LogLevel::Warning;
                contains_delimiter = true;
            }

            let delimiter: Option<&str> = match last_log_level {
                LogLevel::Info => Some(&self.delimiters.info),
                LogLevel::Warn => Some(&self.delimiters.warn),
                LogLevel::Error => Some(&self.delimiters.error),
                LogLevel::Fatal => Some(&self.delimiters.fatal),
                LogLevel::Severe => Some(&self.delimiters.severe),
                LogLevel::Debug => Some(&self.delimiters.debug),
                LogLevel::Trace => Some(&self.delimiters.trace),
                LogLevel::Warning => Some(&self.delimiters.warning),
                LogLevel::Custom | LogLevel::Unknown => None,
            };

            let entry = if contains_delimiter {
                match delimiter {
                    Some(deli) => {
                        let splitted = line.split_once(deli).unwrap();
                        let (prefix, message) = splitted;
                        let prefix = format!("{}{}", prefix, deli);

                        let entry = StructuredLogEntry {
                            level: last_log_level,
                            prefix: Some(prefix),
                            message: message.to_string(),
                        };

                        entry
                    }
                    None => {
                        let entry = StructuredLogEntry {
                            level: last_log_level,
                            prefix: None,
                            message: line.to_string(),
                        };

                        entry
                    }
                }
            } else {
                let entry = StructuredLogEntry {
                    level: last_log_level,
                    prefix: None,
                    message: line.to_string(),
                };

                entry
            };

            structured_lines.push(entry);
        }

        structured_lines
    }

    pub fn structure_entries(&self, entries: Vec<LogEntry>) -> Vec<StructuredLogEntry> {
        let mut structured_entries = Vec::new();

        for entry in entries {
            let level = entry.level;

            let delimiter: Option<&str> = match level {
                LogLevel::Info => Some(&self.delimiters.info),
                LogLevel::Warn => Some(&self.delimiters.warn),
                LogLevel::Error => Some(&self.delimiters.error),
                LogLevel::Fatal => Some(&self.delimiters.fatal),
                LogLevel::Severe => Some(&self.delimiters.severe),
                LogLevel::Debug => Some(&self.delimiters.debug),
                LogLevel::Trace => Some(&self.delimiters.trace),
                LogLevel::Warning => Some(&self.delimiters.warning),
                LogLevel::Custom | LogLevel::Unknown => None,
            };

            match delimiter {
                Some(deli) => {
                    let splitted = entry.content.split_once(deli).unwrap();
                    let (prefix, message) = splitted;
                    let prefix = format!("{}{}", prefix, deli);

                    let entry = StructuredLogEntry {
                        level,
                        prefix: Some(prefix),
                        message: message.to_string(),
                    };

                    structured_entries.push(entry);
                }
                None => {
                    let entry = StructuredLogEntry {
                        level,
                        prefix: None,
                        message: entry.content,
                    };

                    structured_entries.push(entry);
                }
            }
        }
        structured_entries
    }

    /// Parses log lines into log entries
    ///
    /// A line containing a log level (e.g. `INFO`, `ERROR`, ...) always starts a new entry. Lines without log level belong to the entry above them.
    /// Lines at the very beggining of the input before any line with a log level form their own entry with [`LogLevel::Unknown`].
    ///
    /// That means e.g. error with a stacktrace is 1 entry.
    ///
    /// For example snippet below contains 3 entries with following log levels:
    /// 1) INFO (1st line)
    /// 2) ERROR (lines 2-15)
    /// 3) INFO (lines 16-17)
    ///
    /// ```log
    /// 1 [15:14:39] [ServerMain/INFO]: [PluginInitializerManager] Initializing plugins...
    /// 2 [15:14:39] [ServerMain/ERROR]: [DirectoryProviderSource] Error loading plugin: java.lang.IllegalArgumentException: Directory 'plugins/.paper-remapped/citizensapi-2.0.33-SNAPSHOT-sources.zjar' does not contain a paper-plugin.yml or plugin.yml! Could not determine plugin type, cannot load a plugin from it!
    /// 3 java.lang.RuntimeException: java.lang.IllegalArgumentException: Directory 'plugins/.paper-remapped/citizensapi-2.0.33-SNAPSHOT-sources.jar' does not contain a paper-plugin.yml or plugin.yml! Could not determine plugin type, cannot load a plugin from it!
    /// 4 	at io.papermc.paper.plugin.provider.source.FileProviderSource.registerProviders(FileProviderSource.java:85) ~[purpur-1.21.4.jar:1.21.4-2416-51aafbc]
    /// 5 	at io.papermc.paper.plugin.provider.source.DirectoryProviderSource.registerProviders(DirectoryProviderSource.java:52) ~[purpur-1.21.4.jar:1.21.4-2416-51aafbc]
    /// 6 	at io.papermc.paper.plugin.provider.source.DirectoryProviderSource.registerProviders(DirectoryProviderSource.java:17) ~[purpur-1.21.4.jar:1.21.4-2416-51aafbc]
    /// 7 	at io.papermc.paper.plugin.util.EntrypointUtil.registerProvidersFromSource(EntrypointUtil.java:15) ~[purpur-1.21.4.jar:1.21.4-2416-51aafbc]
    /// 8 	at io.papermc.paper.plugin.PluginInitializerManager.load(PluginInitializerManager.java:113) ~[purpur-1.21.4.jar:1.21.4-2416-51aafbc]
    /// 9 	at net.minecraft.server.Main.main(Main.java:117) ~[purpur-1.21.4.jar:1.21.4-2416-51aafbc]
    /// 10 	at io.papermc.paper.PaperBootstrap.boot(PaperBootstrap.java:21) ~[purpur-1.21.4.jar:1.21.4-2416-51aafbc]
    /// 11 	at org.bukkit.craftbukkit.Main.main(Main.java:288) ~[purpur-1.21.4.jar:1.21.4-2416-51aafbc]
    /// 12 	at io.papermc.paperclip.Paperclip.lambda$main$0(Paperclip.java:42) ~[app:?]
    /// 13 	at java.base/java.lang.Thread.run(Unknown Source) ~[?:?]
    /// 14 Caused by: java.lang.IllegalArgumentException: Directory 'plugins/.paper-remapped/citizensapi-2.0.33-SNAPSHOT-sources.jar' does not contain a paper-plugin.yml or plugin.yml! Could not determine plugin type, cannot load a plugin from it!
    /// 15 	... 10 more
    /// 16 [15:14:39] [ServerMain/INFO]: [PluginInitializerManager] Paper plugins (1):
    /// 17 - Minepacks (2.4.27-T20240129235629)
    /// ```
    pub fn parse_entries(&self) -> Vec<LogEntry> {
        let mut entries = Vec::new();

        let mut last_log_level = LogLevel::Unknown;
        let mut last_line_with_log_level: usize = 0;

        let line_limit = self.line_limit.unwrap_or(usize::MAX);

        for (idx, line) in self.lines.iter().take(line_limit).enumerate() {
            let line = line.as_ref();
            let next_line = self.lines.get(idx + 1).map(|l| l.as_ref());
            let is_last_line = next_line.is_none();

            if line.contains(&self.delimiters.info) {
                last_log_level = LogLevel::Info;
            } else if line.contains(&self.delimiters.warn) {
                last_log_level = LogLevel::Warn;
            } else if line.contains(&self.delimiters.warning) {
                last_log_level = LogLevel::Warning;
            } else if line.contains(&self.delimiters.error) {
                last_log_level = LogLevel::Error;
            } else if line.contains(&self.delimiters.severe) {
                last_log_level = LogLevel::Severe;
            } else if line.contains(&self.delimiters.fatal) {
                last_log_level = LogLevel::Fatal;
            } else if line.contains(&self.delimiters.debug) {
                last_log_level = LogLevel::Debug;
            } else if line.contains(&self.delimiters.trace) {
                last_log_level = LogLevel::Trace;
            }

            let next_line_has_level = next_line.is_some_and(|nl| self.line_contains_log_level(nl));

            if self.line_contains_log_level(line) {
                last_line_with_log_level = idx;

                if is_last_line || next_line_has_level {
                    let content = line.to_string();

                    let entry = LogEntry {
                        level: last_log_level,
                        content,
                    };

                    entries.push(entry);
                }
            } else {
                if is_last_line || next_line_has_level {
                    let content: String = self.lines[last_line_with_log_level..=idx]
                        .iter()
                        .map(|l| l.as_ref())
                        .collect::<Vec<&str>>()
                        .join("\n");

                    let entry = LogEntry {
                        level: last_log_level,
                        content,
                    };

                    entries.push(entry);
                }
            }
        }

        entries
    }

    pub fn html_lines(&self) -> Vec<String> {
        let structured_lines = self.structure_lines();

        let mut html = Vec::new();

        let mut id: usize = 1;

        for line in structured_lines {
            let log_level = match line.level {
                LogLevel::Info | LogLevel::Debug => "i",
                LogLevel::Error | LogLevel::Fatal | LogLevel::Severe => "e",
                LogLevel::Warn | LogLevel::Warning => "w",
                LogLevel::Custom => "c",
                LogLevel::Trace => "t",
                LogLevel::Unknown => "u",
            };

            let html_part = match line.level {
                LogLevel::Info | LogLevel::Debug | LogLevel::Trace => format!(
                    r#"<span class="p" id="L{}"><span class={}>{}</span>{}</span>"#,
                    id,
                    log_level,
                    html_escape::encode_text(&line.prefix.unwrap_or_default()),
                    html_escape::encode_text(&line.message),
                ),
                LogLevel::Warn
                | LogLevel::Error
                | LogLevel::Warning
                | LogLevel::Fatal
                | LogLevel::Severe
                | LogLevel::Custom
                | LogLevel::Unknown => {
                    format!(
                        r#"<span class="p" id="L{}"><span class="{}">{}{}</span></span>"#,
                        id,
                        log_level,
                        html_escape::encode_text(&line.prefix.unwrap_or_default()),
                        html_escape::encode_text(&line.message),
                    )
                }
            };

            html.push(html_part);
            id += 1;
        }

        html
    }

    pub fn html_entries(&self) -> Vec<String> {
        let entries = self.parse_entries();
        let entries = self.structure_entries(entries);

        let mut html = Vec::new();

        let mut id: usize = 1;

        for entry in entries {
            let log_level = match entry.level {
                LogLevel::Info | LogLevel::Debug => "i",
                LogLevel::Error | LogLevel::Fatal | LogLevel::Severe => "e",
                LogLevel::Warn | LogLevel::Warning => "w",
                LogLevel::Custom => "c",
                LogLevel::Trace => "t",
                LogLevel::Unknown => "u",
            };

            let html_part = match entry.level {
                LogLevel::Info | LogLevel::Debug | LogLevel::Trace => format!(
                    r#"<span class="p" id="L{}"><span class={}>{}</span>{}</span>"#,
                    id,
                    log_level,
                    html_escape::encode_text(&entry.prefix.unwrap_or_default()),
                    html_escape::encode_text(&entry.message),
                ),
                LogLevel::Warn
                | LogLevel::Error
                | LogLevel::Warning
                | LogLevel::Fatal
                | LogLevel::Severe
                | LogLevel::Custom
                | LogLevel::Unknown => {
                    format!(
                        r#"<span class="p" id="L{}"><span class="{}">{}{}</span></span>"#,
                        id,
                        log_level,
                        html_escape::encode_text(&entry.prefix.unwrap_or_default()),
                        html_escape::encode_text(&entry.message),
                    )
                }
            };

            html.push(html_part);
            id += 1;
        }

        html
    }

    /// Returns true if log line has log level (e.g. `INFO`, `WARN`, `ERROR`, ...).
    ///
    /// ```log
    /// [15:14:39] [ServerMain/INFO]: [PluginInitializerManager] Initializing... -> true
    ///
    /// java.lang.RuntimeException: java.lang.IllegalArgumentException: ... -> false
    ///
    /// ATTENTION! ERROR OCCURED -> true if `Delimiters.error` is just `ERROR`, true otherwise.
    /// ```
    pub fn line_contains_log_level(&self, line: impl AsRef<str>) -> bool {
        let line = line.as_ref();

        return line.contains(&self.delimiters.info)
            || line.contains(&self.delimiters.warn)
            || line.contains(&self.delimiters.error)
            || line.contains(&self.delimiters.fatal)
            || line.contains(&self.delimiters.severe)
            || line.contains(&self.delimiters.warning)
            || line.contains(&self.delimiters.debug)
            || line.contains(&self.delimiters.trace);
    }
}

/// Structure for holding delimiters (e.g. `INFO]:` for `Delimiters.info`) between log line prefix (e.g. `[15:14:39] [ServerMain/INFO]:`) and the message (e.g. `[bootstrap] Running Java 21 (OpenJDK ...`) for all log levels.
///
/// Get the delimiters using [get_delimiters] function.
///
/// Used in [line_contains_log_level] and [parse_entries].
///
/// # Description
/// Returns delimiters for all log levels, e.g. `INFO]:` for `Delimiters.info` when `]:` suffix is provided.
///
/// You can use [find_log_level_suffix] for finding log's suffix automatically.
///
/// ```rs
/// fn get_delimiters(suffix: Option<String>) -> Delimiters {
///     let suffix = suffix.unwrap_or_else(|| "".to_string());
///
///     Delimiters {
///         info: format!("INFO{}", suffix),
///         warn: format!("WARN{}", suffix),
///         error: format!("ERROR{}", suffix),
///         fatal: format!("FATAL{}", suffix),
///         severe: format!("SEVERE{}", suffix),
///         warning: format!("WARNING{}", suffix),
///         debug: format!("DEBUG{}", suffix),
///         trace: format!("TRACE{}", suffix),
///     }
/// }
/// ```
fn get_delimiters(suffix: Option<String>) -> Delimiters {
    let sfx = suffix.clone().unwrap_or_else(|| "".to_string());

    Delimiters {
        info: format!("INFO{}", sfx),
        warn: format!("WARN{}", sfx),
        error: format!("ERROR{}", sfx),
        fatal: format!("FATAL{}", sfx),
        severe: format!("SEVERE{}", sfx),
        warning: format!("WARNING{}", sfx),
        debug: format!("DEBUG{}", sfx),
        trace: format!("TRACE{}", sfx),
    }
}

// TODO used AI for this to be able to move to other functions, rewrite this slop when finished
const LOG_LEVELS: &[&str] = &[
    "TRACE", "DEBUG", "INFO", "WARN", "ERROR", "FATAL", "WARNING", "SEVERE",
];

fn level_suffix_in_line(line: &str) -> Option<&str> {
    let (pos, level) = LOG_LEVELS
        .iter()
        .filter_map(|level| line.find(level).map(|pos| (pos, *level)))
        .min_by_key(|(pos, _)| *pos)?;

    let rest = &line[pos + level.len()..];
    let end = rest.find(char::is_whitespace).unwrap_or(rest.len());
    let suffix = &rest[..end];

    if suffix.is_empty() {
        None
    } else {
        Some(suffix)
    }
}

/// Returns substring between log level keyword (e.g. `INFO`) and first whitespace behind it.
///
/// For example
/// ```log
/// [15:14:39] [INFO] [bootstrap] ... -> Some("]")
///
/// [15:14:39] [ServerMain/INFO]: [bootstrap] ... -> Some("]:")
///
/// [15:14:39] INFO [bootstrap] ... -> None
/// ```
///
/// # Arguments
/// This function will scan up to `line_limit` lines (all if `None`) and will return the suffix after finding it `delimiter_match_limit` times/the most frequent one is returned if `delimiter_match_limit` is not reached.
///
/// # Purpose of the function
/// After finding the log level suffix (meaning the log format) you can easily check for log levels without false positives (meaning messages containing log level keyword but not really being log level), used in e.g. [parse_entries]) or parse log lines into prefix (e.g. `[15:14:39] [ServerMain/INFO]:`) and the message (e.g. `[bootstrap] Running Java 21 (OpenJDK ...`) since `{LOG_LEVEL}{suffix}` is the delimiters.
pub fn find_log_level_suffix(
    lines: &[impl AsRef<str>],
    line_limit: Option<usize>,
    delimiter_match_limit: Option<usize>,
) -> Option<String> {
    let mut suffix_count: HashMap<Option<String>, usize> = HashMap::new();

    const LOG_LEVELS: &[&str] = &[
        "TRACE", "DEBUG", "INFO", "WARN", "ERROR", "FATAL", "WARNING", "SEVERE",
    ];

    for line in lines.iter().take(line_limit.unwrap_or(usize::MAX)) {
        let line_str = line.as_ref();

        let has_log_level = LOG_LEVELS.iter().any(|level| line_str.contains(level));
        if !has_log_level {
            continue;
        }

        let sfx = level_suffix_in_line(line_str).map(str::to_owned);

        let count = suffix_count.entry(sfx.clone()).or_insert(0);
        *count += 1;

        if *count >= delimiter_match_limit.unwrap_or(usize::MAX) {
            return sfx;
        }
    }

    suffix_count
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .and_then(|(suffix, _)| suffix)
}
