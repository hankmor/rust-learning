// 声明模块
mod str;
mod vars;
mod hello_world;

fn main() {
    println!("🚀 Rust 学习演示程序");
    println!("=====================");
    println!();
    
    // 调用 Hello World 综合演示
    hello_world::hello_world_demo();
    println!();
    
    // 调用变量和可变性演示
    vars::vars_demo();
    println!();
    
    // 调用字符串借用演示
    str::str_borrow_demo();
}
