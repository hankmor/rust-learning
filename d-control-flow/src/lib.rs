// 04. 控制流

pub fn run_demo() {
    println!("🔄 控制流演示");
    println!("=============");
    
    // if 表达式
    if_expressions();
    println!();
    
    // 循环
    loops();
    println!();
    
    // match 表达式
    match_expressions();
    println!();
    
    // if let 和 while let
    if_let_and_while_let();
    println!();
    
    // 模式匹配
    pattern_matching();
    println!();
    
    // 守卫
    match_guards();
}

fn if_expressions() {
    println!("=== if 表达式 ===");
    
    let number = 6;
    
    // 基本 if
    if number % 4 == 0 {
        println!("number 能被 4 整除");
    } else if number % 3 == 0 {
        println!("number 能被 3 整除");
    } else if number % 2 == 0 {
        println!("number 能被 2 整除");
    } else {
        println!("number 不能被 4、3 或 2 整除");
    }
    
    // if 作为表达式
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number 的值是: {}", number);
    
    // 嵌套 if
    let score = 85;
    let grade = if score >= 90 {
        "A"
    } else if score >= 80 {
        "B"
    } else if score >= 70 {
        "C"
    } else if score >= 60 {
        "D"
    } else {
        "F"
    };
    println!("分数 {} 对应等级: {}", score, grade);
}

fn loops() {
    println!("=== 循环 ===");
    
    // loop 循环
    println!("loop 循环示例:");
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            println!("  计数到 3，继续");
            continue;
        }
        println!("  计数: {}", count);
        if count == 5 {
            break;
        }
    }
    
    // while 循环
    println!("while 循环示例:");
    let mut number = 3;
    while number != 0 {
        println!("  {}", number);
        number -= 1;
    }
    println!("  发射!");
    
    // for 循环
    println!("for 循环示例:");
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("  值是: {}", element);
    }
    
    // for 循环与范围
    println!("for 循环范围示例:");
    for number in 1..4 {
        println!("  {}", number);
    }
    
    // for 循环与 enumerate
    println!("for 循环 enumerate 示例:");
    for (index, value) in a.iter().enumerate() {
        println!("  a[{}] = {}", index, value);
    }
}

fn match_expressions() {
    println!("=== match 表达式 ===");
    
    // 基本 match
    let number = 13;
    match number {
        1 => println!("一"),
        2 => println!("二"),
        3 => println!("三"),
        4 | 5 | 6 | 7 | 8 | 9 => println!("四到九"),
        10..=19 => println!("十到十九"),
        _ => println!("其他"),
    }
    
    // match 与 Option
    let some_number = Some(7);
    match some_number {
        Some(7) => println!("幸运数字!"),
        Some(x) => println!("数字: {}", x),
        None => println!("没有数字"),
    }
    
    // match 与 Result
    let result: Result<i32, &str> = Ok(42);
    match result {
        Ok(value) => println!("成功: {}", value),
        Err(error) => println!("错误: {}", error),
    }
}

fn if_let_and_while_let() {
    println!("=== if let 和 while let ===");
    
    // if let
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    
    if let Some(color) = favorite_color {
        println!("使用你最喜欢的颜色 {} 作为背景", color);
    } else if is_tuesday {
        println!("星期二是绿色日!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("使用紫色作为背景颜色");
        } else {
            println!("使用橙色作为背景颜色");
        }
    } else {
        println!("使用蓝色作为背景颜色");
    }
    
    // while let
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    println!("while let 示例:");
    while let Some(top) = stack.pop() {
        println!("  {}", top);
    }
}

fn pattern_matching() {
    println!("=== 模式匹配 ===");
    
    // 解构元组
    let point = (3, 5);
    match point {
        (0, 0) => println!("原点"),
        (0, y) => println!("在 y 轴上，y = {}", y),
        (x, 0) => println!("在 x 轴上，x = {}", x),
        (x, y) => println!("在 ({}, {})", x, y),
    }
    
    // 解构结构体
    struct Point {
        x: i32,
        y: i32,
    }
    
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("在 x 轴上，x = {}", x),
        Point { x: 0, y } => println!("在 y 轴上，y = {}", y),
        Point { x, y } => println!("在 ({}, {})", x, y),
    }
    
    // 解构枚举
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => println!("退出程序"),
        Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
        Message::Write(text) => println!("文本消息: {}", text),
        Message::ChangeColor(r, g, b) => println!("改变颜色为 RGB({}, {}, {})", r, g, b),
    }
}

fn match_guards() {
    println!("=== match 守卫 ===");
    
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("小于 5: {}", x),
        Some(x) if x >= 5 => println!("大于等于 5: {}", x),
        Some(x) => println!("其他: {}", x),
        None => println!("没有值"),
    }
    
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
    
    // 复杂守卫
    let point = (4, 5);
    match point {
        (x, y) if x == y => println!("在对角线上"),
        (x, y) if x + y == 0 => println!("在反对角线上"),
        (x, y) if x > y => println!("x 大于 y"),
        (x, y) if x < y => println!("x 小于 y"),
        _ => println!("其他情况"),
    }
}