/// raw_2 executor
use d01::raw_lib;

fn main() {
    let input = raw_lib::get_input();
    let elf_vec = raw_lib::sum_chunks_of_string(&input);
    let sum_top_3 = raw_lib::give_top_three_sum(elf_vec);
    println!("sum_top_3: {}", sum_top_3);
}
