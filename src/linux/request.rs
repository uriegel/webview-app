use serde::{Deserialize, Serialize};

pub struct Request<'a> {
    input: &'a str
}

impl<'a> Request<'a> {
    pub fn new(input: &'a str)->Self {
        Self { input }
    }

    pub fn get_input<T>(&self)->T where T: Deserialize<'a> {
        serde_json::from_str(&self.input).unwrap()
    }

    pub fn get_output<T>(&self, result: &T)->String where T: Serialize {
        serde_json::to_string(result).unwrap()
    }
}

