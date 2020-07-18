///
/// # Traits
///
/// A `trait` is a collection of methods defined for an unknown type: `Self`.
/// They can access other methods declared in the same trait.
///
/// ## Returning Traits with `dyn`
///
/// The Rust compiler needs to know how mutch space every function's return type requires.
/// This means all your functions have to return a concrete type. So, you can't write a function
/// that returns `Animal`. However you can use `Box<dyn Animal>`
///
struct Sheep {
    naked: bool,
    name: &'static str,
}
struct Cow {
    name: &'static str,
}

trait Noise {
    fn noise(&self) -> &'static str;
}
trait Animal: Noise {
    //instance method signatures; these will return a string.
    fn name(&self) -> &'static str;

    //traits can provide default method definitions
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}
trait NewAnimal: Animal {
    //static method signature; `Self` refers to the implementor type.
    fn new(name: &'static str) -> Self;
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

impl Noise for Sheep {
    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }
}

//implement the `Animal` trait for `Sheep`
impl Animal for Sheep {
    fn name(&self) -> &'static str {
        self.name
    }

    //default trait methods can be overridden
    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name(), self.noise());
    }
}

impl NewAnimal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep { naked: false, name }
    }
}

impl Noise for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}
impl Animal for Cow {
    fn name(&self) -> &'static str {
        self.name
    }
}

fn rand_animal() -> Box<dyn Animal> {
    if rand::random() {
        Box::new(Sheep {
            name: "sheep",
            naked: false,
        })
    } else {
        Box::new(Cow { name: "cow" })
    }
}

#[test]
fn test_traits() {
    let mut dolly: Sheep = NewAnimal::new("Dolly");
    dolly.talk();
    dolly.shear();
    dolly.talk();
    for _i in 1..=10 {
        let animal = rand_animal();
        animal.talk();
    }
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

///
/// ## Operator Overloading
///
use std::ops;
struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

impl<T> ops::Add<T> for Foo {
    type Output = FooBar;
    fn add(self, _rhs: T) -> Self::Output {
        println!("> Foo.add(Bar) was called");
        FooBar
    }
}
impl ops::Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called");

        BarFoo
    }
}
#[test]
fn test_ops() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Foo + 1 = {:?}", Foo + 1);
    println!("Foo + 1 = {:?}", Foo + "1");
    println!("Bar + Foo = {:?}", Bar + Foo);
}
