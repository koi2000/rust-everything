use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
/*
如果 Result 值是成员 Ok，unwrap 会返回 Ok 中的值。如果 Result 是成员 Err，unwrap 会为我们调用 panic!
 */
fn test1(){
    let greeting_file = File::open("hello.txt").unwrap();
}

fn read_username_from_file1() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        // 在 Err 的情况下，没有调用 panic!，使用 return 关键字提前结束整个函数，
        // 将来自 File::open 的错误值（现在在模式变量 e 中）作为函数的错误值传回给调用者。

        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

/*
File::open 调用结尾的 ? 会将 Ok 中的值返回给变量 username_file。
如果发生了错误，? 运算符会使整个函数提前返回并将任何 Err 值返回给调用代码。
 */
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result{
        Ok(file)=>file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("Helllo.txt"){
                Ok(file) => file,
                Err(error) => panic!("Couldn't create the file: {}", error),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    };
}
