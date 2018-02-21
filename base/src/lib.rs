extern crate baseapi;

use baseapi::{LoggerTrait,LogEntryTrait};
use std::convert::AsRef;

pub struct Logger;
pub struct LogEntry(String);

impl LoggerTrait for Logger{
    type X = LogEntry;
    fn new() -> Logger {
        Logger
    }

    fn log(&self, s: &str) -> Self::X {
        Self::X::new(s)
    }
}

impl LogEntryTrait for LogEntry {
    fn new(s: &str) -> LogEntry {
        LogEntry(s.to_string())
    }
}

impl AsRef<str> for LogEntry {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
pub fn do_a_bunch_of_things(logger: &Logger, things: &[&str]) -> Vec<LogEntry> {
    things.iter().map(|s| logger.log(s)).collect()
}
