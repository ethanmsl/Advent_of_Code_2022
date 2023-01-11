//! Advent of Code 2022 - Day 02 - Part A

use d02::raw_lib;

fn main() {
    let input = raw_lib::get_input();
    let vec = raw_lib::to_throw_pairs(&input);
    let solution = raw_lib::score_rounds(vec);

    println!("Solution: {}", solution);
}
