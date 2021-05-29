///
/// ##`From` and `Into`
///
/// The `From` and `Into` traits are inherently linked. and this is actually part of its impementation.
/// If you are able to convert A from type B, then it should be easy to believe that we should be able
/// to convert type B to type A.
///
/// ##`TryFrom` and `TryInto`
///
/// Similar to `From` and `Into`, return `Result`
///
/// ## To and from Strings
///
/// * `fmt::Display` trait, `ToString`
/// * `FromStr` trait
///
use std::convert::From;
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug)]
struct Number<T> {
    value: T,
}

impl<T> From<T> for Number<T> {
    fn from(v: T) -> Self {
        Number { value: v }
    }
}

#[derive(Debug, PartialEq)]
struct Even<T>(T);

impl TryFrom<i32> for Even<i32> {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        if v % 2 == 0 {
            Ok(Even::<i32>(v))
        } else {
            Err(())
        }
    }
}

#[test]
fn test_convert() {
    let my_str = "hello";
    let my_string = String::from(my_str);
    let my_string2: String = my_str.into();
    assert_eq!(my_string, my_string2);

    let i = 5i64;
    let n: Number<i64> = i.into();
    let n2 = Number::<i64>::from(i);
    println!("n={:?}, n2={:?}", n, n2);

    assert_eq!(Even::<i32>::try_from(8), Ok(Even::<i32>(8)));
    assert_eq!(Even::<i32>::try_from(1), Err(()));

    let result: Result<Even<i32>, ()> = 8.try_into();
    assert_eq!(result, Ok(Even::<i32>(8)));
    let result: Result<Even<i32>, ()> = 1.try_into();
    assert_eq!(result, Err(()));

    let parsed: i32 = "5".parse().unwrap();
    let parsed2 = "10".parse::<i32>().unwrap();
    assert_eq!(15, parsed + parsed2);
}

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// We implement the Default trait to use it as a fallback
// when the provided string is not convertible into a Person object
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

impl From<&str> for Person {
    fn from(s: &str) -> Person {
        let vs: Vec<&str> = s.split(',').collect();
        if vs.len() == 2 {
            if let Ok(n) = vs[1].parse::<usize>() {
                if !vs[0].is_empty() {
                    return Person {
                        name: String::from(vs[0]),
                        age: n,
                    };
                }
            }
        }
        Person::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        // Test that the default person is 30 year old John
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        // Test that John is returned when bad string is provided
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        // Test that "Mark,20" works
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        // Test that "Mark,twenty" will return the default person due to an error in parsing age
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}

use std::convert::AsRef;

// Obtain the number of bytes (not characters) in the given argument
// Add the AsRef trait appropriately as a trait bound
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

// Obtain the number of characters (not bytes) in the given argument
// Add the AsRef trait appropriately as a trait bound
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

#[cfg(test)]
mod tests_asref {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }
}
