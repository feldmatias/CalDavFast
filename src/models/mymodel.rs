use std::fmt::Display;

pub struct MyModel {
    name: String,
    age: u32,
}

impl MyModel {
    pub fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }

    pub fn calculate(&self) -> u32 {
        self.age * 2
    }
}

impl Display for MyModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MyModel from impl {{ name: {}, age: {} }}",
            self.name, self.age
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_doubles_age() {
        let item = MyModel {
            name: String::from("test"),
            age: 15,
        };

        let result = item.calculate();

        assert_eq!(result, 30)
    }
}
