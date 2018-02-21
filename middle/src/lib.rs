extern crate baseapi;

use baseapi::{LoggerTrait};
use std::mem;


pub trait MiddlewareTrait<T:LoggerTrait >{

    fn new(logger: T)->Self;
    fn log_violation(&mut self, s: &str); 
    fn take_violations(&mut self) -> Vec<T::X>; 
    fn take_logger(self) -> T;
}


pub struct Middleware<T:LoggerTrait > {
    logger: T,
    violations: Vec<T::X>,
    }

impl<T:LoggerTrait > MiddlewareTrait<T> for Middleware<T> {

    fn new(logger: T) -> Middleware<T> {
        Middleware {
            logger,
            violations: vec![]
        }
    }

    fn log_violation(&mut self, s: &str) {
            self.violations.push(self.logger.log(s));
    }

    fn take_violations(&mut self) -> Vec<T::X> {
        mem::replace(&mut self.violations, vec![])
    }

    fn take_logger(self) -> T {
        self.logger
    }
}

pub fn create_middleware<T:LoggerTrait>() -> Middleware<T> {
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