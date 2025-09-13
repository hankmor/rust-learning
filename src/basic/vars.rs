// 变量和可变性相关的演示模块

// 演示不可变变量
fn immutable_demo() {
    println!("=== 不可变变量演示 ===");
    let x = 5;
    println!("x 的值是: {}", x);
    // x = 6; // 这行会编译错误！x 是不可变的
    println!("x 仍然是: {}", x);
}

// 演示可变变量
fn mutable_demo() {
    println!("=== 可变变量演示 ===");
    let mut y = 5;
    println!("y 的初始值是: {}", y);
    y = 6;
    println!("y 修改后的值是: {}", y);
}

// 演示变量遮蔽
fn shadowing_demo() {
    println!("=== 变量遮蔽演示 ===");
    let x = 5;
    println!("第一个 x: {}", x);
    
    let x = x + 1;
    println!("遮蔽后的 x: {}", x);
    
    let x = x * 2;
    println!("再次遮蔽的 x: {}", x);
    
    // 遮蔽可以改变类型
    let spaces = "   ";
    println!("spaces 是字符串: '{}'", spaces);
    let spaces = spaces.len();
    println!("spaces 现在是数字: {}", spaces);
}

// 演示常量
fn constants_demo() {
    println!("=== 常量演示 ===");
    const MAX_POINTS: u32 = 100_000;
    println!("最大点数: {}", MAX_POINTS);
    
    // 常量必须显式指定类型
    const PI: f64 = 3.14159;
    println!("圆周率: {}", PI);
}

// 主演示函数
pub fn vars_demo() {
    println!("📊 变量和可变性演示");
    println!("================================");
    
    immutable_demo();
    println!();
    
    mutable_demo();
    println!();
    
    shadowing_demo();
    println!();
    
    constants_demo();
    println!();
    
    println!("💡 总结：");
    println!("  - let 创建不可变绑定");
    println!("  - let mut 创建可变绑定");
    println!("  - 变量遮蔽允许重新绑定");
    println!("  - const 创建编译时常量");
}
