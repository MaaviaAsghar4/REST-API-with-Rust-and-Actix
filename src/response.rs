// use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Response<T> {
    pub results: Vec<T>,
}

impl<T> Response<T> {
    pub fn new() -> Self {
        Self { results: vec![] }
    }
}
