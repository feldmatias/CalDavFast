use cal_dav_fast::models::mymodel::MyModel;
use factori::*;

factori!(MyModel, {
    default {
        name: String = "John".to_string(),
        age: u32 = 42
    }

    builder {
        MyModel::new(name, age)
    }

    /*mixin baby {
        age = 1,
    }*/
});
