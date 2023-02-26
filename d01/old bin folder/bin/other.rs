//! other
//! just playing with things

use d01::explore;

fn main() {
    {
        // Just hooking up `explore.rs`
        let a = 12 + 3;
        let b = 3 + a;
        let c: i32 = 17;

        let sum = explore::add_three(a, b, c);
        explore::print_to_test_sourcing(sum);
    }

    let a_num = explore::add_three(2, 3, 4);
    explore::print_to_test_sourcing(a_num);
}
