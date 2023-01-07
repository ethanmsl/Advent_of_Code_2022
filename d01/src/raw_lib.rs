//! raw_lib

// read input from a text file
use std::fs;

pub fn get_input() -> String {
    let input = fs::read_to_string("input").expect("Error reading input file");
    input
}

pub fn sum_chunks_of_string(input: &str) -> Vec<u64> {
    // split by lines; return iterator
    let lines = input.lines();

    // []
    let mut elf_vec: Vec<u64> = Vec::new();

    // for line in lines.take(100) {
    //      match line {
    //          "" => println!(" '', {}", line),
    //          _ => println!(" _, {}", line),
    //      }
    // }

    // sum all sections that are contiguous wrt containing numbers
    let mut accumulator: u64 = 0;
    for line in lines {
        match line {
            // blank line divider
            "" => {
                // under the assumption of a single blank line divider
                // contiguous sections: push accumulated sum to vec
                elf_vec.push(accumulator);
                accumulator = 0;
            }
            // something readable as u64
            _ => {
                accumulator += line.parse::<u64>().unwrap();
            }
        }
    }

    elf_vec
}

/////////////////////// Part_1 ///////////////////////
pub fn give_max<T: Ord + Copy>(elf_vec: &Vec<T>) -> T {
    // retun max value
    let max_elf_val = elf_vec.iter().max().unwrap();
    // let x = *max_elf_val;
    // x.clone()
    *max_elf_val // derefing
}

/////////////////////// Part_2 ///////////////////////
pub fn give_top_three_sum<T: Ord + Copy + std::iter::Sum>(elf_vec: Vec<T>) -> T {
    // return sum top 3 values of vector
    // apparently I can't chain the sort & reverse since they don't return the vector
    let mut sort_vec = elf_vec.clone();
    sort_vec.sort();
    sort_vec.reverse();
    // sum the top three values
    let sum_top_3: T = sort_vec[0..3].iter().copied().sum::<T>();
    //                                        ^ needed otherwise
    //                              complaints that Sum over &T not implemented
    //                              seems very strange to me that it would operate
    //                              on T and not &T
    //                              ... perhaps there are good reasons ...
    //                              (if I followed its syntax suggestions to do so
    //                              it then required lifetime annotations
    //                              and its suggested fixes were *quite* strict!)

    sum_top_3
}
