///
/// # Error handling
///
/// Error handling is the process of handling the possibility of failure.
///
/// ## `panic`
///
/// The simplest error handling is `panic`. It prints an error message, starts
/// unwinding the stack, and usually exits the program
///
/// ## `Option` & `unwrap`
///
/// An `enum` called `Option<T>` in the `std` library is used when absence is a possibility.
/// It manifests itself as one of two "options".
///
/// * `Some(T)`: An element ot type `T` was found
/// * `None`: No element was found
///

fn give_commoner(gift: Option<&str>) {
    // Specify a course of action for each case.
    match gift {
        Some("snake") => println!("Yuck! I'm putting this snake back in the forest."),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No gift? Oh well."),
    }
}

// Our sheltered princess will `panic` at the sight of snakes.
// All gifts are handled implicitly using `unwrap`.
fn give_princess(gift: Option<&str>) {
    // `unwrap` returns a `panic` when it receives a `None`.
    let inside = gift.unwrap();
    if inside == "snake" {
        panic!("AAAAaaaaa!!!!!");
    }

    println!("I love {}s!!!!!", inside);
}

#[test]
//#[should_panic(expected = "AAAAaaaaa!!!!!")]
#[should_panic]
fn test_error_handling() {
    let food = Some("cabbage");
    let snake = Some("snake");
    let void = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    let nothing = None;

    give_princess(bird);
    give_princess(nothing);
}
