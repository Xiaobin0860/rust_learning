///
/// ## Scalar Types
///
/// * signed integers: i8, i16, i32, i64, i128, isize
/// * unsigned integers: u8, u16, u32, u64, u128, usize
/// * floating point: f32, f64
/// * char: Unicode scalar values 'a'
/// * bool: true, false
/// * the unit type (), only possible value is an empty tuple: ()
///
/// ## Compound Types
///
/// * arrays like [1, 2, 3]
/// * tuples like (1, true)
///

#[derive(Debug)]
struct M(f32, f32, f32, f32);

#[test]
#[allow(unused_assignments)]
fn test_primitives() {
    let logical: bool = false;
    assert!(!logical);
    let mut a: f64 = 1.0;
    a = 3.14;
    println!("{}", a);
    let a = 4;
    println!("{}", a);

    println!("1+2={}", 1 + 2);
    println!("1-2={}", 1 - 2);
    println!("true AND false is {}", true && false);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}

#[test]
fn test_tuples() {
    //tuples can be destructured to create bindings
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{}, {}, {}, {}", a, b, c, d);

    let m = M(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", m);
}

use std::mem;

#[test]
fn test_arrays_and_slices() {
    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` returns the size of the array
    assert_eq!(5, xs.len());
    assert_eq!(500, ys.len());

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    // They are of the form [starting_index..ending_index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);

    let s = &xs[1..4];
    assert_eq!(2, s[0]);
    assert_eq!(4, s[2]);

    // Out of bound indexing causes compile error
    //println!("{}", xs[5]);
}

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}
