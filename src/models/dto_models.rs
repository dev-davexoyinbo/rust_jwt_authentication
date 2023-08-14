use serde::{Deserialize, Serialize};
use core::fmt;
use std::ops::Deref;

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseDTO<T> {
    message: String,
    data: T,
}

impl<T: serde::Serialize> fmt::Display for ResponseDTO<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
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