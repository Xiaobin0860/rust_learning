mod my;
use my::nested;

pub fn public_function() {
    println!("called rary's `public_function()`");
    nested::function();
    my::function();
    my::call_public_in_my_mod();
    my::call_private();
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}
