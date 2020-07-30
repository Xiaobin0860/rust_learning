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
            println!("begin this is thread number {}", i);
            if i == 0 {
                // panic current thread
                panic!("panic {}", i);
            }
            println!("end this is thread number {}", i);
        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }

    println!("ok");

    //map-reduce
    // This is our data to process.
    // We will calculate the sum of all digits via a threaded  map-reduce algorithm.
    // Each whitespace separated chunk will be handled in a different thread.
    //
    // TODO: see what happens to the output if you insert spaces!
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";

    // Make a vector to hold the child-threads which we will spawn.
    let mut children = vec![];

    /*************************************************************************
     * "Map" phase
     *
     * Divide our data into segments, and apply initial processing
     ************************************************************************/

    // split our data into segments for individual calculation
    // each chunk will be a reference (&str) into the actual data
    let chunked_data = data.split_whitespace();

    // Iterate over the data segments.
    // .enumerate() adds the current loop index to whatever is iterated
    // the resulting tuple "(index, element)" is then immediately
    // "destructured" into two variables, "i" and "data_segment" with a
    // "destructuring assignment"
    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", i, data_segment);

        // Process each data segment in a separate thread
        //
        // spawn() returns a handle to the new thread,
        // which we MUST keep to access the returned value
        //
        // 'move || -> u32' is syntax for a closure that:
        // * takes no arguments ('||')
        // * takes ownership of its captured variables ('move') and
        // * returns an unsigned 32-bit integer ('-> u32')
        //
        // Rust is smart enough to infer the '-> u32' from
        // the closure itself so we could have left that out.
        children.push(thread::spawn(move || -> u32 {
            // Calculate the intermediate sum of this segment:
            let result = data_segment
                // iterate over the characters of our segment..
                .chars()
                // .. convert text-characters to their number value..
                .map(|c| c.to_digit(10).expect("should be a digit"))
                // .. and sum the resulting iterator of numbers
                .sum();

            // println! locks stdout, so no text-interleaving occurs
            println!("processed segment {}, result={}", i, result);

            // "return" not needed, because Rust is an "expression language", the
            // last evaluated expression in each block is automatically its value.
            result
        }));
    }

    /*************************************************************************
     * "Reduce" phase
     *
     * Collect our intermediate results, and combine them into a final result
     ************************************************************************/

    // collect each thread's intermediate results into a new Vec
    let mut intermediate_sums = vec![];
    for child in children {
        // collect each child thread's return-value
        let intermediate_sum = child.join().unwrap();
        intermediate_sums.push(intermediate_sum);
    }

    // combine all intermediate sums into a single final sum.
    //
    // we use the "turbofish" ::<> to provide sum() with a type hint.
    //
    // TODO: try without the turbofish, by instead explicitly
    // specifying the type of final_result
    let final_result = intermediate_sums.iter().sum::<u32>();

    println!("Final sum result: {}", final_result);
}

use std::sync::mpsc;
///
/// ## Channels
///
/// Rust provides asynchronous `channels` for communication between threads. Channels allow a
/// unidirectional flow of infomation between two end-points: the `Sender` and the `Receiver`.
///
use std::sync::mpsc::{Receiver, Sender};
#[test]
fn test_channels() -> Result<(), mpsc::RecvError> {
    const N: i32 = 3;
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let mut children = Vec::new();

    for id in 0..N {
        // The sender endpoint can be copied
        let thread_tx = tx.clone();

        // Each thread will send its id via the channel
        let child = thread::spawn(move || {
            thread_tx.send(id).unwrap();
            println!("thread {} finished", id);
        });

        children.push(child);
    }
    // Here, all the messages are collected
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..N {
        // The `recv` method picks a message from the channel
        // `recv` will block the current thread if there are no messages available
        ids.push(rx.recv()?);
    }

    // Wait for the threads to complete any remaining work
    for child in children {
        child.join().expect("oops! the child thread panicked");
    }

    // Show the order in which the messages were sent
    println!("{:?}", ids);

    Ok(())
}

///
/// ## Path
///
/// the `Path` struct represents file paths in the underlying filesystem. There are two flavors of `Path`:
/// `posix::Path`, for UNIX-like systems, and `windows::Path`, for Windows. The prelude exports the appropriate
/// platform-specific `Path` variant.
///
/// A `Path` can be created from an `OsStr`, and provides several methods to get information from the
/// file/directory the path points to.
///
/// Note that a `Path` is not internally represented as an UTF-8 string, but instead is stored as a vector of
/// bypes(`Vec<u8>`). Therefore, converting a `Path` to a `&str` is not free and may fail(an `Option` is returned).
///
use std::path::Path;

#[test]
fn test_path() {
    let path = Path::new(".");
    println!("{}", path.display());

    for f in path.read_dir().expect("read_dir failed") {
        if let Ok(f) = f {
            println!("{:?}", f.path());
        }
    }

    let new_path = path.join("a").join("b");

    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}

///
/// ## File I/O
///
/// The `File` struct represents a file that has been opened(it wraps a file descriptor), and gives read
/// and/or write access to the underlying file.
///
/// Since many things can go wrong when doing file I/O, all the `File` methods return the `io::Result<T>` type,
/// which is an alias for `Result<T, io::Error>`.
///
/// This makes the failure of all I/O operations explicit. Thanks to this, the programmer can see all the
/// failure paths, and is encouraged to handle them in a proactive manner.
///
/// * `open`
/// The `open` static method can be used to open a file in read-only mode
///
/// * `create`
/// The `create` static method opens a file in write-only mode. If the file already existed, the old content
/// is destroyed. Otherwise, a new file is created.
///
/// * `read_lines`
/// The method `lines()` returns an iterator over the lines of a file.
///
use std::fs::File;
use std::io::{self, BufRead, Write};
static TXT: &str = "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";
#[test]
fn test_file() {
    {
        let mut f = File::create("./test.txt").unwrap();
        f.write_all(TXT.as_bytes()).unwrap();
    }

    if let Ok(lines) = read_lines("./test.txt") {
        for line in lines {
            if let Ok(line) = line {
                println!("{}", line);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
