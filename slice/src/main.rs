/*
字符串slice是string中一部分值的引用
 */

fn first_world(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
fn main() {
    let mut s = String::from("Hello, world!");
    let word = first_world(&s);
    // clear 需要清空 String，它尝试获取一个可变引用。
    // 在调用 clear 之后的 println! 使用了 word 中的引用，
    // 所以这个不可变的引用在此时必须仍然有效。
    // Rust 不允许 clear 中的可变引用和 word 中的不可变引用同时存在，因此编译失败。
    // s.clear();
    println!("the first word is {}", word);
}
