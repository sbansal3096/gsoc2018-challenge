This project is a Cargo workspace set up as follows:

* `app/` is a Rust application that depends on the `middle` and `base` crates
* `middle/` is a Rust crate that depends on the `base` crate
* `base/` is a Rust crate that has no dependencies
* `Cargo.toml` declares all of the members of the workspace.

Any time we make a change to a file in `base/`, this causes the `middle` and `app`
crates to be rebuilt. We want to make a new `base-api` crate that contains as little
code as possible, and make `middle` depend on `base-api` instead of `base`. You will
want to use Rust features like traits with associated types and generic types in order
to accomplish this. Our goal is to avoid rebuilding the `middle` crate whenever we make
a change to a file in `base/`.