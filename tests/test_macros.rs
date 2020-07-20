///
/// # macro_rules!
///
/// Rust provides a powerful macro system that allows metaprogramming.
///
macro_rules! say_hello {
    // `()` indicates that the macro taks no argument.
    () => {
        // The macro will expand into the contents of this block.
        println!("Hello");
    };
}
#[test]
fn test_macros() {
    say_hello!();
}

///
/// ## Syntax
///
/// ### Disignators
///
/// The arguments of a macro are prefixed by a `$` and type annotated with a designator:
/// These are some of  the available designator:assert_eq!
///
/// * `block`
/// * `expr` is used for expressings
/// * `ident` is used for variable/function names
/// * `item`
/// * `literal` is used for literal constants
/// * `pat`(pattern)
/// * `path`
/// * `stmt`(statement)
/// * `tt`(token tree)
/// * `ty`(type)
/// * `vis`(visibility qualifier)
///
macro_rules! create_function {
    // This macro takes an argument of designator `ident` and
    // creates a function named `$func_name`.
    // The `ident` designator is used for variable/function names.
    ($func_name:ident) => {
        fn $func_name() {
            // The `stringify!` macro converts an `ident` into a string
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    ($expression:expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

#[test]
fn test_designators() {
    foo();
    bar();
    print_result!(1u32 + 1);
    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    })
}

///
/// ### Overload
///
/// Macro can be overloaded to accept different combinations of arguments.
/// In that regard, `macro_rules!` can work similarly to a match block:
///
macro_rules! test {
    // Arguments don't need to be separated by a comma.
    // Any template can be used!
    ($left:expr; and $right:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        )
    };
    // Arguments don't need to be separated by a comma.
    // Any template can be used!
    ($left:expr; or $right:expr) => {
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        )
    };
}

#[test]
fn test_overload() {
    test!(1+1==2; and 2*2==4);
    test!(true; or false);
}

///
/// ### Repeat
///
/// Macros can use `+` in the argument list to indicate that an argument may repeat at least once,
/// or `*`, to indicate that argument may repeat zero or more times.
///
macro_rules! find_min {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr), +) => (
        std::cmp::min($x, find_min!($($y), +))
    )
}

#[test]
fn test_repeat() {
    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2, 2u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 4u32));
    let a = vec![1, 2, 3, 4];
    println!("1---- {:?} {:?}", a, a.len());
    println!("2---- {:?} {:?}", a, (a.len()));
    println!("3---- {:?} {:?}", a, (a.len(),));

    assert!(true, "{:?} {:?}, {:?}", a.len(), (a.len()), (a.len(),));
}

///
/// ## DRY
///
/// Macros allow writing DRY code by factoring out the common parts of functions and/or test suites.
///
use std::ops::{Add, Mul, Sub};
macro_rules! assert_equal_len {
    //The `tt`(token tree) designator is used for operators and tokens
    ($a:expr, $b:expr,$func:ident,$op:tt) => {
        assert!(
            $a.len() == $b.len(),
            "{:?}: dimension mismatch: {:?} {:?} {:?}",
            stringify!($func),
            ($a.len(),), //????
            stringify!($op),
            ($b.len(),)
        )
    };
}
macro_rules! op {
    ($func: ident, $bound: ident, $op:tt, $method:tt) => {
        fn $func<T: $bound<T, Output = T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);
            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y);
            }
        }
    };
}

op!(add_assign, Add, +=, add);
op!(mul_assign, Mul, *=, mul);
op!(sub_assign, Sub, -=, sub);

mod test {
    use std::iter;
    macro_rules! test {
        ($func: ident,$x:expr, $y:expr,$z:expr) => {
            #[test]
            fn $func() {
                for size in 0usize..10 {
                    let mut x: Vec<_> = iter::repeat($x).take(size).collect();
                    let y: Vec<_> = iter::repeat($y).take(size).collect();
                    let z: Vec<_> = iter::repeat($z).take(size).collect();
                    super::$func(&mut x, &y);
                    assert_eq!(x, z);
                }
            }
        };
    }
    // Test `add_assign`, `mul_assign`, and `sub_assign`.
    test!(add_assign, 1u32, 2u32, 3u32);
    test!(mul_assign, 2u32, 3u32, 6u32);
    test!(sub_assign, 3u32, 2u32, 1u32);
}

///
/// ## DSLs
///
/// A DSL is a mini "language" embedded in a Rust macro
///
/// ## Variadic Interfaces
///
/// A variadic interface takes an arbitrary number of arguments.
///
macro_rules! calculate {
    // The pattern for a single `eval`
    (eval $e: expr) => {{
        {
            let val:usize = $e;
            println!("{} = {}", stringify!($e), val);
        }
    }};
    // Decompose multiple `eval`s recursively
    (eval $e:expr, $(eval $es:expr),+) => {{
        calculate! { eval $e }
        calculate! { $(eval $es),+ }
    }};
}

#[test]
fn test_dsl() {
    calculate! {
        eval 1 + 2 // hehehe `eval` is _not_ a Rust keyword!
    }

    calculate! {
        eval (1 + 2) * (3 / 4)
    }

    calculate! { // Look ma! Variadic `calculate!`!
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
}
