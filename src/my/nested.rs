pub fn function() {
    println!("called `my::nested::function()`");
}
#[allow(dead_code)]
fn private_function() {
    println!("called `my::nested::private_function()`");
}
// Functions declared using `pub(in path)` syntax are only visible
// within the given path. `path` must be a parent or ancestor module
pub(in crate::my) fn public_function_in_my_mod() {
    print!("called `my::nested::public_function_in_my_mod()`, that\n> ");
    public_function_in_nested();
}
// Functions declared using `pub(self)` syntax are only visible within
// the current module, which is the same as leaving them private
pub(self) fn public_function_in_nested() {
    println!("called `my::nested::public_function_in_nested()`");
}
// Functions declared using `pub(super)` syntax are only visible within
// the parent module
pub(super) fn public_function_in_super_mod() {
    println!("called `my::nested::public_function_in_super_mod()`");
}
