// 03. 函数

pub fn run_demo() {
    println!("🔧 函数演示");
    println!("===========");
    
    // 基本函数
    basic_functions();
    println!();
    
    // 参数和返回值
    parameters_and_return_values();
    println!();
    
    // 表达式和语句
    expressions_and_statements();
    println!();
    
    // 函数作为值
    functions_as_values();
    println!();
    
    // 高阶函数
    higher_order_functions();
    println!();
    
    // 递归函数
    recursive_functions();
}

fn basic_functions() {
    println!("=== 基本函数 ===");
    
    // 无参数无返回值
    say_hello();
    
    // 有参数有返回值
    let result = add(5, 3);
    println!("5 + 3 = {}", result);
    
    // 多个参数
    let result = multiply(4, 7);
    println!("4 * 7 = {}", result);
}

fn say_hello() {
    println!("Hello from a function!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

fn parameters_and_return_values() {
    println!("=== 参数和返回值 ===");
    
    // 不同类型的参数
    let result1 = calculate_area(5.0, 3.0);
    println!("矩形面积: {}", result1);
    
    let result2 = greet_person("Alice");
    println!("{}", result2);
    
    // 返回元组
    let (min, max) = find_min_max(&[1, 5, 3, 9, 2]);
    println!("最小值: {}, 最大值: {}", min, max);
    
    // 返回 Option
    let result = divide(10, 2);
    match result {
        Some(value) => println!("10 / 2 = {}", value),
        None => println!("除零错误"),
    }
}

fn calculate_area(width: f64, height: f64) -> f64 {
    width * height
}

fn greet_person(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn find_min_max(arr: &[i32]) -> (i32, i32) {
    let mut min = arr[0];
    let mut max = arr[0];
    
    for &item in arr.iter() {
        if item < min {
            min = item;
        }
        if item > max {
            max = item;
        }
    }
    
    (min, max)
}

fn divide(a: i32, b: i32) -> Option<f64> {
    if b == 0 {
        None
    } else {
        Some(a as f64 / b as f64)
    }
}

fn expressions_and_statements() {
    println!("=== 表达式和语句 ===");
    
    // 表达式（有返回值）
    let x = {
        let a = 1;
        let b = 2;
        a + b // 这是表达式，没有分号
    };
    println!("表达式的结果: {}", x);
    
    // 语句（无返回值）
    let y = {
        let a = 1;
        let b = 2;
        a + b; // 这是语句，有分号，返回 ()
    };
    println!("语句的结果: {:?}", y);
    
    // if 表达式
    let number = 6;
    let description = if number % 2 == 0 {
        "偶数"
    } else {
        "奇数"
    };
    println!("{} 是 {}", number, description);
}

fn functions_as_values() {
    println!("=== 函数作为值 ===");
    
    // 函数指针
    let func_ptr: fn(i32, i32) -> i32 = add;
    let result = func_ptr(3, 4);
    println!("通过函数指针调用: {}", result);
    
    // 函数数组
    let operations = [add, subtract, multiply];
    let a = 10;
    let b = 5;
    
    for (i, op) in operations.iter().enumerate() {
        let result = op(a, b);
        let op_name = match i {
            0 => "加法",
            1 => "减法", 
            2 => "乘法",
            _ => "未知",
        };
        println!("{}: {} {} {} = {}", op_name, a, 
                 match i { 0 => "+", 1 => "-", 2 => "*", _ => "?" }, b, result);
    }
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn higher_order_functions() {
    println!("=== 高阶函数 ===");
    
    let numbers = [1, 2, 3, 4, 5];
    
    // 使用闭包
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("原数组: {:?}", numbers);
    println!("翻倍后: {:?}", doubled);
    
    // 过滤
    let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).cloned().collect();
    println!("偶数: {:?}", evens);
    
    // 折叠
    let sum: i32 = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("求和: {}", sum);
}

fn recursive_functions() {
    println!("=== 递归函数 ===");
    
    // 阶乘
    let n = 5;
    let factorial = calculate_factorial(n);
    println!("{}! = {}", n, factorial);
    
    // 斐波那契数列
    for i in 0..10 {
        let fib = fibonacci(i);
        print!("{} ", fib);
    }
    println!();
    
    // 二分查找
    let arr = [1, 3, 5, 7, 9, 11, 13, 15];
    let target = 7;
    let result = binary_search(&arr, target, 0, arr.len() - 1);
    match result {
        Some(index) => println!("在索引 {} 找到 {}", index, target),
        None => println!("未找到 {}", target),
    }
}

fn calculate_factorial(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        n * calculate_factorial(n - 1)
    }
}

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn binary_search(arr: &[i32], target: i32, left: usize, right: usize) -> Option<usize> {
    if left > right {
        return None;
    }
    
    let mid = (left + right) / 2;
    
    if arr[mid] == target {
        Some(mid)
    } else if arr[mid] > target {
        if mid == 0 { None } else { binary_search(arr, target, left, mid - 1) }
    } else {
        binary_search(arr, target, mid + 1, right)
    }
}