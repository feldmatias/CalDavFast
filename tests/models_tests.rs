use cal_dav_fast::models::mymodel::MyModel;

#[test]
fn calculate_doubles_age() {
    let item = MyModel::new(String::from("test"), 15);

    let result = item.calculate();

    assert_eq!(result, 3)
}
