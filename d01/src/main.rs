//! Intended to be used with the "clap" crate as an alternate way to access
//! various implementations (& expermeint with the clap crate :)
//!
//! # NOTE:
//! Originally this was a `main.rs` file, but I was not able to get cargo to run it...

use clap::Parser;
use d01::{iodiom_lib, raw_lib, CliArgs, Phase, Problem};

fn main() {
    let args = CliArgs::parse();
    println!("args: {:?}", args);

    match args {
        CliArgs {
            phase: Phase::Raw,
            problem: Problem::A,
        } => {
            let input = raw_lib::get_input();
            let elf_vec = raw_lib::sum_chunks_of_string(&input);
            let max_elf_val = raw_lib::give_max(&elf_vec);
            println!("max_elf_val: {}", max_elf_val);
        }
        CliArgs {
            phase: Phase::Raw,
            problem: Problem::B,
        } => {
            let input = raw_lib::get_input();
            let elf_vec = raw_lib::sum_chunks_of_string(&input);
            let sum_top_3 = raw_lib::give_top_three_sum(elf_vec);
            println!("sum_top_3: {}", sum_top_3);
        }
        CliArgs {
            phase: Phase::Idiom,
            problem: Problem::A,
        } => {
            let answer = iodiom_lib::solution_a();
            println!("Day_01, Part_01 answer: {answer:?}")
        }
        CliArgs {
            phase: Phase::Idiom,
            problem: Problem::B,
        } => {
            let answer = iodiom_lib::solution_b();
            println!("Day_01, Part_02 answer: {answer:?}")
        }
    }
}
