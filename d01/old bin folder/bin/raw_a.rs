/// raw_1 executor
use d01::raw_lib;

fn main() {
    let input = raw_lib::get_input();
    let elf_vec = raw_lib::sum_chunks_of_string(&input);
    let max_elf_val = raw_lib::give_max(&elf_vec);
    println!("max_elf_val: {}", max_elf_val);
}
