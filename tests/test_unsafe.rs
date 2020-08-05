///
/// # Unsafe Operations
///
/// Unsafe annotations in Rust are used to bypass protections put in place by the compiler;
/// specifically, there are four primary things that unsafe is used for:
///
/// * dereferencing raw pointers
/// * calling functions or methods which are `unsafe`(including calling a function over FFI)
/// * accessing or modifying static mutable variables
/// * implementing unsafe traits
///
/// ## Raw Pointers
///
/// Raw pointers `*` and references `&T` function similarly, but references are always safe
/// because they are guaranteed to point to valid data due to the borrow checker. Dereferencing
/// a raw pointer can only be done through an unsafe block.
///
/// ## Calling Unsafe Functions
///
/// Some functions can be declared as `unsafe`, meaning it is the programmer's responsibility to
/// ensure correctness instead of the compiler's.
///
/// ## Foreign Function Interface
///
/// Rust provides a Foreign Function Interface(FFI) to C libraries. Foreign functions must be
/// declared inside an `extern` block annotated with a `#[link]` attribute containing the name
/// of the foreign library.
///
use std::slice;
#[test]
fn test_unsafe() {
    let raw_p: *const u32 = &10;
    unsafe {
        assert_eq!(10, *raw_p);
    }

    let v = vec![1, 2, 3, 4];
    let pointer = v.as_ptr();
    let length = v.len();
    unsafe {
        let my_slice: &[u32] = slice::from_raw_parts(pointer, length);
        assert_eq!(v.as_slice(), my_slice);
    }
}
