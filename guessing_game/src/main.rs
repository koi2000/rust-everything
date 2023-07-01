use rand::Rng;
use std::{io, cmp::Ordering};

fn main() {
    println!("猜数！");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    print!("神秘数字是：{}", secret_number);
    loop {
        println!("猜测一个数");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("无法读取行");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜测的数为：{}", guess);
        match guess.cmp(&secret_number) {  
            Ordering::Less => println!("Too small"),
            Ordering::Greater   => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
