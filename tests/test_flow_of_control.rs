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
