///
/// ## Expressions
///

#[allow(unused_must_use)]
#[test]
fn test_expressions() {
    let x = 5u32;
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        x_cube + x_squared + x
    };

    let z = {
        2 * x;
    };

    assert_eq!(155, y);
    assert_eq!((), z);
}
