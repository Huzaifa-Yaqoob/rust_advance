//! #My Crate
//!
//! `my_crate` is a collection of utilities to make performance certain calculations more convenient.


/// Adds one to the number
/// #Examples
///
/// ```
/// use cargo_crate::add_one;
/// let arg = 5;
/// let answer = my_crate::add_one();
///
/// assert_eq!(6, answer);
/// ```



pub fn add_one(x:i32) -> i32 {
    x + 1
}