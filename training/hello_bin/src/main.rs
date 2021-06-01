mod types;
use types::{Gender, User};
mod value_tour;

fn main() {
    println!("Hello, world!");
    value_tour::value_tour();

    let u = User::new("lxb".to_owned(), 18, Gender::Male);
    println!("{:?} name={}", u, u.name);
}
