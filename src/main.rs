// 声明模块
mod basic;
mod complex_types;

fn main() {
    println!("🚀 Rust 基础演示");
    println!("=====================");
    println!();
    
    // 调用 Hello World 综合演示
    basic::hello_world::hello_world_demo();
    println!();
    
    // 调用变量和可变性演示
    basic::vars::vars_demo();
    println!();
    
    // 调用格式化演示
    basic::formatting_demo::formatting_demo();
    println!();
    
    // 调用 Debug 演示
    basic::debug_demo::debug_demo();
    println!();
    
    // 调用复合类型演示
    println!("🔧 复合类型演示");
    println!("================================");
    
    // 调用字符串演示
    complex_types::str::str_demo();
    println!();
    
    // 调用数组和向量演示
    complex_types::arrays::arrays_demo();
    println!();
    
    // 调用元组演示
    complex_types::tuples::tuples_demo();
    println!();
    
    // 调用结构体演示
    complex_types::structs::struct_demo();
    println!();
    
    // 调用属性 vs 方法演示
    complex_types::property_vs_method_demo::property_vs_method_demo();
    println!();
    
    // 调用可见性控制对比演示
    complex_types::visibility_comparison_demo::visibility_comparison_demo();
    println!();
}
