/*
语言均有管理内存的方式，一些需要程序员自己分配和释放，一些拥有垃圾回收机制
Rust 则选择了第三种方式：通过所有权系统管理内存，编译器在编译时会根据一系列的规则进行检查。
如果违反了任何这些规则，程序都不能编译。
1. Rust 中的每一个值都有一个 所有者（owner）。
2. 值在任一时刻有且只有一个所有者。
3. 当所有者（变量）离开作用域，这个值将被丢弃。
 */

fn scope() {
    {
        // s 在这里无效，它尚未声明
        let s = "hello"; // 从此处起，s 是有效的

        // 使用 s
    } // 此作用域已结束，s 不再有效
}

fn Move() {
    // 基本数据类型此时的操作为拷贝
    let x = 5;
    let y = x;
    // 两个均可用
    println("{x},{y}");
    //
    let s1 = String::from("hello");
    let s2 = s1;
    // 此时第二个是不可用的
    /*
    当变量离开作用域后，Rust 自动调用 drop 函数并清理变量的堆内存。
    当 s2 和 s1 离开作用域，他们都会尝试释放相同的内存。这是一个叫做 二次释放
    */
    // println!(s1);
}

fn copyfn() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn main() {
    println!("Hello, world!");
}

fn testGiveFn() {
    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里 ...
                        // ... 所以到这里不再有效

    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，
                   // 但 i32 是 Copy 的，
                   // 所以在后面可继续使用 x
} // 这里，x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 没有特殊之处

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。
  // 占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。没有特殊之处

fn testReturnFn() {
    let s1 = gives_ownership(); // gives_ownership 将返回值
                                // 转移给 s1

    let s2 = String::from("hello"); // s2 进入作用域

    let s3 = takes_and_gives_back(s2); // s2 被移动到
                                       // takes_and_gives_back 中，
                                       // 它也将返回值移给 s3
} // 这里，s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
  // 所以什么也不会发生。s1 离开作用域并被丢弃

fn gives_ownership() -> String {
    // gives_ownership 会将
    // 返回值移动给
    // 调用它的函数

    let some_string = String::from("yours"); // some_string 进入作用域。

    some_string // 返回 some_string
                // 并移出给调用的函数
                //
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域
    a_string // 返回 a_string 并移出给调用的函数
}
