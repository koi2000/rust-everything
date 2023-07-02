fn testref() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}
// 以引用的方式传入，不会更换所有权
fn calculate_length(s: &String) -> usize {
    s.len()
}

// 传入引用后不可修改
// fn testchange() {
//     let s = String::from("hello");
//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// 可变的引用
// 如果有一个对该变量的可变引用，就不能再创建对该变量的引用
fn testmutref() {
    let mut s = String::from("hello");

    testChange(&mut s);
}

fn testChange(some_string: &mut String) {
    some_string.push_str(", world");
}

// 尝试返回一个已被释放的值
// fn testdangle() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String {
//     // dangle 返回一个字符串的引用

//     let s = String::from("hello"); // s 是一个新字符串

//     &s // 错误做法：返回字符串 s 的引用
// } // 这里 s 离开作用域并被丢弃。其内存被释放。

fn testnodangle() {
    let string = no_dangle();
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

// 在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用。
// 引用必须总是有效的。
fn main() {
    println!("Hello, world!");
}
