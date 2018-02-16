use std::convert::AsRef;

pub trait LogEntryTrait{
    fn new(s: &str)-> Self;
}

pub trait LoggerTrait<U:LogEntryTrait+AsRef<str> >{
    fn new()->Self;
    fn log(&self, s: &str) -> U;
}