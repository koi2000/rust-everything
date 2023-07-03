enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // 使用枚举可以让一个vetcor中存储多种类型的数据
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
