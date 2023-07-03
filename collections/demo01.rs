fn main() {
    let v1 :Vec<i32> = Vec::new();
    let v2 = vec![1,2,3,4,5];
    let mut v3 = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    let third:&i32 = &v[2];
    printlt!("The third element of the vector is{third}")
    // 使用get返回的值为option
    let third:Option<&i32> = v3.get(2);
    match third {
        Some(third) => println!("The third element of the vector is{third}"),
        None => println!("There is no third element of the vector"),
    }
    // 遍历vector
    // let v = vec![100,32,57];
    // for i in &V{
    //     println!("{i}",);
    // }
    // 遍历更新
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}