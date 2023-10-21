fn main() {
    // 简单枚举
    enum IpAddress {
        V4,
        V6,
    }

    // 带构造器的枚举
    enum IpAddressTwo {
        V4(String),
        V6(String),
    }

    // 枚举的构造器可以是任意类型
    struct IpStruct {
        ip: String,
        name: String,
    }

    enum Message {
        ip(IpStruct),
        color(u32,u32,u32),
    }

    // 枚举甚至也能挂载方法
    enum Shape {
        Circle(f64),
        Rectangle(f64,f64),
        Square(f64),
    }

    impl Shape {
        fn area(&self) -> f64 {
            match &self {
                Self::Circle(radius) => 3.1415926535 * radius * radius,
                Self::Rectangle(width, height) => width * height,
                Self::Square(side) => side * side,
            }
        }
    }

    let circle = Shape::Circle(10.0);
    let rectangle = Shape::Rectangle(10.0, 20.0);
    let square = Shape::Square(10.0);

    println!("circle: {}\n rectangle: {}\n square: {}\n", circle.area(), rectangle.area(), square.area());

    // 更加复杂的match例子
    #[derive(Debug)]
    enum UsState {
        NewYork,
        Florida,
    }

    enum Coin {
        Quarter(UsState)
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Quarter(state) => {
                println!("city is {:?}", state);
                25
            }
        }
    }
    let quarter_coin = Coin::Quarter(UsState::NewYork);
    value_in_cents(quarter_coin);

    // match结合Option枚举(必须处理None和Some情况，从而规避了空值)
    fn plus_one(num: Option<i64>) -> Option<i64> {
        match num {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // match的语法糖
    let max_value = Some(1000);
    // 这种写法只在match匹配时执行，因此无需额外的判空
    if let Some(num) = max_value {
        println!("max value is {}", num);
    }

}