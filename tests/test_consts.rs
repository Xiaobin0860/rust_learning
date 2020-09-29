const fn fib(n: i32) -> i32 {
    // println!("n={}", n);
    if n < 3 {
        1
    } else {
        fib(n - 2) + fib(n - 1)
    }
}

#[test]
fn test_consts() {
    assert_eq!(1, fib(1));
    assert_eq!(1, fib(2));
    assert_eq!(2, fib(3));
    assert_eq!(3, fib(4));
    assert_eq!(5, fib(5));
    assert_eq!(8, fib(6));
    assert_eq!(55, fib(10));
}
