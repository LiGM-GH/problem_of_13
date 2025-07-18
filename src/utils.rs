// Since this module contains utility functions, some code can be never used here.
// It's OK.
#![allow(dead_code, unused)]

pub type PrintOnDrop = Pod;

/// A debugging utility for multithreaded code.
/// It groups the debug messages that are pushed into it and then prints once dropped.
pub struct Pod(Vec<String>);

impl Pod {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, value: String) {
        self.0.push(value);
    }
}

impl Drop for Pod {
    fn drop(&mut self) {
        println!("{}", self.0.join("\n"));
    }
}
