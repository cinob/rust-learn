// 声明front_of_house模块，其代码位于src/front_of_house.rs文件中
mod front_of_house;

// pub use crate::front_of_house::hosting;

// use将模块引入作用域
// use 绝对路径
use crate::front_of_house::hosting;
// use 导入多个
// use crate::front_of_house::{hosting, serving::serve_order};
// use 通配符导入 常用于测试
// use crate::front_of_house::*;


// use 相对路径
// use self::front_of_house::hosting;

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // 从父模块开始构造相对路径
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        // seasonal_fruit为私有字段，需要提供一个公共方法来创建Breakfast实例
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    // 公有枚举中的所有字段也为公有
    pub enum Appetizer {
        Soup,
        Salad
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();

    // use引入的模块
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 公共字段，可以修改
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // 私有字段，不可修改
    // meal.seasonal_fruit = String::from("Watermelon");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}


