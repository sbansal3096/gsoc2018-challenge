extern crate base;
extern crate middle;

use base::{do_a_bunch_of_things,Logger};
use middle::{create_middleware,MiddlewareTrait};

fn main() {
    let mut m = create_middleware::<Logger>();
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