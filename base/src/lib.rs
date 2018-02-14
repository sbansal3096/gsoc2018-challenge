extern crate baseapi;

use baseapi::{Logger, LogEntry};

	
pub fn do_a_bunch_of_things(logger: &Logger, things: &[&str]) -> Vec<LogEntry> {
    things.iter().map(|s| logger.log(s)).collect()
}
