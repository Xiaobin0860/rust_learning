pub mod nested;

fn private_function() {
    println!("called `my::mod::private_function()`");
}

pub fn function() {
    println!("called `my::mod::function()`");
}

pub fn call_private() {
    private_function();
    //nested::private_function();
}
pub fn call_public_in_my_mod() {
    print!("called `my::mod::call_public_function_in_my_mod()`, that\n> ");
    nested::public_function_in_my_mod();
    print!("> ");
    nested::public_function_in_super_mod();
}
