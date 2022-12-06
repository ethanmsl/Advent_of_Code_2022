//! # Explore.rs
//!
//! Just here so I can play with input and output types
//! And only linked to main.rs for exploratory run purposes

/// # just a rust file connection test
/// print a line from `main.rs` to make sure this file is being sourced
/// ## Example
/// print_to_test_sourcing();
/// (lol)
///
///
/// The next lines present detailed documentation. Code blocks start with
/// triple backquotes and have implicit `fn main()` inside
/// and `extern crate <cratename>`. 
///
/// ```
/// let result = 2+3;
/// assert_eq!(result, 5);
/// ```
pub fn print_to_test_sourcing(num: i32) {
    println!("------------function defined in Explore.rs------------");
    println!("The input number is: {}", num);
}


/// *le sigh*  
// trying to get documentation tests up and running
/// ```
/// let result = explore::add_three(1, 2, 3);
/// assert_eq!(result, 6);
/// ```
pub fn add_three (n1: i32, n2: i32, n3: i32) -> i32 {
    n1 + n2 + n3
}
