///
/// # Functions
///
/// Functions are declared using the `fn` keyword. Its arguments are type annontated, just like variables,
/// and, the return type must be specified after an arrow `->`.
///
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0
}

#[test]
fn test_funcs() {
    assert_eq!(true, is_divisible_by(20, 5));
    assert_eq!(false, is_divisible_by(4, 3));
    assert_eq!(false, is_divisible_by(33, 0));
}

///
/// ## Methods
///
/// Methods are functions attached to objects. These methods have access to the data of object and
/// its other methods via a `self` keyword. Methods are defined under an `impl` block.
///
struct Point {
    x: f64,
    y: f64,
}
impl Point {
    // static method
    fn origion() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

struct Rect {
    p1: Point,
    p2: Point,
}
impl Rect {
    // instance method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the caller object.
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        ((x1 - x2) * (y1 - y2)).abs()
    }
}

struct Pair(Box<i32>, Box<i32>);
impl Pair {
    fn destroy(self) {
        let Pair(first, second) = self;
        println!("Destroying Pair({}, {})", first, second);
        //`first` and `second` go out of scope and get freed.
    }
}

#[test]
fn test_methods() {
    let rect = Rect {
        p1: Point::origion(),
        p2: Point::new(3.0, 4.0),
    };
    assert_eq!(12.0, rect.area());

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();
}

///
/// # Closures
///
/// Closures in Rust, also called lambda expressions, are functions that can capture the
/// enclosing environment. For example, a closure that captures the x variable.Box
///
/// ```
/// |val| val + x
/// ```
///
/// * using `||` instead of `()` around input variables
/// * optional boday delimination(`{}`) for single expression(mandatory otherwise).
/// * the ability to capture the out environment variables.
///
/// ## Capturing
///
/// Closures can capture variables:
///
/// * by reference: `&T`
/// * by mutable reference: `&mut T`
/// * by value: `T`
///
/// ## As input parameters
///
/// When taking a closure as an parameter, the closure's complete type
/// must be annotated using one of a few `traits`.
///
/// * `Fn`: the closure captures by reference(`&T`)
/// * `FnMut`: the closure captures by mutable reference(`&mut T`)
/// * `FnOnce`: the closure captures by value(`T`)
///
/// ## Type anonymity
///
/// ## Input functions
///
/// ## As output parameters
///

fn apply<T>(f: T)
where
    T: FnOnce(),
{
    f();
}

fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}
#[test]
fn test_closures() {
    fn func(i: i32) -> i32 {
        i + 1
    }
    let c1 = |i: i32| -> i32 { i + 1 };
    let c2 = |i| i + 1;

    let i = 1;
    assert_eq!(2, func(i));
    assert_eq!(2, c1(i));
    assert_eq!(2, c2(i));

    let double = |x| 2 * x;
    assert_eq!(6, apply_to_3(double));

    use std::mem;

    let greeting = "hello";
    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };
    // Call the function which applies the closure.
    apply(diary);

    fn create_fnonce() -> impl FnOnce() {
        let text = "FnOnce".to_owned();
        move || println!("This is a: {}", text)
    }
    let fn_once = create_fnonce();
    fn_once();
}
