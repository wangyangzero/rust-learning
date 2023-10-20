fn main() {
    // 基础结构体使用
    struct User {
        username: String,
        password: String,
        phone: String,
        email: String,
    }
    let user1 = User {
        username: String::from("some_username"),
        password: String::from("acc"),
        phone: String::from("911"),
        email: String::from("some_emial")
    };

    let user2 = User {
        username: user1.username,
        password: String::from("another"),
        phone: String::from("119"),
        email: user1.email
    };

    // 元祖结构体
    // 打印结构体必须开启debug模式
    #[derive(Debug)]
    struct Color (i32,i32,i32);
    struct Point(i32,i32);
    let black = Color(0,0,0);
    let point = Point(0,0);

    // 没有任何字段的类单元结构
    struct AlwaysEqual;
    let alwaysEqual = AlwaysEqual;

    // 打印结构体
    println!("color: {:?}", black);
    // 注意dbg宏会获取并返回所有权，在使用时通常入参为引用
    dbg!(&black);

    // 定义结构体方法
    #[derive(Debug)]
    struct Rect {
        width: u32,
        height: u32,
    }

    impl Rect {
        // 普通方法
        fn getArea(&self) -> u32 {
            self.width * self.height
        }
        // Rust没有构造函数和析构函数的概念，所以方法名与属性相同并不是getter和构造器
        fn width(&self) -> bool {
            self.width > 0
        }
        // 多个入参
        fn can_hold(&self, other: &Rect) -> bool {
            self.width >= other.width && self.height >= other.height
        }

        // 入参不含self,Rust没有new关键词，可以用下面函数实现一个new方法
        fn new(width: u32, height: u32) -> Self {
            Self { width, height }
        }
    }

}
