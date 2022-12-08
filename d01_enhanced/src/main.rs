//! main.rs docs
//! More idiomatic rust based on other people's approaches
//! To contrast with whatever answer I clomped together and highlight growth opps

mod explore;
use itertools::Itertools;
use std::cmp::Reverse;

fn main() {
    {
        // Just hooking up `explore.rs`
        let a = 12 + 3;
        let b = 3 + a;
        let c: i32 = 17;

        let sum = explore::add_three(a, b, c);
        explore::print_to_test_sourcing(sum);
    }

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
        let answer = dbg!(answer).lines();
        let answer = dbg!(answer).map(|v| v.parse::<u64>().ok());
        let answer = dbg!(answer).batching(|mut it| (&mut it).map_while(|x| x).sum1::<u64>());
        let answer = dbg!(answer).map(Reverse);
        let answer = dbg!(answer).k_smallest(2);
        let answer = dbg!(answer).map(|x| x.0);
        let answer = dbg!(answer).collect::<Vec<_>>();
        let answer = dbg!(answer);
    }
    {
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
            .k_smallest(1)
            .map(|x| x.0)
            .sum::<u64>();
        println!("Day_01, Part_01 answer: {answer:?}")
    }
    {
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
        println!("Day_01, Part_02 answer: {answer:?}")
    }
}
