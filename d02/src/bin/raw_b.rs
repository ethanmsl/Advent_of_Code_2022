//! Advent of Code 2022 - Day 02 - Part B

use d02::raw_lib;

fn main() {
    let input = raw_lib::get_input();
    let vec = raw_lib::to_gt_pair(&input);
    let vec = raw_lib::strat_to_rounds(&vec);
    let solution = raw_lib::score_rounds(vec);

    println!("Solution: {}", solution);
}

