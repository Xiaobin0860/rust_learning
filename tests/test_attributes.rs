///
/// # Attributes
///
/// An attribute is metadata applied to some module, crate or item. This metadata can be used to/for:
///
/// * conditional compilation of code
/// * set crate name, version and type(binary or library)
/// * disable lints(warnings)
/// * enable compiler features(macros, glob imports, etc.)
/// * link to a foreign library
/// * mark functions as unit tests
/// * mark functions that will be part of a benchmark
///
/// When attributes apply to whole crate, their syntax is `#![crate_attribute]`, and when they apply
/// to a module or item, the syntax is `#[item_attribute]`.
///
/// Attributes can take arguments with different syntaxes:
///
/// * `#[attribute = "value"]`
/// * `#[attribute(key = "value")]`
/// * `#[attribute(value)]`
///
/// Attributes can have mutiple values and can be separated over multiple lines, too:
///
/// ```
/// #[attribute(value1, value2)]
///
/// #[attribute(value, value2, value3,
///             value4, value5)]
/// ```
///
/// ## `dead_code`
///
/// The compiler provides a `dead_code` lint that will warn about unused functions.
/// An attribute can be used to disable the lint. `#[allow(dead_code)]`
///
/// ## Crates
///
/// The `crate_type` attribute can be used to tell the compiler whether a crate is
/// a binary or a library, and the `crate_name` attribute can be used to set the name of the crate.
///
/// ```
/// // This crate is a library
/// #![crate_type = "lib"]
/// // The library is named "rary"
/// #![crate_name = "rary"]
/// ```
///
/// ## `cfg`
///
/// Conditional compilation is possible throuth two different operators:
///
/// * the `cfg` attribute: `#[cfg(...)]` in attribute position
/// * the `cfg!` macro: `cfg!(...)` in boolean expressions
///
/// ### Custom
///
/// Some conditionals like `target_os` are provided by `rustc`, but custom conditionals must
/// be passed to `rustc` using the `--cfg` flag.
///
/// ```
/// rustc --cfg some_condition custom.rs
/// ```
///

// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// And this function only gets compiled if the target OB is *not* linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

#[test]
fn test_attributes() {
    are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes, It's definitely linux!");
    } else {
        println!("Yes, It's definitely *not* linux!");
    }
}
