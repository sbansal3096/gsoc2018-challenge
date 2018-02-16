use std::convert::AsRef;

pub trait LogEntryTrait{
    fn new(s: &str)-> Self;
}

pub trait LoggerTrait<U:LogEntryTrait+AsRef<str> >{
    //type LogEnt: LogEntr;
    fn new()->Self;
    fn log(&self, s: &str) -> U;
}