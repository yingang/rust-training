use anyhow::Result;

pub trait Encoder {
    fn encode(&self) -> Result<Vec<u8>>;
}

pub struct Event<Id, Data> {
    id: Id,
    data: Data,
}

impl<Id: Encoder, Data: Encoder> Event<Id, Data> {
    pub fn new(id: Id, data: Data) -> Self { Self { id, data } }

    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut result = self.id.encode()?;
        result.append(&mut self.data.encode()?);
        Ok(result)
    }
}

impl Encoder for i32 {
    fn encode(&self) -> Result<Vec<u8>> {
        Ok(vec![1, 2, 3, 4])
    }
}

impl Encoder for String {
    fn encode(&self) -> Result<Vec<u8>> {
        Ok(self.as_bytes().to_vec())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let event = Event::new(1, "hello world".to_string());
        let result = event.encode().unwrap();
        println!("{:?}", result);

        let event = Event::new("hi".to_string(), "hello world".to_string());
        let result = event.encode().unwrap();
        println!("{:?}", result);
    }
}