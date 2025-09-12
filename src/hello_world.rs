// Hello World 相关的演示模块

// 基础 Hello World 演示
fn basic_hello_demo() {
    println!("=== 基础 Hello World 演示 ===");
    println!("Hello, world!");
    
    // 多语言问候
    let chinese = "你好,世界!";
    let english = "Hello World!";
    let langs = [chinese, english];
    for lang in langs {
        println!("{}", &lang);
    }
}

// 语句和表达式演示
fn statement_expression_demo() {
    println!("=== 语句和表达式演示 ===");
    let x = {
        let a = 1; // 语句
        a + 1 // 表达式
    };
    println!("x = {}", x);
    
    // 单元类型演示
    let r = return_unit();
    assert_eq!(r, ());
    println!("r == (): {}", r == ()); // true
}

fn return_unit() {
    let a = 2;
    let _b = if a % 2 == 1 { "奇数" } else { "偶数" };
}

// 字符串操作演示
fn string_operations_demo() {
    println!("=== 字符串操作演示 ===");
    let mut s = String::from("hello"); // 创建动态字符串String类型, 在堆上分配内存
    s.push_str(", world!"); // push_str() 在字符串后追加字面值
    println!("{}", s); // 将打印 `hello, world!`
}

// 所有权演示
fn ownership_demo() {
    println!("=== 所有权演示 ===");
    
    // 浅拷贝
    let x = 5;
    let y = x; // y 有效, 基本类型i32在栈上分配内存, y将x的值拷贝了一份, 即 浅拷贝
    println!("x = {}, y = {}", x, y);
    
    // 所有权移动
    let s1 = String::from("hello"); // String动态字符串在堆上分配内存, 包括指针、长度和容量
    let s2 = s1; // s2 拷贝了s1的指针、长度和容量,但是底层的堆上的数据没有拷贝,
    // 造成一个值对应了两个所有者, 因此, Rust 规定所有权转移, 这里 s1 不再拥有底层堆数据的所有权,而是转移给了s2
    // rust 称之为移动 move, s1 被移动到了 s2, s1 就无效了

    // println!("{}, world!", s1); // 这里会报错, 因为 s1 已经无效了
    println!("{}, world!", s2);
    
    // 字符串字面量（栈上数据）
    let x: &str = "hello, world";
    let y = x; // 所以 y 和 x 不存在所有权转移的问题, 只是将 x 的引用拷贝
    println!("{},{}", x, y);
    
    // 深拷贝后, 不再有所有权转移
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 拷贝整个堆内存上的数据, 也就是深拷贝
    println!("s1 = {}, s2 = {}", s1, s2);
}

// 引用和借用演示
fn reference_borrowing_demo() {
    println!("=== 引用和借用演示 ===");
    let s = String::from("hello");
    not_change(&s); // 不可变引用
    println!("out s = {}", s);
    
    let mut s = String::from("hello");
    change(&mut s); // 传递可变引用
    println!("s = {}", s);
}

fn not_change(s: &String) {
    println!("in s = {}", s);
}

// 参数必须是可变的引用, 否则报错
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 复杂数据处理演示
fn complex_data_processing_demo() {
    println!("=== 复杂数据处理演示 ===");
    let penguin_data = "\
   common name,length (cm)
   Little penguin,33
   Yellow-eyed penguin,65
   Fiordland penguin,60
   Invalid,data
   ";
    // 每一行数据的迭代器
    let records = penguin_data.lines();
    for (i, r) in records.enumerate() {
        if i == 0 || r.trim().len() == 0 {
            continue;
        }
        // 声明一个 fields 变量，类型是 Vec
        // Vec 是 vector 的缩写，是一个可伸缩的集合类型，可以认为是一个动态数组
        // <_>表示 Vec 中的元素类型由编译器自行推断，在很多场景下，都会帮我们省却不少功夫
        let fields: Vec<_> = r.split(',').map(|field| field.trim()).collect();
        if cfg!(debug_assertions) {
            // 输出到标准错误输出
            eprintln!("debug: {:?} -> {:?}", r, fields);
        }

        let name = fields[0];
        // 1. 尝试把 fields[1] 的值转换为 f32 类型的浮点数，如果成功，则把 f32 值赋给 length 变量
        //
        // 2. if let 是一个匹配表达式，用来从=右边的结果中，匹配出 length 的值：
        //   1）当=右边的表达式执行成功，则会返回一个 Ok(f32) 的类型，若失败，则会返回一个 Err(e) 类型，if let 的作用就是仅匹配 Ok 也就是成功的情况，如果是错误，就直接忽略
        //   2）同时 if let 还会做一次解构匹配，通过 Ok(length) 去匹配右边的 Ok(f32)，最终把相应的 f32 值赋给 length
        //
        // 3. 当然你也可以忽略成功的情况，用 if let Err(e) = fields[1].parse::<f32>() {...}匹配出错误，然后打印出来，但是没啥卵用
        if let Ok(length) = fields[1].parse::<f32>() {
            // 输出到标准输出
            println!("{}, {}cm", name, length);
        }
    }
}

// 多语言问候演示
fn multilingual_greeting_demo() {
    println!("=== 多语言问候演示 ===");
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";
    // 创建一个集合
    let regions = [southern_germany, chinese, english];
    // 集合不能直接迭代，需要变为迭代器(.iter方法)
    // for region in regions.iter() {
    // 2021版本之后可以简写，编译器自动隐式转换
    for region in regions {
        // ! 为宏操作符, {} 为占位符
        println!("{}", &region);
    }
}

// 主演示函数
pub fn hello_world_demo() {
    println!("🌍 Hello World 综合演示");
    println!("================================");
    
    basic_hello_demo();
    println!();
    
    statement_expression_demo();
    println!();
    
    string_operations_demo();
    println!();
    
    ownership_demo();
    println!();
    
    reference_borrowing_demo();
    println!();
    
    complex_data_processing_demo();
    println!();
    
    multilingual_greeting_demo();
    println!();
    
    println!("💡 总结：");
    println!("  - 语句以分号结尾，表达式没有分号");
    println!("  - 所有权规则：每个值只有一个所有者");
    println!("  - 借用规则：可以有多个不可变借用或一个可变借用");
    println!("  - 移动语义：堆数据的所有权转移");
    println!("  - 克隆：深拷贝堆数据");
}
