extern crate baseapi;

use std::convert::AsRef;
use baseapi::{LogEntryTrait, LoggerTrait};
use std::mem;

struct Logger;

pub trait MiddlewareTrait<T:LoggerTrait<U> ,U:LogEntryTrait+AsRef<str> >{
    type associatedType1 :LoggerTrait<U>;
    fn new(logger: T)->Self;
    fn log_violation(&mut self, s: &str); 
    fn take_violations(&mut self) -> Vec<U>; 
    fn take_logger(self) -> T;
}


pub struct Middleware<T:LoggerTrait<U>, U: LogEntryTrait+AsRef<str> > {
    logger: T,
    violations: Vec<U>,
    }

impl<T:LoggerTrait<U> ,U:LogEntryTrait+AsRef<str> > MiddlewareTrait<T,U> for Middleware<T,U> {
    type assocuatedType1 = Logger;
    fn new(logger: T) -> Middleware<T,U> {
        Middleware {
            logger,
            violations: vec![]
        }
    }

    fn log_violation(&mut self, s: &str) {
            self.violations.push(self.logger.log(s));
    }

    fn take_violations(&mut self) -> Vec<U> {
        mem::replace(&mut self.violations, vec![])
    }

    fn take_logger(self) -> T {
        self.logger
    }
}

pub fn create_middleware<T:LoggerTrait<U>, U:LogEntryTrait+AsRef<str> >() -> Middleware<T,U> {
    Middleware::new(T::new())
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