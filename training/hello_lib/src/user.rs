use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub name: String,
    age: u8,
    pub gender: Gender,
}

impl User {
    pub fn new(name: String, age: u8, gender: Gender) -> Self {
        Self { name, age, gender }
    }

    pub fn to_string(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }

    pub fn from_str(data: &str) -> Result<Self> {
        Ok(serde_json::from_str(data)?)
    }
}

impl Default for User {
    fn default() -> Self {
        Self::new("".into(), 0, Gender::Unknown)
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Gender {
    Unknown,
    Male,
    Female,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let u = User::new("lxb".into(), 18, Gender::Male);
        let s = u.to_string().unwrap();
        let u2 = User::from_str(s.as_str()).unwrap();
        assert_eq!(u, u2);
    }
}
