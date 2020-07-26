///
/// # Std library types
///
/// The `std` library provides many custom types which expands drastically on the `primitives`.
/// Some of these include:
///
/// * growable `String`s like: "hello world"
/// * growable vectors: `[1, 2, 3]`
/// * optional types: `Option<i32>`
/// * error handling types: `Result<i32, i32>
/// * heap allocated pointers: `Box<i32>`
///
/// ## Box, stack and heap
///
/// All values in Rust are stack allocated by default. Values can be boxed(allocked on the heap) by
/// creating a `Box<T>`. A box is a smart pointer to a heap allocated value of the `T`. When a box goes
/// out of scope, its destructor is called, the inner object is destroyed, and the memory on the heap is freed.
///
/// Boxed values can be dereferenced using the `*` operator; this removes one layer of indirection.
///
/// ## `Rc`
///
/// When multiple ownership is needed, `Rc`(Reference Counting) can be used. 'Rc` keeps
/// track of the bumber of the references which means the number of owners of the value
/// wrapped inside an `Rc`.
///
/// Reference count of an `Rc` increases by 1 whenever an `Rc` is cloned, and decreases by
/// 1 whenever one cloned 'Rc` is dropped ot of the scope. When an `Rc`'s reference count
/// becomes zero, which means there are no owners remained, both the `Rc` and the value are all dropped.
///
/// Cloning an `Rc` never performs a deep copy. Cloning creates just pointer to the
/// wrapped value, and increments the count.
use std::mem;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    Box::new(Point { x: 0.0, y: 0.0 })
}

#[test]
fn test_box() {
    let point = origin();
    let rect = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    };
    let boxed_rect = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });
    let boxed_point = Box::new(origin());
    let box_in_a_box = Box::new(boxed_origin());
    println!(
        "Point occupies {} bytes on the stack",
        mem::size_of_val(&point)
    );
    println!(
        "Rectangle occupies {} bytes on the stack",
        mem::size_of_val(&rect)
    );
    println!(
        "Boxed point occupies {} bytes on the stack",
        mem::size_of_val(&boxed_rect)
    );
    println!(
        "Boxed box occupies {} bytes on the stack",
        mem::size_of_val(&box_in_a_box)
    );

    // Copy the data contained in `boxed_point` into `unboxed_point`
    let unboxed_point = *boxed_point;
    println!(
        "Unboxed point occupies {} bytes on the stack",
        mem::size_of_val(&unboxed_point)
    );
}

///
/// ## Vectors
///
/// Vectors are re-sizable arrays. Like slices, their size is not known at compile time, but they can
/// grow or shrink at any time. A vector is represented using 3 parameters:
///
/// * pointer to the data
/// * length
/// * capacity
///
#[test]
fn test_vecs() {
    // Iterators can be collected into vectors
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    // The `vec!` macro can be used to initialize a vector
    let mut xs = vec![1, 2, 3];
    println!("Initial vector: {:?}", xs);
    xs.push(4);
    println!("Vector: {:?}", xs);
    println!("Pop last element: {:?}", xs.pop());
    // `Vector`s can be easily iterated over
    println!("Contents of xs:");
    for x in xs.iter() {
        println!("> {}", x);
    }
    // A `Vector` can also be iterated over while the iteration
    // count is enumerated in a separate variable (`i`)
    for (i, x) in xs.iter().enumerate() {
        println!("In position {} we have value {}", i, x);
    }

    // Thanks to `iter_mut`, mutable `Vector`s can also be iterated
    // over in a way that allows modifying each value
    for x in xs.iter_mut() {
        *x *= 3;
    }
    println!("Updated vector: {:?}", xs);
}

///
/// ## Strings
///
/// There are two types of strings in Rust: `String` and `&str`.
///
/// A `String` is stored as a vector of bytes(`Vec<u8>`), but guaranteed to
/// always be a valid UTF-8 sequence. `String` is heap allocated, growable
/// and not null terminated.
///
/// `&str` is a slice(`&[u8]`) that always points to a valid UTF-8 sequence,
/// and can be used to view into a `String`, just like `&[T]` is a view into `Vec<T>`.
///
#[test]
fn test_strings() {
    // (all the type annotations are superfluous)
    // A reference to a string allocated in read only memory
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);
    // Iterate over words in reverse, no new string is allocated
    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }
    // Copy chars into a vector, sort and remove duplicates
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();
    // Create an empty and growable `String`
    let mut string = String::new();
    for c in chars {
        // Insert a char at the end of string
        string.push(c);
        // Insert a string at the end of string
        string.push_str(", ");
    }
    // The trimmed string is a slice to the original string,
    // hence no new allocation is performaed
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);
    // Heap allocate a string
    let alice = String::from("I likes dogs");
    // Allocate new memory and store the modified string there
    let bob = alice.replace("dog", "cat");
    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);
}

///
/// ## `Option`
///
/// Sometimes it's desirable to catch the failure of some parts of
/// a program instead of calling `panic!`; this can be accomplished
/// using the `Option` enum.
///
/// The `Option<T>` enum has two variants:
///
/// * `None`, to indicate failure or lack of value
/// * `Some(value), a tuple struct that wraps a `value` with type `T`.
///
/// ## `Result`
///
/// We've seen that the `Option` enum can be used as a return value from
/// functions that may fail, where `None` can be returned to indicate failure.
/// However, sometimes it's important to express why an operation failed.
/// To do this we have the `Result` enum.
///
/// The `Result<T, E> enum has two variants:
/// * `Ok(value)` which indicates that the operation succeeded, and wraps
/// the `value` returned by the operation(`value` has type `T`)
/// * `Err(why)` which indicates that the operation failed, and wraps `why`,
/// which(hopefully) explains the cause of the failure.(`why` has type `E`)
///
/// ## `panic!`
///
/// The `panic!` macro can be used to generater a panic and start unwinding its stack.
/// While unwinding, the runtime will take care of freeing all the resources owned by
/// the thread by calling the destructor of all its objects.
///
/// Since we are dealing with programs with only one thread, `panic!` will cause the
/// program to report the panic message and exit.
///
/// ## HashMap
///
/// Where vectors store values by an iterger index, `HashMap`s store values by key.
/// `HashMap` keys can be booleans, integers, strings, or any other type that implements
/// the `Eq` and `Hash` traits. More on this in the next section.
///
/// Like vectors, `HashMap`s are growable, but `HashMap`s can also shrink themselves
/// when they have excess space. You can create a HashMap with a certain starting
/// capacity using `HashMap::with_capacity(uint)`, or use `HashMap::new()` to get a
/// HashMap with a default initial capacity(recommended).
///
use std::collections::HashMap;

fn call(number: &str) -> &str {
    match number {
        "798-1364" => {
            "We're sorry, the call cannot be completed as dialed. 
            Please hang up and try again."
        }
        "645-7689" => {
            "Hello, this is Mr. Awesome's Pizza. My name is Fred.
            What can I get for you today?"
        }
        _ => "Hi! Who is this again?",
    }
}

#[test]
fn test_maps() {
    let mut contacts = HashMap::new();
    contacts.insert("Daniel", "897-1234");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");
    // Takes a reference and returns Option<&V>
    match contacts.get(&"Daniel") {
        Some(&number) => println!("Calling Daniel: {}", call(number)),
        _ => println!("Don't have Daniel's number."),
    }

    // `HashMap::insert()` returns `None`
    // if the inserted value is new, `Some(value)` otherwise
    contacts.insert("Daniel", "164-6743");

    match contacts.get(&"Ashley") {
        Some(&number) => println!("Calling Ashley: {}", call(number)),
        _ => println!("Don't have Ashley's number."),
    }

    contacts.remove(&"Ashley");

    // `HashMap::iter()` returns an iterator that yields
    // (&'a key, &'a value) pairs in arbitrary order.
    for (contact, &number) in contacts.iter() {
        println!("Calling {}: {}", contact, call(number));
    }
}

///
/// ### Alternate/custom key types
///
/// Any type that implements the `Eq` and `Hash` traits can be a key in `HashMap`:
///
/// * `bool`(thouth not very useful since there is only two possible keys)
/// * `int`, `uint`, and all variations thereof
/// * `String` and `&str`(protip: you can have a `HashMap` keyed by `String`
/// and call `.get()` with an `&str`)
///
/// All collection classes implement `Eq` and `Hash` if their contained type also
/// respectively implements `Eq` and `Hash`. For example, `Vec<T>` will implement
/// `Hash` if `T` implements `Hash`.
///
/// You can easily implement `Eq` and `Hash` for a custom type with just one line:
/// `#[derive(PartialEq, Eq, Hash)]`
/// The compiler will do the rest. If you want more control over the details, you
/// can implement `Eq` and/or `Hash` yourself.
///
/// ### HashSet
///
/// Consider a `HashSet` as a `HashMap` where we just care about the keys(`HashSet<T>` is,
/// in actuality, just a wrapper around `HashMap<T, ()>)
///
use std::hash::{Hash, Hasher};
// Eq requires that you derive PartialEq on the type
#[derive(Eq, Debug)]
struct Account<'a> {
    username: &'a str,
    password: &'a str,
}

impl PartialEq for Account<'_> {
    fn eq(&self, other: &Self) -> bool {
        let my_account = self.username.to_lowercase();
        let other_account = other.username.to_lowercase();
        my_account == other_account && self.password == other.password
    }
}

impl Hash for Account<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let account = self.username.to_lowercase();
        account.hash(state);
        self.password.hash(state);
    }
}

struct AccountInfo<'a> {
    name: &'a str,
    email: &'a str,
}

type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn try_logon<'a>(accounts: &Accounts<'a>, username: &'a str, password: &'a str) -> bool {
    println!("Username: {}", username);
    println!("Password: {}", password);
    println!("Attempting logon ...");

    let logon = Account { username, password };
    println!("try_logon {:?}", logon);

    match accounts.get(&logon) {
        Some(account_info) => {
            println!("Successful logon!");
            println!("Name: {}", account_info.name);
            println!("Email: {}", account_info.email);
            true
        }
        _ => {
            println!("Login failed!");
            false
        }
    }
}

#[test]
fn test_custom_key_types() {
    let mut accounts: Accounts = HashMap::new();

    let account = Account {
        username: "j.everyman",
        password: "password123",
    };

    let account_info = AccountInfo {
        name: "John Everyman",
        email: "j.everyman@email.com",
    };

    accounts.insert(account, account_info);

    assert_eq!(true, try_logon(&accounts, "j.everyman", "password123"));
    assert_eq!(false, try_logon(&accounts, "j.everyman", "Password123"));
    assert_eq!(true, try_logon(&accounts, "j.Everyman", "password123"));
}

/// ## Threads

/// Rust provides a mechanism for spawning native OS threads via the spawn function, 
/// the argument of this function is a moving closure.
use std::thread;

static NTHREADS: i32 = 10;

#[test]
fn test_threads() {
    // Make a vector to hold the children which are spawned.
    let mut children = vec![];

    for i in 0..NTHREADS {
        // Spin up another thread
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i);
        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}
