use std::collections::HashMap;

fn test1() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // 只在没有值的时候插入
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}
/*
or_insert 方法返回这个键的值的一个可变引用（&mut V）。
这里我们将这个可变引用储存在 count 变量中，
所以为了赋值必须首先使用星号（*）解引用 count。
这个可变引用在 for 循环的结尾离开作用域，这样所有这些改变都是安全的并符合借用规则。
 */
fn test2(){
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Green"), 50);

    let team_name = String::from("Blue");
    /*
    如果某个键在哈希 map 中没有对应的值，get 会返回 None。
    程序中通过调用 copied 方法来获取一个 Option<i32> 而不是 Option<&i32>，
    接着调用 unwrap_or 在 score 中没有该键所对应的项时将其设置为零。
     */
    let score = scores.get(&team_name).copied().unwrap_or(0);
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    /*
    对于像 i32 这样的实现了 Copy trait 的类型，其值可以拷贝进哈希 map。
    对于像 String 这样拥有所有权的值，其值将被移动而哈希 map 会成为这些值的所有者
     */
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 这里 field_name 和 field_value 不再有效，
}
