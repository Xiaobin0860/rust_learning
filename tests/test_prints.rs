///
/// ## Printing is handled by a series of `macros` defined in `std::fmt`
///
/// * `format!`: write formatted text to `String`
/// * `print!`: same as `format!` but the text is printed to the console(io::stdout)
/// * `println!`: same as `print!` but a newline is appended
/// * `eprint!`: same as `format!` but the text is printed to the standand error(io::stderr)
/// * `eprintln!`: same as `eprint!` but a new line is appended
///
/// ## `std::fmt` contains many `traits` which govern the display of text.
///
/// * `fmt::Debug`: Uses `{:?}` marker. Format text for debugging purposes.
/// * `fmt::Display`: Uses `{}` marker. Automatically implements the `ToString` trait
/// which allows us convert the type to `String`.
///
use std::fmt;

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}, {}}}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
struct Line2D {
    p1: Point,
    p2: Point,
}

#[test]
fn test_prints() {
    println!("{} days", 31);
    println!("{1}, this is {0}, {0}, this is {1}", "Alice", "Bob");
    // special formatting can be specified after a `:`
    println!("{number:0width$}", number = 1, width = 4); //0001
    println!("{:>w$}", 1, w = 4); //   1
    eprintln!("[error] e");
    println!("0x{:02X}", 13);
    println!("0b{:b}", 10);

    assert!(true);

    let p1 = Point { x: 0, y: 0 };
    let p2 = Point { x: 1, y: 1 };
    println!("p1={}, p2={}", p1, p2);

    let line = Line2D { p1: p1, p2: p2 };
    println!("line={:?}", line);
}
