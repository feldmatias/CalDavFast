use cal_dav_fast::models::mymodel::MyModel;
use factori::create;

mod common;
use common::*;

#[test]
fn calculate_doubles_age3() {
    run_test(|| {
        let item = MyModel::new(String::from("test"), 15);

        let result = item.calculate();

        assert_eq!(result, 30);

        let item2 = create!(MyModel);

        let result2 = item2.calculate();

        assert_eq!(result2, 84);
        println!("this is my model {}", item2);

        let item3 = create!(MyModel, age: 421);
        assert_eq!(item3.calculate(), 842);

        let item4 = create!(MyModel, :baby);
        assert_eq!(item4.calculate(), 2);
    });
}
