extern crate baseapi;

use baseapi::{Logger, LogEntry};
use std::mem;

pub struct Middleware {
    logger: Logger,
    violations: Vec<LogEntry>,
}

impl Middleware {
    fn new(logger: Logger) -> Middleware {
        Middleware {
            logger,
            violations: vec![]
        }
    }

    pub fn log_violation(&mut self, s: &str) {
        self.violations.push(self.logger.log(s));
    }

    pub fn take_violations(&mut self) -> Vec<LogEntry> {
        mem::replace(&mut self.violations, vec![])
    }

    pub fn take_logger(self) -> Logger {
        self.logger
    }
}

pub fn create_middleware() -> Middleware {
    Middleware::new(Logger::new())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut m = create_middleware();
        m.log_violation("hi");
        m.log_violation("byte");
        let vs: Vec<_> = m.take_violations().into_iter().map(|v| v.as_ref()).collect();
        assert_eq!(&vs, &["hi", "bye"]);
    }
}
