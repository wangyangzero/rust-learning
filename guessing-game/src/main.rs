use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("请猜测一个数字");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("请输入您猜测的数字");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("读取失败");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
            
        println!("你猜测的数字是: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("被你猜中了！");
                break;
            }
        }
    }
}