// 1. 运行: cargo run， 先编译再运行，相当于: cargo build, ./target/debug/hello-rust
// 2. 默认使用的是 debug 模式，编译器没有优化，使用 `--release` 开启release模式
// cargo run --release
// 然后会在 target 目录下创建 release 目录
// 3. cargo check 是我们在代码开发过程中最常用的命令，它的作用很简单：快速的检查一下代码能否编译通过。
// 因此该命令速度会非常快，能节省大量的编译时间
fn main() {
    println!("Hello, world!");
}
