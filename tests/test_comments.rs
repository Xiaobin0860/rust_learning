/// add two numbers
///
/// # Examples
///
/// ```
/// let result = test_comments::add(2, 4);
/// assert_eq!(5, result);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[path = "../src/comments.rs"]
mod comments;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comments() {
        //line comment
        assert!(true); /*block comment*/

        assert_eq!(5, add(2, 3));
        assert_eq!(4, comments::add(1, 3));
    }
}
