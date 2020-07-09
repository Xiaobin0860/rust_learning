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
