// 09. 错误处理

use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;

pub fn run_demo() {
    println!("⚠️ 错误处理演示");
    println!("===============");
    
    // panic! 宏
    panic_demo();
    println!();
    
    // Result 类型
    result_type();
    println!();
    
    // 匹配不同的错误
    matching_different_errors();
    println!();
    
    // 错误传播
    error_propagation();
    println!();
    
    // 简化的错误处理
    simplified_error_handling();
    println!();
    
    // 自定义错误类型
    custom_error_types();
    println!();
    
    // 错误处理最佳实践
    error_handling_best_practices();
}

fn panic_demo() {
    println!("=== panic! 宏 ===");
    
    // 基本 panic
    // panic!("crash and burn");
    
    // 数组越界 panic
    let v = vec![1, 2, 3];
    // v[99]; // 这会 panic
    
    println!("panic 演示完成（实际 panic 被注释掉了）");
}

fn result_type() {
    println!("=== Result 类型 ===");
    
    // 基本 Result 使用
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => {
            println!("文件打开成功");
            file
        },
        Err(error) => {
            println!("文件打开失败: {:?}", error);
            // 这里应该处理错误，但为了演示我们 panic
            panic!("无法打开文件: {:?}", error);
        },
    };
    
    // 简化的错误处理
    let f2 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("创建文件失败: {:?}", error);
            })
        } else {
            panic!("打开文件失败: {:?}", error);
        }
    });
    
    println!("Result 类型演示完成");
}

fn matching_different_errors() {
    println!("=== 匹配不同的错误 ===");
    
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => {
                    println!("文件不存在，已创建新文件");
                    fc
                },
                Err(e) => panic!("创建文件失败: {:?}", e),
            },
            other_error => {
                panic!("打开文件失败: {:?}", other_error);
            }
        },
    };
    
    println!("错误匹配演示完成");
}

fn error_propagation() {
    println!("=== 错误传播 ===");
    
    // 使用 ? 操作符传播错误
    let result = read_username_from_file("hello.txt");
    match result {
        Ok(username) => println!("用户名: {}", username),
        Err(e) => println!("读取用户名失败: {:?}", e),
    }
    
    // 链式调用
    let result2 = read_username_from_file_chain("hello.txt");
    match result2 {
        Ok(username) => println!("用户名（链式）: {}", username),
        Err(e) => println!("读取用户名失败（链式）: {:?}", e),
    }
}

fn read_username_from_file(filename: &str) -> Result<String, std::io::Error> {
    let mut f = File::open(filename)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_chain(filename: &str) -> Result<String, std::io::Error> {
    let mut s = String::new();
    File::open(filename)?.read_to_string(&mut s)?;
    Ok(s)
}

fn simplified_error_handling() {
    println!("=== 简化的错误处理 ===");
    
    // unwrap 和 expect
    let f = File::open("hello.txt").unwrap(); // 成功时返回值，失败时 panic
    println!("文件打开成功（unwrap）");
    
    // expect 提供自定义错误信息
    let f2 = File::open("hello.txt").expect("无法打开 hello.txt 文件");
    println!("文件打开成功（expect）");
    
    // unwrap_or 提供默认值
    let f3 = File::open("nonexistent.txt").unwrap_or_else(|_| {
        File::create("nonexistent.txt").unwrap()
    });
    println!("文件处理成功（unwrap_or_else）");
    
    // 使用 ? 操作符
    let result = read_username_from_file("hello.txt");
    match result {
        Ok(username) => println!("用户名: {}", username),
        Err(e) => println!("错误: {:?}", e),
    }
}

fn custom_error_types() {
    println!("=== 自定义错误类型 ===");
    
    // 定义自定义错误类型
    #[derive(Debug)]
    enum CustomError {
        DivisionByZero,
        NegativeNumber,
        Overflow,
    }
    
    impl std::fmt::Display for CustomError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                CustomError::DivisionByZero => write!(f, "除零错误"),
                CustomError::NegativeNumber => write!(f, "负数错误"),
                CustomError::Overflow => write!(f, "溢出错误"),
            }
        }
    }
    
    impl std::error::Error for CustomError {}
    
    // 使用自定义错误类型
    fn divide(a: i32, b: i32) -> Result<i32, CustomError> {
        if b == 0 {
            Err(CustomError::DivisionByZero)
        } else if a < 0 {
            Err(CustomError::NegativeNumber)
        } else {
            Ok(a / b)
        }
    }
    
    // 测试自定义错误
    match divide(10, 2) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("错误: {}", e),
    }
    
    match divide(10, 0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("错误: {}", e),
    }
    
    match divide(-10, 2) {
        Ok(result) => println!("-10 / 2 = {}", result),
        Err(e) => println!("错误: {}", e),
    }
}

fn error_handling_best_practices() {
    println!("=== 错误处理最佳实践 ===");
    
    // 1. 使用 Result 而不是 panic
    fn safe_divide(a: i32, b: i32) -> Result<f64, String> {
        if b == 0 {
            Err("除零错误".to_string())
        } else {
            Ok(a as f64 / b as f64)
        }
    }
    
    // 2. 提供有意义的错误信息
    match safe_divide(10, 0) {
        Ok(result) => println!("结果: {}", result),
        Err(e) => println!("错误: {}", e),
    }
    
    // 3. 使用 ? 操作符简化错误传播
    fn process_file(filename: &str) -> Result<String, std::io::Error> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
    
    // 4. 使用 map_err 转换错误类型
    fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
        s.parse::<i32>()
    }
    
    let result = parse_number("42").map_err(|e| format!("解析错误: {}", e));
    match result {
        Ok(n) => println!("解析成功: {}", n),
        Err(e) => println!("解析失败: {}", e),
    }
    
    // 5. 使用 unwrap_or 提供默认值
    let number = "not_a_number".parse::<i32>().unwrap_or(0);
    println!("解析结果（默认值）: {}", number);
    
    // 6. 使用 unwrap_or_else 提供计算默认值
    let number2 = "not_a_number".parse::<i32>().unwrap_or_else(|_| {
        println!("解析失败，使用默认值");
        42
    });
    println!("解析结果（计算默认值）: {}", number2);
}
