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
/// ### Early returns
///
/// ### Introducing `?`
///
/// Upon finding an `Err`, there are two valid actions to take:
///
///     1. `panic!` which we already decided to try to avoid if possible
///     2. `return` because an `Err` means it cannot be handled
///
/// `?` is almost exactly equivalent to an `unwrap` with `return`s instead of `panic`king on `Err`s
///
use std::num::ParseIntError;

fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1: i32 = n1_str.parse()?;
    let n2 = n2_str.parse::<i32>()?;
    Ok(n1 * n2)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

#[test]
fn test_result() -> Result<(), ParseIntError> {
    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number) => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);

    print(multiply("10", "2"));
    print(multiply("t", "2"));

    Ok(())
}

///
/// ## Multiple error types
///
/// Sometimes an `Option` needs to interact with a `Result`, or a `Result<T, Error1> need to interact
/// with an `Result<T, Error2>. In those cases, we want to manage our different error types in a way
/// that makes them composable and easy to interact with.
///
/// ### Pulling `Result`s out of `Option`s
///
/// The most basic way of handing mixed error types is to just embed them in each other.
///
/// ### Defining an error type
///
/// Sometimes it simplifies the code to mask all of the different errors with a single type of error.
/// Rust allows us to define our own error types. In general, a "good" error type:
///
/// * Represents different errors with the same type
/// * Presents nice error messages to the user
/// * Is easy to compare with other types
///     - Good: `Err(EmptyVec)`
///     - Bad: `Err("Use a vector with at least one element".to_owned())`
/// * Can hold infomation about the error
///     - Good: `Err(BadChar(c, position))`
///     - Bad: `Err("+ cannot be used here".to_owned())`
/// * Composes well with other errors
///
/// ### `Box`ing errors
///
/// A way to write simple code while preserving the original errors is to `Box` them.
/// The drawback is that the underlying error type is only known at runtime and not `statically determined`.
///
/// The stdlib helps in boxing our errors by having `Box` implement conversion from any type that implements
/// the `Error` trait into the trait object `Box<Error>`, via `From`.
///
/// ### Other uses of `?`
///
/// `?` was previously explained as either `unwrap` or return `return Err(err)`. This is only mostly true. It
/// actually means `unwrap` or `return Err(From::from(err))`. Since `From:from` is conversion utility between
/// different types, this means that if you `?` where the error is converrible to the return type, it will
/// convert automatically.
///
/// ### Wrapping errors
///
/// An alternative to boxing errors is to wrap them in your own error type.
///
#[derive(Debug)]
enum DError {
    EmptyVec,
    // We will defer to the parse error implementation for their error.
    // Supplying extra info requires adding more data to the type.
    Parse(ParseIntError),
}

impl fmt::Display for DError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            DError::EmptyVec => write!(f, "please use a vector with at least one element"),
            // This is a wrapper, so defer to the underlying type' implementation of `fmt`.
            DError::Parse(e) => e.fmt(f),
        }
    }
}

impl error::Error for DError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DError::EmptyVec => None,
            DError::Parse(ref e) => Some(e),
        }
    }
}

// Implement the conversion from `ParseIntError` to `DError`
// This will be automatically called by `?` if a `ParseIntError` need to be converted into a `DError`
impl From<ParseIntError> for DError {
    fn from(err: ParseIntError) -> Self {
        DError::Parse(err)
    }
}

//Vec::first returns an `Option`, while `parse::<i32>` returns a `Result<i32, ParseIntError>
fn double_first(vec: &Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n))
}
fn double_first2(vec: &Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));
    opt.map_or(Ok(None), |r| r.map(Some))
}

use std::fmt;
#[derive(Debug, Clone)]
struct DoubleError;

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

fn double_first3(vec: &Vec<&str>) -> Result<i32, DoubleError> {
    vec.first()
        .ok_or(DoubleError)
        .and_then(|s| s.parse::<i32>().map_err(|_| DoubleError).map(|i| 2 * i))
}

use std::error;
#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}
impl error::Error for EmptyVec {}

fn double_first4(vec: &Vec<&str>) -> Result<i32, Box<dyn error::Error>> {
    vec.first()
        .ok_or(EmptyVec.into())
        .and_then(|s| s.parse::<i32>().map_err(|e| e.into()).map(|i| 2 * i))
}

fn double_first5(vec: &Vec<&str>) -> Result<i32, Box<dyn error::Error>> {
    Ok(vec.first().ok_or(EmptyVec)?.parse::<i32>()? * 2)
}

fn double_first6(vec: &Vec<&str>) -> Result<i32, DError> {
    Ok(vec.first().ok_or(DError::EmptyVec)?.parse::<i32>()? * 2)
}

#[test]
fn test_multiple_error_types() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {:?}", double_first(&numbers));
    println!("The first doubled is {:?}", double_first(&empty));
    println!("The first doubled is {:?}", double_first(&strings));

    println!("2 The first doubled is {:?}", double_first2(&numbers));
    println!("2 The first doubled is {:?}", double_first2(&empty));
    println!("2 The first doubled is {:?}", double_first2(&strings));

    println!("3 The first doubled is {:?}", double_first3(&numbers));
    println!("3 The first doubled is {:?}", double_first3(&empty));
    println!("3 The first doubled is {:?}", double_first3(&strings));

    println!("4 The first doubled is {:?}", double_first4(&numbers));
    println!("4 The first doubled is {:?}", double_first4(&empty));
    println!("4 The first doubled is {:?}", double_first4(&strings));

    println!("5 The first doubled is {:?}", double_first5(&numbers));
    println!("5 The first doubled is {:?}", double_first5(&empty));
    println!("5 The first doubled is {:?}", double_first5(&strings));

    println!("6 The first doubled is {:?}", double_first6(&numbers));
    println!("6 The first doubled is {:?}", double_first6(&empty));
    println!("6 The first doubled is {:?}", double_first6(&strings));
}

/// ## Iterating over Results
///
/// An `Iter::map` operation might fail, let's step through strategies for handing this.
///

#[test]
fn test_iterating_over_results() {
    // ignore the failed items with `filter_map()`
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<i32> = strings
        .into_iter()
        .map(|s| s.parse())
        .filter_map(Result::ok)
        .collect();
    println!("Results: {:?}", numbers);

    //Fail the entire operation with `collect()`
    let strings = vec!["tofu", "93", "18"];
    let numbers: Result<Vec<i32>, _> = strings.into_iter().map(|s| s.parse()).collect();
    println!("Results: {:?}", numbers);

    //Collect all valid values and failures with partition()
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);

    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}
