#[test]
fn test_if() {
    let n = 5;
    let m = if n > 10 { 10 * n } else { n / 2 };
    assert_eq!(2, m);
}

#[test]
fn test_loop() {
    let mut count = 0;
    let n = loop {
        count += 1;
        if count == 4 {
            println!("four");
            continue;
        }
        if count == 5 {
            break count * 10;
        }
    };
    assert_eq!(5, count);
    assert_eq!(50, n);
}

#[test]
fn test_while() {
    let mut n = 1;
    while n < 101 {
        n += 1;
    }
    assert_eq!(101, n);
}

#[test]
fn test_for() {
    let mut n = 0;
    for _i in 1..101 {
        n += 1;
    }
    for _i in 1..=100 {
        n += 1;
    }
    assert_eq!(200, n);

    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Frank" => println!("Hello {}", name),
            _ => {}
        }
    }
}

///
/// # match
///
/// Rust provides pattern matching via the `match` keyword, which can be used like a C `switch`.assert_eq!
///

#[test]
fn test_match() {
    let n = 13;
    match n {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("ok"),
        _ => println!("no"),
    }
    let b = true;
    let b = match b {
        false => 0,
        true => 1,
    };
    assert_eq!(1, b);
}

///
/// ## Destructuring
///
/// A `match` block can destructure items in a variety of ways.
///

#[test]
fn test_match_destructure() {
    let pair = (0, -2);
    match pair {
        (0, y) => println!("x=0, y={}", y),
        (x, 0) => println!("x={}, y=0", x),
        _ => println!("{:?}", pair),
    }

    #[allow(dead_code)]
    enum Color {
        Red,
        Green,
        Blue,
        RGB(u32, u32, u32),
    }

    let color = Color::RGB(0, 255, 0);
    match color {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
        Color::RGB(r, g, b) => println!("rgb({}, {}, {})", r, g, b),
    }

    struct Foo {
        x: (u32, u32),
        y: i32,
    }
    let foo = Foo { x: (1, 2), y: 3 };
    match foo {
        Foo { x: (2, b), y } => println!("x.0=1, x.1={}, y={}", b, y),
        //the order is not important, and you can ignore some variables
        Foo { y: xxx, .. } => println!("y={}", xxx),
    }
}

///
/// ## Guards
///
/// A `match` guard can be added to filter the arm.
///
#[test]
fn test_match_guard() {
    let pair = (2, -2);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("x+y==0: x={}, y={}", x, y),
        (x, _) if x % 2 == 1 => println!("x is odd"),
        _ => println!("..."),
    }
}

///
/// ## Binding
///
/// `match` provides `@` sigil for binding values to names
///
fn some_number() -> Option<i32> {
    Some(42)
}
#[test]
fn test_match_binding() {
    match some_number() {
        Some(n @ 42) => println!("42 ok. n={}", n),
        Some(n) => println!("n={}", n),
        _ => (),
    }
}

///
/// ## if let, while let
///
#[test]
fn test_match_let() {
    let optinal = Some(7);
    if let Some(i) = optinal {
        println!("some {}", i);
    }

    let mut opt = Some(0);
    while let Some(i) = opt {
        if i > 9 {
            println!("Greater then 9, quit!");
            opt = None;
        } else {
            println!("i={}, Try again!", i);
            opt = Some(i + 1);
        }
    }
}
