#[allow(dead_code)]
#[derive(Debug)]
pub enum Gender {
    Unknown,
    Male,
    Female,
}

#[allow(dead_code)]
enum ConnectionState {
    Init,
    SyncReceived(HalfOpen),
    SyncAckSent(HalfOpen),
    AckReceived(FullSession),
}

struct HalfOpen {}
struct FullSession {}

#[derive(Debug)]
pub struct User {
    pub name: String,
    age: u8,
    pub(crate) gender: Gender,
}

impl Default for User {
    fn default() -> Self {
        Self::new("".into(), 0, Gender::Unknown)
    }
}

impl User {
    pub fn new(name: String, age: u8, gender: Gender) -> Self {
        Self { name, age, gender }
    }
}
