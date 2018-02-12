use std::convert::AsRef;

pub struct Logger;

pub struct LogEntry(String);

impl LogEntry {
    fn new(s: &str) -> LogEntry {
        LogEntry(s.to_string())
    }
}

impl AsRef<str> for LogEntry {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Logger {
    pub fn new() -> Logger {
        Logger
    }
    
    pub fn log(&self, s: &str) -> LogEntry {
        LogEntry::new(s)
    }
}

pub fn do_a_bunch_of_things(logger: &Logger, things: &[&str]) -> Vec<LogEntry> {
    things.iter().map(|s| logger.log(s)).collect()
}
