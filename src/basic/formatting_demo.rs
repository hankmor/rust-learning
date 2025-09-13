// 格式化占位符演示模块

// 基本格式化演示
pub fn basic_formatting_demo() {
    println!("=== 基本格式化演示 ===");
    
    let number = 42;
    let text = "Hello";
    let vector = vec![1, 2, 3, 4, 5];
    let tuple = (1, "hello", 3.14);
    
    // {} - Display 格式
    println!("数字 (Display): {}", number);
    println!("文本 (Display): {}", text);
    
    // {:?} - Debug 格式
    println!("数字 (Debug): {:?}", number);
    println!("文本 (Debug): {:?}", text);
    println!("向量 (Debug): {:?}", vector);
    println!("元组 (Debug): {:?}", tuple);
}

// 不同精度和格式演示
pub fn precision_formatting_demo() {
    println!("=== 精度和格式演示 ===");
    
    let pi = 3.14159265359;
    let number = 12345;
    
    // 控制小数位数
    println!("π (2位小数): {:.2}", pi);
    println!("π (4位小数): {:.4}", pi);
    
    // 控制总宽度
    println!("数字 (宽度10): {:10}", number);
    println!("数字 (左对齐): {:<10}", number);
    println!("数字 (右对齐): {:>10}", number);
    println!("数字 (居中对齐): {:^10}", number);
    
    // 填充字符
    println!("数字 (用0填充): {:010}", number);
    println!("数字 (用*填充): {:*^10}", number);
}

// 复杂类型格式化演示
pub fn complex_formatting_demo() {
    println!("=== 复杂类型格式化演示 ===");
    
    let person = ("张三", 25, 175.5);
    let colors = ["红色", "绿色", "蓝色"];
    let matrix = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9]
    ];
    
    // 元组格式化
    println!("人员信息: {:?}", person);
    println!("人员信息 (美化): {:#?}", person);
    
    // 数组格式化
    println!("颜色数组: {:?}", colors);
    println!("颜色数组 (美化): {:#?}", colors);
    
    // 嵌套结构格式化
    println!("矩阵: {:?}", matrix);
    println!("矩阵 (美化): {:#?}", matrix);
}

// 自定义结构体格式化演示
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    name: String,
}

pub fn custom_struct_formatting_demo() {
    println!("=== 自定义结构体格式化演示 ===");
    
    let point = Point { x: 10, y: 20 };
    let rect = Rectangle {
        width: 100,
        height: 50,
        name: "我的矩形".to_string(),
    };
    
    // 结构体格式化
    println!("点: {:?}", point);
    println!("点 (美化): {:#?}", point);
    
    println!("矩形: {:?}", rect);
    println!("矩形 (美化): {:#?}", rect);
}

// 错误处理格式化演示
pub fn error_formatting_demo() {
    println!("=== 错误处理格式化演示 ===");
    
    let result: Result<i32, String> = Ok(42);
    let error_result: Result<i32, String> = Err("出错了".to_string());
    
    // Result 类型格式化
    println!("成功结果: {:?}", result);
    println!("错误结果: {:?}", result);
    
    // Option 类型格式化
    let some_value: Option<i32> = Some(100);
    let none_value: Option<i32> = None;
    
    println!("有值: {:?}", some_value);
    println!("无值: {:?}", none_value);
}

// 主演示函数
pub fn formatting_demo() {
    println!("🎨 格式化占位符演示");
    println!("================================");
    
    basic_formatting_demo();
    println!();
    
    precision_formatting_demo();
    println!();
    
    complex_formatting_demo();
    println!();
    
    custom_struct_formatting_demo();
    println!();
    
    error_formatting_demo();
    println!();
    
    println!("💡 总结：");
    println!("  - {{}} 用于用户友好的显示格式 (Display trait)");
    println!("  - {{:?}} 用于调试输出 (Debug trait)");
    println!("  - {{:#?}} 用于美化的调试输出");
    println!("  - 可以控制精度、宽度、对齐方式等");
    println!("  - 复杂类型通常需要实现 Debug trait 才能使用 {{:?}}");
}
