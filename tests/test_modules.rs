///
/// # Modules
///
/// A module is a collection of items: functions, structs, traits, `impl` blocks, and even other modules.
///
/// ## Visibility
///
/// By default, the items in a module have private visibility, but this can be overridden with the `pub` modifier.
/// Only the public items of a module can be accessed from outside the module scope.
///
/// ## Struct visibility
///
/// Struct have an extra level of visibility with thair fields. The visibility defaults to private, and can be
/// overridden with the `pub` modifier. This visibility only matters when a struct is accessed from outside
/// the module where is defined, and has the goal of hiding information.
///
/// ## The `use` declaration
///
/// The `use` declaration can be used to bind a full path to a new name, for easier access.
///
/// ## `super` and `self`
///
/// The `super` and `self` keywords can be used in the path to remove ambiguity when accessing items
/// and to prevent unnecessary hardcoding of paths.
///
/// ## File hierarchy
///
/// Modules can be mapped to a file/directory hierarchy.
///

#[path = "../src/my/mod.rs"]
// This declaration will look for a file named `my.rs` or `my/mod.rs` and will
// insert its contents inside a module named `my` under this scope
mod my;
use my::{call_private, call_public_in_my_mod};

#[path = "../src/my_mod.rs"]
mod my_mod;
use my_mod::my_mod::nested;

#[test]
fn test_modules() {
    my::function();
    my::nested::function();
    call_private();
    //my::nested::public_function_in_my_mod();
    call_public_in_my_mod();

    my_mod::my_mod::function();
    nested::function();
    my_mod::my_mod::call_private();
    my_mod::my_mod::call_public_in_my_mod();
}
