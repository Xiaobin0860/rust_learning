///
/// # Traits
///
/// A `trait` is a collection of methods defined for an unknown type: `Self`.
/// They can access other methods declared in the same trait.
///
struct Sheep {
    naked: bool,
    name: &'static str,
}

trait Animal {
    //static method signature; `Self` refers to the implementor type.
    fn new(name: &'static str) -> Self;

    //instance method signatures; these will return a string.
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    //traits can provide default method definitions
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            //implementor methods can use the implementor's trait methods
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name());
            self.naked = true;
        }
    }
}

//implement the `Animal` trait for `Sheep`
impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep { naked: false, name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }

    //default trait methods can be overridden
    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name(), self.noise());
    }
}

#[test]
fn test_traits() {
    let mut dolly: Sheep = Animal::new("Dolly");
    dolly.talk();
    dolly.shear();
    dolly.talk();
}

///
/// ## Derive
///
/// The compiler is capable of providing basic implementations for some traits via the `#[derive]` attribute.
/// These traits can still be manually implemented if a more complex behavior is required.
///
/// * Comparision traits: `Eq`, `PartialEq`, `Ord`, `PartialOrd`
/// * `Clone`, to create `T` from `&T` via a copy
/// * `Copy`, to give a type `copy semantics` instead of `move semantics`
/// * `Hash`, to compute a hash from `&T`
/// * `Default`, to create an empty instance of a data type
/// * `Debug`, to format a value using the `{:?}` formatter
///
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}

struct Seconds(i32);

#[test]
fn test_derive() {
    let _one_second = Seconds(1);
    let foot = Inches(12);
    println!("One foot equals {:?}", foot);
    let meter = Centimeters(100.0);
    let cmp = if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };
    println!("One foot is {} than one meter.", cmp);
}
