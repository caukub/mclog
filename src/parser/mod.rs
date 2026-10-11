use std::ops::Deref;

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Custom,
    Unknown,
}

#[derive(Debug)]
pub struct LogEntry {
    level: LogLevel,
    content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogLine(String);

impl From<&str> for LogLine {
    fn from(value: &str) -> Self {
        LogLine(value.to_string())
    }
}

impl From<String> for LogLine {
    fn from(value: String) -> Self {
        LogLine(value)
    }
}

impl Deref for LogLine {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn find_log_level_suffix(limit: Option<usize>, match_limit: Option<usize>) -> Option<String> {
    None
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
/// ```
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
pub fn parse_entries(lines: &[impl AsRef<str>]) -> Vec<LogEntry> {
    let mut entries = Vec::new();

    let mut latest_log_level = LogLevel::Unknown;
    let mut last_line_with_log_level: usize = 0;

    for (idx, line) in lines.iter().enumerate() {
        let line = line.as_ref();
        let next_line = lines.get(idx + 1).map(|l| l.as_ref());
        let is_last_line = next_line.is_none();

        if line.contains("INFO") {
            latest_log_level = LogLevel::Info;
        } else if line.contains("WARN") || line.contains("WARNING") {
            latest_log_level = LogLevel::Warn;
        } else if line.contains("ERROR") || line.contains("SEVERE") || line.contains("FATAL") {
            latest_log_level = LogLevel::Error;
        }

        let next_line_has_level = next_line.is_some_and(|nl| line_contains_log_level(nl));

        if line_contains_log_level(line) {
            last_line_with_log_level = idx;

            if is_last_line || next_line_has_level {
                let entry = LogEntry {
                    level: latest_log_level,
                    content: line.to_string(),
                };

                entries.push(entry);
            }
        } else {
            if is_last_line || next_line_has_level {
                let content: String = lines[last_line_with_log_level..=idx]
                    .iter()
                    .map(|l| l.as_ref())
                    .collect::<Vec<&str>>()
                    .join("\n");

                let entry = LogEntry {
                    level: latest_log_level,
                    content,
                };

                entries.push(entry);
            }
        }
    }

    entries
}

pub fn line_contains_log_level(line: impl Into<LogLine>) -> bool {
    let line = line.into();

    // todo debug, trace, kontrolovat jen prvnich x znaku?

    return line.contains("INFO")
        || line.contains("WARN")
        || line.contains("ERROR")
        || line.contains("FATAL")
        || line.contains("SEVERE")
        || line.contains("WARNING");
}
