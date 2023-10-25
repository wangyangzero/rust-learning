mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}

// 在模块中可以用super取到模块上级，从而进行相对路径的访问
mod other_front_house {
    pub fn get_waitlist () {
        super::front_of_house::hosting::add_to_waitlist()
    }

    pub fn eat_at_restaurant() {
        super::eat_at_restaurant()
    }
}



// 创建公有结构体（每个属性都具有私有性/公有性）
mod restaurant {
    pub struct Breakfast {
        pub meal: String,
        drink: String,
    }

    impl Breakfast {
        pub fn summer(meal: &str) -> Breakfast {
            Breakfast { meal: String::from(meal), drink: String::from("茶") }
        }
    }
}

pub fn eat_on_summer() {
    let mut summer_breakfast = crate::restaurant::Breakfast::summer("牛排");
    summer_breakfast.meal = String::from("羊腿"); 
    // 下面这段会因为尝试篡改私有属性而报错
    summer_breakfast.drink = String::from("可乐");
}

// 创建公有枚举（每个枚举值的公有性/私有性跟随枚举体）
mod other_restaurant {
    pub enum Drink {
        Cola,
        Tea,
        Coffee,
        Milk
    }
}

// 使用 use 关键字将名称引入作用域
use front_of_house::hosting;

pub fn eat_at_other_restaurant() {
    hosting::add_to_waitlist();
}

// 引入不同模块下的同名方法常用手段：

use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}

// 另外一种解法

use std::fmt::Result;
use std::io::Result as IResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IResult<()> {
    // --snip--
}

// 使用 pub use 重导出名称
// 外界可以通过hosting来访问add_to_waitlist
pub use crate::front_of_house::hosting;

// 嵌套路径来消除大量的 use 行
use std::cmp::Ordering;
use std::io;

use std::{cmp::Ordering, io};

use std::io;
use std::io::Write;

use std::io::{self, Write};
