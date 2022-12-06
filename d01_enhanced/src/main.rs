//! main.rs docs
//!

mod explore;

fn main() {
    let a = 12 + 3;
    let b = 3 + a;
    let c: i32 = 17;

    let sum = explore::add_three(a, b, c);
    explore::print_to_test_sourcing(sum);
}
