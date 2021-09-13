//! # Crate Test
//! crate练习包

/// 将传入的数字加1
/// # Examples
/// 
/// ```
/// let arg = 5;
/// let answer = crate_test::add_one(arg);
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}