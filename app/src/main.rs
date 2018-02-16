extern crate base;
extern crate middle;

use std::convert::AsRef;
use base::{do_a_bunch_of_things,Logger,LogEntry};
use middle::create_middleware;

fn main() {
    let mut m = create_middleware::<Logger,LogEntry>();
    m.log_violation("invalid");
    m.log_violation("unauthorized");
    let violations = m.take_violations();
    for v in violations {
        println!("{}", v.as_ref());
    }

    let logger = m.take_logger();
    let entries = do_a_bunch_of_things(&logger, &["hello", "world"]);
    for e in entries {
        println!("{}", e.as_ref());
    }
}
