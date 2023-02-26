//! Intended to be used with the "clap" crate as an alternate way to access
//! various implementations (& expermeint with the clap crate :)
//!
//! # NOTE:
//! Originally this was a `main.rs` file, but I was not able to get cargo to run it...

use clap::Parser;
use d01::CliArgs;

fn main() {
    let args = CliArgs::parse();
    println!("args: {:?}", args);
}
