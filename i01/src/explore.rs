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
/// trying to get documentation tests up and running
/// ```
/// use d01_enhanced::explore;
/// let result = explore::add_three(1, 2, 3);
/// assert_eq!(result, 6);
/// ```
pub fn add_three(n1: i32, n2: i32, n3: i32) -> i32 {
    n1 + n2 + n3
}

/// *le sigh* cont'd  
///
/// creating a private function to check that local testing still works with it
/// even though documentation testing apparently does not
/// ```ignore
/// let result = mult_three(1, 2, 3);
/// assert_eq!(result, 7);
/// ```
fn mult_three(n1: i32, n2: i32, n3: i32) -> i32 {
    n1 * n2 * n3
}

// some tests to check that scoping here is local
// , contrary to documentation-tests, which appear to compile the library and then
// call from it like an external caller.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// using public function
    fn test_add_three() {
        assert_eq!(add_three(1, 7, 3), 11);
    }

    #[test]
    /// using public function
    fn test_mult_three() {
        assert_eq!(mult_three(1, 7, 3), 21);
    }
}
