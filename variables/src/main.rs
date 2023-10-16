fn main() {
    // 不可变变量（变量遮蔽）
    let immutable_varible = 100;
    println!("不可变变量：{}", immutable_varible);
    let immutable_varible = "我是200";
    println!("不可变变量使用变量遮蔽：{}", immutable_varible);
    // 可变变量
    // ! 可变变量和变量遮蔽的有两个区别
    // ! 1: 可变变量重新赋值时无需let关键词，并且新值覆盖旧值后，旧值会直接销毁
    // ! 2: 可变变量重新赋值不能改变数据类型，而这点恰好是变量遮蔽的一大优势
    let mut mutable_varible = 1000;
    println!("可变变量：{}", mutable_varible);
    mutable_varible = 2000;
    println!("可变变量重新赋值：{}", mutable_varible);
    // 常量
    const CONST_VARIBLE: u32 = 10000;
    println!("常量：{}", CONST_VARIBLE);

    // 整形
    let integer32: u32 = 100;
    let interger_u32: i32 = -100;
    let interger_u64: u64 = 1000000000000000;

    // 浮点型
    let float32: f32 = 1.0;
    let float64: f64 = 1.00000000;

    // 布尔型
    let bool_value: bool = false;

    // 字符型
    let char_value: char = 'A';

    // 元组：定义后就拥有固定的长度，元祖中的每一个元素可以是不同的类型
    let tur: (i32,char,bool) = (100, 'C', false);
    let (x,y,z) = tur;
    println!("x: {},y: {},z: {}", tur.0, tur.1, tur.2);

    // 数组：定义后就拥有固定长度，数组的每一个元素必须是相同的类型
    let arr1: [i32; 5] = [1,2,3,4,5];
    let arr2 = [3;5]; // [3,3,3,3,3]
    println!("arr1[0]: {}", arr1[0]);

    // 函数调用
    firstFn();
    secondFn(32);

    // 表达式：末尾无需添加分号，添加分号就变成语句了
    let define_statement = {
        let x = 32;
        x + 1
    };
    println!("define_statement: {}", define_statement);

    // 函数 + 表达式
    five();
    plus_one(100);

    // 条件判断: condition必须是一个bool
    let condition: i32 = 32;
    if condition % 2 == 0 {
        println!("2的倍数");
    } else if condition % 3 == 0 {
        println!("3的倍数");
    } else {
        println!("其他倍数")
    }

    // 条件判断 + 表达式 => 赋初值
    let mut condition1 = if condition % 2 == 0 { 1 } else { 1 };

    // 无限循环 + 跳出循环 + 跳过本轮循环
    loop {
        println!("2333");
        if condition1 == 0 { 
            condition1 += 1;
            continue; 
        }
        break;
    }

    // 条件循环
    while condition1 != 0 {
        println!("{condition1}!");

        condition1 -= 1;
    }

    // 循环遍历
    for element in arr1 {
        println!("the value is: {element}");
    }

}

fn firstFn() {
    println!("第一个函数");
}

fn secondFn(num: i32) {
    println!("第二个函数: {}", num);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 5
}
