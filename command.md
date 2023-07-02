
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
rustc编译参数
``` bash
--cfg SPEC - 传入自定义的条件编译参数，使用方法如:rustc --cfg hello main.rs
--crate-type - 指定编译输出类型，它的参数包括
     rustc --crate-type staticlib myhello.rs 链接库生成.a的链接库
     bin - 二进行可执行文件         -> bin或者lib二选一
     lib - 编译为库                -> 
     rlib - Rust库                -> 链接库生成.rlib的链接库(默认生成的库)
     dylib - 动态链接库            -> 链接库生成.so的链接库
     staticlib - 静态链接库        -> 链接库生成.a的链接库
--crate-name - 指定这个Crate的名字，默认是文件名，如main.rs编译成可执行文件时默认是main，但你可以指定它为foo
     rustc --crate-name foo main.rs
--emit - 指定编译器的输出。编译器默认是输出一个可执行文件或库文件，但你可以选择输出一些其它的东西用于Debug
     asm - 输出汇编
     llvm-bc - LLVM Bitcode；
     llvm-ir - LLVM IR，即LLVM中间码（LLVM Intermediate Representation）；
     obj - Object File（就是*.o文件）；
     link - 这个是要结合其它--emit参数使用，会执行Linker再输出结果；
     dep-info - 文件依赖关系（Debug用，类似于Makefile一样的依赖）。
     以上参数可以同时使用，使用逗号分割，如
     rustc --emit asm,llvm-ir,obj main.rs
     同时，在最后可以加一个=PATH来指定输出到一个特定文件，如
     rustc --emit asm=output.S,llvm-ir=output.ir main.rs
--print - 打印一些信息，参数有
     crate-name - 编译目标名；
     file-names - 编译的文件名；
     sysroot - 打印Rust工具链的根目录地址。
-g - 在目标文件中保存符号，这个参数等同于-C debuginfo=2。
-O - 开启优化，这个参数等同于-C opt-level=2。
-o FILENAME - 指定输出文件名，同样适用于--emit的输出。
--out-dir DIR - 指定输出的文件夹，默认是当前文件夹，且会忽略-o配置。
--test - 编译成一个单元测试可执行文件
--target TRIPLE - 指定目标平台.例如:rustc --target x86_64-apple-darwin

```