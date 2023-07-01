fn main() {
    let x = 5;
    // rust默认所有的变量均不可变
    // 需加mut关键字才可变
    // x = 6;
    let mut x = 5;
    x = 6;
    println!("x: {}", x);
    println!("================================");
    // 常量
    const THREE_HOUSES_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("x: {}", THREE_HOUSES_IN_SECONDS);
    println!("================================");
    // shadowing 两个变量的类型可以是不同的
    let z = 5;
    let z = z + 1;
    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {z}");
    }
    println!("The value of z is: {z}");
    // mut 不能改变变量的类型，shadowing可以
    // let mut spaces = "   ";
    // spaces = spaces.len();
    // 函数调用
    another(32, 'x');

    // 语句（Statements）是执行一些操作但不返回值的指令。
    // 表达式（Expressions）计算并产生一个值。
    let y = 6; // 语句
               // 表达式，rust默认返回最后一个表达式
    let y = {
        let x = 3;
        x + 1
    };

    // 带有返回值的函数
    let x = five();
    println!("The value of x is: {x}");
}
// 函数的传参
fn another(value: u32, unit_label: char) {
    println!("{value},{unit_label}");
}

// 带有返回值的函数
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    // 表达式可以作为函数返回值
    x + 1
    // 语句不可以
    // x+1;
}
