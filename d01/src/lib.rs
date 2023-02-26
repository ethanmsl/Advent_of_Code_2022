//! This is *only* here to allow documentation tests
//! Unforunately, it seems that documentation tests are run
//! by compiling the library and then referncing into it, as per normal.
//! This means that we can *only* run tests on **Public** functions.
//! Unlike typical code tests, the documentation tests are *not*
//! run in the scope of module where they are written.

use clap::{Parser, ValueEnum};

pub mod explore;
pub mod iodiom_lib;
pub mod raw_lib;

/// arguments struct
#[derive(Parser, Debug)]
pub struct CliArgs {
    #[arg(value_enum)]
    pub phase: Phase,
    #[arg(value_enum)]
    pub problem: Problem,
}

/// Phase of solution
/// Either "Raw" -- solving the problem with no outside reference
/// Or "Idiom" -- implemented after consulting idiomatic solutions to the same problem
#[derive(ValueEnum, Clone, Debug)]
pub enum Phase {
    Raw,
    Idiom,
}

/// Which Problem is being solved
/// Section A of the problem
/// Or Section B
/// (not a ton to say here :shrug: :)
#[derive(ValueEnum, Clone, Debug)]
pub enum Problem {
    A,
    B,
}
