use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseDTO<T> {
    message: String,
    data: T,
}

impl<T> ResponseDTO<T> {
    pub fn new(data: T) -> Self {
        return ResponseDTO {
            data,
            message: String::from(""),
        };
    }

    pub fn message(mut self, message_str: &str) -> Self {
        self.message = String::from(message_str);
        return self;
    }
}

impl<T> Deref for ResponseDTO<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        return &self.data;
    }
}