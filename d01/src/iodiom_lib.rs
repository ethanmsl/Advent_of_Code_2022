//! idiom_lib
//! More idiomatic rust based on other people's approaches
//! To contrast with whatever answer I clomped together and highlight growth opps

use itertools::Itertools;
use std::cmp::Reverse;

pub fn solution_a() -> u64 {
    // Advent_of_Code 01_a
    let answer = include_str!("../input")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|mut it| (&mut it).map_while(|x| x).sum1::<u64>())
        .map(Reverse)
        .k_smallest(1)
        .map(|x| x.0)
        .sum::<u64>();

    answer
}

pub fn solution_b() -> u64 {
    // Advent_of_Code 01_b
    // Hmmmmm..... re: `Reverse`
    // looking into where it's defined it seems like it may
    // be playing with the `PartialOrd` trait ... that might be close enough to the bone
    // that it becomes reasonable ... I must consider and look into further...
    // Advent_of_Code 01_a
    let answer = include_str!("../input")
        // str_slice ~~> iterator (with properties)
        .lines()
        // iter_str_slice ~~> iter_<u64>Result (unsigned, no celery in elves' packs!)
        // iter_u<64>Result ~~> iter<u64>Option (Errors ~~> None)
        .map(|v| v.parse::<u64>().ok())
        // using the fact that "" return `None` by dint of the .ok()
        // and thereby creating partial runs on iterators
        // and then collecting those into ... a 'Batching'
        // (and Note: that the defacto delimters are tossed as a result of
        // the micro-batcher reading them, but not using them (re: `map_while`)
        .batching(|mut it| (&mut it).map_while(|x| x).sum1::<u64>())
        //
        // this is PERVERSE
        // the interaction between Reverse (on single numbers at this point)
        // and k_smallest to get the largest is unclear
        // presumably there's some sort of ordering function being used
        // and k_smallest ...somehow... interacts with that.
        // ... looking further into docs that's how `Reverse` is intended to work
        // it's meant to interface with comparisons somewhere
        // (almost certainly should NOT exist to be used)
        .map(Reverse)
        .k_smallest(3)
        .map(|x| x.0)
        .sum::<u64>();

    answer
}

#[allow(dead_code)]
fn explore() {
    {
        let boop = (1..=3).sum1::<i32>();
        println!("{boop:?}");
    }

    {
        println!("{:?}", 12_i32.checked_div(2));
        println!("{:?}", 12_i32.checked_div(0));
    }
    {
        // checking the results step-by-step
        let answer = "12\n13\n\n5\n\n3456\n245";
        let answer = answer.lines();
        let answer = answer.map(|v| v.parse::<u64>().ok());
        let answer = answer.batching(|mut it| (&mut it).map_while(|x| x).sum1::<u64>());
        dbg!(answer.clone().collect::<Vec<_>>());

        let answer = answer.map(Reverse);
        dbg!(answer.clone().collect::<Vec<_>>());

        let answer = answer.k_smallest(2);
        dbg!(answer.clone().collect::<Vec<_>>());

        let answer = answer.map(|x| x.0);
        dbg!(answer.clone().collect::<Vec<_>>());

        let answer = answer.collect::<Vec<_>>();
        println!("example run {:?}", answer);
    }
    {
        println!("\n-----------------------------------------------\n");
        let mut v = vec![1_i32, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        v.sort_by_key(|x| *x);
        println!("v, sorted: {:?}", v);
        println!("\n-----------------------------------------------\n");
    }
}
