// 01. Hello World 库

pub fn run_demo() {
    println!("🌍 Hello World 演示");
    println!("==================");
    
    // 基本 Hello World
    basic_hello_world();
    println!();
    
    // 多语言问候
    multilingual_greeting();
    println!();
    
    // 复杂数据处理
    complex_data_processing();
    println!();
    
    // Ferris 说话
    ferris_says_hello();
    println!();
    
    // 语句和表达式
    statements_and_expressions();
    println!();
    
    // 函数基础
    function_basics();
}

fn basic_hello_world() {
    println!("=== 基本 Hello World ===");
    println!("Hello, world!");
    
    // 多语言问候
    let chinese = "你好,世界!";
    let english = "Hello World!";
    let langs = [chinese, english];
    for lang in langs {
        println!("{}", &lang);
    }
}

fn multilingual_greeting() {
    println!("=== 多语言问候 ===");
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";
    let regions = [southern_germany, chinese, english];
    
    for region in regions {
        println!("{}", &region);
    }
}

fn complex_data_processing() {
    println!("=== 复杂数据处理 ===");
    let penguin_data = "\
   common name,length (cm)
   Little penguin,33
   Yellow-eyed penguin,65
   Fiordland penguin,60
   Invalid,data
   ";
    
    let records = penguin_data.lines();
    for (i, r) in records.enumerate() {
        if i == 0 || r.trim().len() == 0 {
            continue;
        }
        
        let fields: Vec<_> = r.split(',').map(|field| field.trim()).collect();
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", r, fields);
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }
}

fn ferris_says_hello() {
    println!("=== Ferris 说话 ===");
    use ferris_says::say;
    use std::io::{stdout, BufWriter};
    
    let stdout = stdout();
    let message = String::from("Hello fellow Rustacean!");
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

fn statements_and_expressions() {
    println!("=== 语句和表达式 ===");
    let x = {
        let a = 1; // 语句
        a + 1 // 表达式
    };
    println!("x = {}", x);
    
    // 单元类型演示
    let r = return_unit();
    assert_eq!(r, ());
    println!("r == (): {}", r == ());
}

fn return_unit() {
    let a = 2;
    let _b = if a % 2 == 1 { "奇数" } else { "偶数" };
}

fn function_basics() {
    println!("=== 函数基础 ===");
    let result = add(5, 3);
    println!("5 + 3 = {}", result);
    
    let greeting = create_greeting("Rust");
    println!("{}", greeting);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn create_greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}
