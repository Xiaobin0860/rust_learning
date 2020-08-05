extern crate libc;
extern crate rary;

use std::slice;

fn main() {
    println!("Hello, world!");
    rary::public_function();
    rary::indirect_access();

    let v = vec![1, 2, 3, 4];
    let pointer = v.as_ptr();
    let length = v.len();
    unsafe {
        let my_slice: &[u32] = slice::from_raw_parts(pointer, length);
        assert_eq!(v.as_slice(), my_slice);

        assert_eq!(libc::abs(-5), 5);
    }
}

#[cfg(test)]
mod comments;
