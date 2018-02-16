extern crate base;
extern crate middle;

use base::{do_a_bunch_of_things,AsRef,as_ref};
use middle::create_middleware;

fn main() {
    let mut m = create_middleware();
    m.log_violation("invalid");
    m.log_violation("unauthorized");
    let violations = m.take_violations();
    for v in violations {
        //let d1:<U:=
        //let d=v.as_ref();
        println!("{}", v.as_ref());
    }

    let logger = m.take_logger();
    let entries = do_a_bunch_of_things(&logger, &["hello", "world"]);
    for e in entries {
        println!("{}", e.as_ref());
    }
}
