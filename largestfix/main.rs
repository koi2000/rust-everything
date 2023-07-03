fn largest<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();
    for item in list.iter() {
        if item > &largest {
            largest = item.clone();
        }
    }
    largest
}
fn main() {
    let str_list = vec![String::from("hello"), String::from("world")];
    let result = largest(&str_list);
    println!("The largest word is {}", result);
}
