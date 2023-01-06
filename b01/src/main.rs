// read input from a text file
use std::fs;

fn main() {
    let input = fs::read_to_string("input").as_ref().unwrap();

    // .expect("Error reading input file");
    let lines = input.lines();

    // []
    let mut elf_vec: Vec<u64> = Vec::new();

    // for line in lines.take(100) {
    //      match line {
    //          "" => println!(" '', {}", line),
    //          _ => println!(" _, {}", line),
    //      }
    // }

    let mut accumulator: u64 = 0;
    for line in lines {
        match line {
            "" => {
                elf_vec.push(accumulator);
                accumulator = 0;
            }
            _ => {
                accumulator += line.parse::<u64>().unwrap();
            }
        }
    }

    // retun max value
    let max_elf_val = elf_vec.iter().max().unwrap();
    println!("max_elf_val: {}", max_elf_val);

    // return sum top 3 values of vector
    // apparently I can't chain the sort & reverse since they don't return the vector
    elf_vec.sort();
    elf_vec.reverse();
    let sum_top_3 = elf_vec[0..3].iter().sum::<u64>();
    println!("sum_top_3: {}", sum_top_3);
}
