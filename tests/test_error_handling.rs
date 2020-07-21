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

///
/// ### Unpacking options with `?`
///
/// You can unpack `Option`s by using `match` statements, but it's often easier to use the `?` operator.
/// If `x` is an `Option`, then evaluating `x?` will return the underlying value if `x` is `Some`, otherwise
/// it will terminate whatever function is being executed and return `None`.
///
struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    //Gets the area code of the phone number of the person's job, if it exists.
    fn work_phone_area_code(&self) -> Option<u8> {
        self.job?.phone_number?.area_code
    }

    fn work_phone_number(&self) -> Option<u32> {
        Some(self.job?.phone_number?.number)
    }
}

#[test]
fn test_unpacking() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 43922222,
            }),
        }),
    };

    assert_eq!(p.work_phone_area_code(), Some(61));
    assert_eq!(p.work_phone_number(), Some(43922222));

    let p = Person { job: None };
    assert_eq!(p.work_phone_area_code(), None);
    assert_eq!(p.work_phone_number(), None);

    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: None,
                number: 43922222,
            }),
        }),
    };
    assert_eq!(p.work_phone_area_code(), None);
    assert_eq!(p.work_phone_number(), Some(43922222));
}

///
/// ### Combinators: `map`
///
/// `Option` has a built in method called `map()`, a combinator for the simple mapping of `Some -> Some`
/// and `None -> None`. Multiple `map()` replaces all functions previous to it while staying compact.
///
/// ### Combinators: `and_then`
///
/// `and_then()` calls its function input with the wrapped value and returns the result. If the `Option` is
/// `None`, then it returns `None` instead.
///

#[derive(Debug, PartialEq)]
enum Food {
    Apple,
    Carrot,
    Potato,
    CordonBleu,
    Steak,
    Sushi,
}
#[derive(Debug)]
enum Day {
    Monday,
    Tuesday,
    Wendnesday,
}
#[derive(Debug)]
struct Peeled(Food);
#[derive(Debug)]
struct Chopped(Food);
#[derive(Debug)]
struct Cooked(Food);

fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}

fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    peeled.map(|Peeled(f)| Chopped(f))
}

fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(f)| Cooked(f))
}

fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("Mmm. I love {:?}", food),
        None => println!("Oh no! It wasn't edible."),
    }
}

fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _ => Some(food),
    }
}

fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBleu => None,
        _ => Some(food),
    }
}

fn cookable_v1(food: Food) -> Option<Food> {
    match have_recipe(food) {
        None => None,
        Some(food) => match have_ingredients(food) {
            None => None,
            Some(food) => Some(food),
        },
    }
}

fn cookable_v2(food: Food) -> Option<Food> {
    have_recipe(food).and_then(have_ingredients)
}

fn eat2(food: Food, day: Day) {
    match cookable_v2(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
        None => println!("Oh no. We don't get to eat on {:?}?", day),
    }
}

#[test]
fn test_combinators() {
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));
    // Let's try the simpler looking `process()` now.
    let cooked_potato = process(potato);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);
    let potato = Some(Food::Potato);
    let cooked_potato = process(potato);
    eat(cooked_potato);

    let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);
    eat2(cordon_bleu, Day::Monday);
    eat2(steak, Day::Tuesday);
    eat2(sushi, Day::Wendnesday);

    assert_eq!(cookable_v1(Food::Steak), Some(Food::Steak));
    assert_eq!(cookable_v1(Food::CordonBleu), None);
}

///
/// ## `Result`
///
/// `Result` si a richer version of the `Option` type that describes possible error instead of possible absence.
///
/// That is, `Result<T, E>` could have one of two outcomes:
///
/// * `Ok(T)`: An element `T` was found
/// * `Err(E)`: An error was found with element `E`
///
/// ### `map` for `Result`
///
/// ### aliases for `Result`
///
use std::num::ParseIntError;
#[test]
fn test_result() -> Result<(), ParseIntError> {
    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number) => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}
