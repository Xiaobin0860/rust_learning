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
