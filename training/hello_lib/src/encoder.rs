use anyhow::Result;

pub trait Encoder {
    fn encode(&self) -> Result<Vec<u8>>;
}

pub struct Event<Id, Data> {
    id: Id,
    data: Data,
}

impl<Id, Data> Event<Id, Data> {
    pub fn new(id: Id, data: Data) -> Self {
        Self { id, data }
    }
}

impl<Id, Data> Encoder for Event<Id, Data>
where
    Id: Encoder,
    Data: Encoder,
{
    fn encode(&self) -> Result<Vec<u8>> {
        let mut result = self.id.encode()?;
        result.append(&mut self.data.encode()?);
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Encoder for i32 {
        fn encode(&self) -> Result<Vec<u8>> {
            //todo: encode
            Ok(vec![1, 2, 3, 4])
        }
    }

    impl Encoder for String {
        fn encode(&self) -> Result<Vec<u8>> {
            Ok(self.as_bytes().to_vec())
        }
    }

    #[test]
    fn it_works() {
        let e = Event::new(1, "Hello World!".to_string());
        let _ = e.encode().unwrap();
    }
}
