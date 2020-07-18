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
