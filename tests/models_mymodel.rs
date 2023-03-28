use cal_dav_fast::models::mymodel::MyModel;

mod common;
use common::test_runner::run_test;

#[test]
fn calculate_doubles_age() {
    run_test(|| {
        let item = MyModel::new(String::from("test"), 15);

        let result = item.calculate();

        assert_eq!(result, 30)
    });
}
