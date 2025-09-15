// 02. 变量和类型库

pub fn run_demo() {
    println!("📊 变量和类型演示");
    println!("==================");
    
    // 基本类型
    basic_types();
    println!();
    
    // 变量和可变性
    variables_and_mutability();
    println!();
    
    // 变量遮蔽
    variable_shadowing();
    println!();
    
    // 常量
    constants();
    println!();
    
    // 类型转换
    type_conversion();
    println!();
    
    // 字符串类型
    string_types();
    println!();
    
    // 数组和元组
    arrays_and_tuples();
}

fn basic_types() {
    println!("=== 基本类型 ===");
    
    // 整数类型
    let a: i32 = 42;
    let b: u32 = 42;
    let c: i64 = 42;
    println!("整数: i32={}, u32={}, i64={}", a, b, c);
    
    // 浮点数类型
    let x: f32 = 3.14;
    let y: f64 = 3.14159265359;
    println!("浮点数: f32={}, f64={}", x, y);
    
    // 布尔类型
    let t: bool = true;
    let f: bool = false;
    println!("布尔值: true={}, false={}", t, f);
    
    // 字符类型
    let c: char = '中';
    let emoji: char = '😀';
    println!("字符: {}, {}", c, emoji);
}

fn variables_and_mutability() {
    println!("=== 变量和可变性 ===");
    
    // 不可变变量
    let x = 5;
    println!("x 的值是: {}", x);
    // x = 6; // 这行会编译错误！
    
    // 可变变量
    let mut y = 5;
    println!("y 的初始值是: {}", y);
    y = 6;
    println!("y 修改后的值是: {}", y);
}

fn variable_shadowing() {
    println!("=== 变量遮蔽 ===");
    
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

fn constants() {
    println!("=== 常量 ===");
    
    const MAX_POINTS: u32 = 100_000;
    println!("最大点数: {}", MAX_POINTS);
    
    // 常量必须显式指定类型
    const PI: f64 = 3.14159;
    println!("圆周率: {}", PI);
}

fn type_conversion() {
    println!("=== 类型转换 ===");
    
    // 显式类型转换
    let x: i32 = 42;
    let y: f64 = x as f64;
    println!("i32 {} 转换为 f64: {}", x, y);
    
    // 字符串转数字
    let s = "42";
    let n: i32 = s.parse().unwrap();
    println!("字符串 '{}' 转换为数字: {}", s, n);
    
    // 数字转字符串
    let num = 42;
    let str_num = num.to_string();
    println!("数字 {} 转换为字符串: '{}'", num, str_num);
}

fn string_types() {
    println!("=== 字符串类型 ===");
    
    // 字符串字面量（&str）
    let s1 = "Hello, world!";
    println!("字符串字面量: {}", s1);
    
    // String 类型
    let mut s2 = String::from("Hello");
    s2.push_str(", world!");
    println!("String 类型: {}", s2);
    
    // 字符串切片
    let s3 = String::from("Hello, world!");
    let hello = &s3[0..5];
    let world = &s3[7..12];
    println!("字符串切片: '{}', '{}'", hello, world);
}

fn arrays_and_tuples() {
    println!("=== 数组和元组 ===");
    
    // 数组
    let a = [1, 2, 3, 4, 5];
    println!("数组: {:?}", a);
    println!("第一个元素: {}", a[0]);
    
    // 元组
    let tup = (500, 6.4, 1);
    println!("元组: {:?}", tup);
    println!("元组第一个元素: {}", tup.0);
    
    // 元组解构
    let (x, y, z) = tup;
    println!("解构: x={}, y={}, z={}", x, y, z);
}
