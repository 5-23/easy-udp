use std::str::from_utf8;

use serde::{Deserialize, Serialize};

mod server;

enum Event {
    Connect,
    Disconnect,
    Message(String),
}

impl Event {
    #[allow(non_snake_case)]
    pub fn Message<T: ?Sized + serde::Serialize>(value: &T) -> Self {
        let binding = bincode::serialize(value).unwrap();
        let bin = binding.as_slice();
        Self::Message(from_utf8(bin).unwrap().to_string())
    }
    pub fn set<'a, T: serde::Deserialize<'a>>(value: T) {
        todo!()
    } 
}
