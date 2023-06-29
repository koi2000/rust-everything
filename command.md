
# Rust!
## 基本命令
rustc main.rs

cargo new project

cargo build

cargo run

cargo check 确保可以通过编译，不会生成任何文件

cargo build --release 进行优化，正式发布

cargo.toml
cargo的配置格式
``` toml
[package]
name = "hello"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```
