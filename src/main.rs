extern crate rary;

fn main() {
    println!("Hello, world!");
    rary::public_function();
    rary::indirect_access();
}

#[cfg(test)]
mod comments;
