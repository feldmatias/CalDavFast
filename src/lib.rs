mod models;

use crate::models::mymodel::MyModel;

pub fn print_model() {
    let mymodel = MyModel::new("John".to_string(), 42);
    println!("this is my model {}", mymodel);
}