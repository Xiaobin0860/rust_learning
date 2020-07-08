///
/// ## Structures
///
/// * Tuple structs, which are, basically,named tuples
/// * The classic `C structs`
/// * Unit structs, which are field-less, are useful for generics
///

#[derive(Debug)]
struct Person<'a> {
    // The 'a defines a lifetime
    name: &'a str,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rect {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(r: &Rect) -> f32 {
    let Point { x: x1, y: y1 } = r.top_left;
    let Point { x: x2, y: y2 } = r.bottom_right;
    ((x1 - x2) * (y1 - y2)).abs()
}

#[test]
fn test_structs() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter);

    let p1 = Point { x: 10.3, y: 0.4 };
    let x = 1.2;
    let y = 3.2;
    let p2 = Point { x, y };

    let bottom_right = Point { x: 5.2, ..p1 };
    let top_left = Point { y: 10.5, ..p2 };

    let Point { x: left, y: top } = top_left;
    let Point {
        x: right,
        y: bottom,
    } = bottom_right;

    let rect = Rect {
        top_left,
        bottom_right,
    };
    println!("rect={:?}, area={}", rect, rect_area(&rect));

    assert_eq!(top, rect.top_left.y);
    assert_eq!(left, rect.top_left.x);
    assert_eq!(right, rect.bottom_right.x);
    assert_eq!(bottom, rect.bottom_right.y);

    let _unit = Unit;

    let pair = Pair(1, 0.1);

    assert_eq!(1, pair.0);
    assert_eq!(0.1, pair.1);
}

///
/// Enums
///
/// Thie `enum` keyword allows the creation of a type
/// which may be one of few different variants
///
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: f64, y: f64 },
}

use std::fmt;
use WebEvent::*;

impl fmt::Display for WebEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PageLoad => write!(f, "page loaded"),
            WebEvent::PageUnload => write!(f, "page unloaded"),
            KeyPress(c) => write!(f, "pressed '{}'", c),
            Click { x, y } => write!(f, "clicked at x={}, y={}", x, y),
            Paste(s) => write!(f, "pasted \"{}\"", s),
        }
    }
}

enum Number {
    Zero,
    One,
    Two,
}

#[allow(dead_code)]
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

#[test]
fn test_enums() {
    let pressed = KeyPress('x');
    let pasted = Paste("my text".to_owned());
    let click = Click { x: 10.123, y: 12.2 };
    let load = WebEvent::PageLoad;
    let unload = PageUnload;

    println!("{}", pressed);
    println!("{}", pasted);
    println!("{}", click);
    println!("{}", load);
    println!("{}", unload);

    use Number::{One, Zero};

    assert_eq!(0, Zero as i32);
    assert_eq!(1, One as i32);
    assert_eq!(2, Number::Two as i32);
    assert_eq!(0xff0000, Color::Red as i32);
}
