use std::fmt::Display;

pub struct MyModel {
    name: String,
    age: u32,
}

impl MyModel {
    pub fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }
}

impl Display for MyModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyModel from impl {{ name: {}, age: {} }}", self.name, self.age)
    }
}