use cal_dav_fast::models::mymodel::factory::*;
use cal_dav_fast::models::mymodel::MyModel;
use factori::create;

mod common;
use common::test_runner::run_test;

#[test]
fn calculate_doubles_age() {
    run_test(|| {
        let item = MyModel::new(String::from("test"), 15);

        let result = item.calculate();

        assert_eq!(result, 30);

        let item2 = create!(MyModel);

        let result2 = item2.calculate();

        assert_eq!(result2, 84);
        println!("this is my model {}", item2)
    });
}
