/// add two numbers
///
/// # Examples
///
/// ```
/// let result = test_comments::add(2, 4);
/// assert_eq!(5, result);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test_comments() {
    //line comment
    assert!(true); /*block comment*/
}

#[test]
fn test_add() {
    assert_eq!(5, add(2, 3));
}
