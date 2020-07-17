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
