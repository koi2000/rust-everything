struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
// 元组结构体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


fn build_user(email: String, username: String) -> User {
    // 当字段名和参数名一样的时候，可以简写
    User {
        active: false,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        active: false,
        username: String::from("User1"),
        email: String::from("koi20000@163.com"),
        sign_in_count: 1,
    };
    // 不可变对象的属性不可被改变
    user1.username = String::from("111");
    println!("{}", user1.active);
    println!("{}", user1.username);
    println!("{}", user1.email);
    println!("{}", user1.sign_in_count);
    // 可以从其他对象中获取

    /*
    创建 user2后不能就再使用 user1 了，因为 user1 的 username 字段中的 String 被移到 user2 中。
    如果给 user2 的 email 和 username 都赋予新的 String 值，
    从而只使用 user1 的 active 和 sign_in_count 值，那么 user1 在创建 user2 后仍然有效。
     */
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
