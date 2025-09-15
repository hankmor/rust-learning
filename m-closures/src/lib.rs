// 13. 闭包

use std::thread;
use std::time::Duration;

// 基本闭包
fn basic_closures() {
    let add_one = |x| x + 1;
    println!("5 + 1 = {}", add_one(5));
    
    let add = |x, y| x + y;
    println!("3 + 4 = {}", add(3, 4));
    
    let multiply = |x: i32, y: i32| -> i32 { x * y };
    println!("6 * 7 = {}", multiply(6, 7));
}

// 闭包捕获环境
fn closure_captures_environment() {
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    
    assert!(equal_to_x(y));
    println!("闭包捕获环境变量 x = {}", x);
}

// 闭包类型推断
fn closure_type_inference() {
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    let n = example_closure(5.to_string());
    
    println!("字符串: {}", s);
    println!("数字: {}", n);
}

// 闭包捕获方式
fn closure_capture_modes() {
    let x = vec![1, 2, 3];
    
    // 不可变借用
    let equal_to_x = |z| z == x;
    println!("x 仍然可用: {:?}", x);
    
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
    
    // 可变借用
    let mut x = vec![1, 2, 3];
    let mut equal_to_x = |z| {
        x.push(4);
        z == x
    };
    
    let y = vec![1, 2, 3, 4];
    assert!(equal_to_x(y));
    
    // 移动所有权
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    // println!("x 不再可用: {:?}", x); // 编译错误！
    
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

// 闭包作为参数
fn closure_as_parameter() {
    let list_of_numbers = vec![1, 2, 3, 4, 5];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string())
        .collect();
    
    println!("数字列表: {:?}", list_of_numbers);
    println!("字符串列表: {:?}", list_of_strings);
}

// 闭包作为返回值
fn closure_as_return_value() {
    fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
        move |y| x + y
    }
    
    let add_5 = make_adder(5);
    println!("5 + 3 = {}", add_5(3));
    
    let add_10 = make_adder(10);
    println!("10 + 7 = {}", add_10(7));
}

// 闭包与迭代器
fn closures_with_iterators() {
    let v1 = vec![1, 2, 3, 4, 5];
    
    // map
    let v2: Vec<i32> = v1.iter().map(|x| x * 2).collect();
    println!("翻倍: {:?}", v2);
    
    // filter
    let v3: Vec<&i32> = v1.iter().filter(|&&x| x % 2 == 0).collect();
    println!("偶数: {:?}", v3);
    
    // fold
    let sum: i32 = v1.iter().fold(0, |acc, &x| acc + x);
    println!("求和: {}", sum);
    
    // any
    let has_even = v1.iter().any(|&x| x % 2 == 0);
    println!("有偶数: {}", has_even);
    
    // all
    let all_positive = v1.iter().all(|&x| x > 0);
    println!("都为正数: {}", all_positive);
}

// 闭包与线程
fn closures_with_threads() {
    let list = vec![1, 2, 3, 4, 5];
    let handle = thread::spawn(move || {
        println!("线程中的列表: {:?}", list);
        list.iter().sum::<i32>()
    });
    
    let result = handle.join().unwrap();
    println!("线程计算结果: {}", result);
}

// 闭包与异步
async fn closures_with_async() {
    let x = 5;
    let async_closure = async move |y| x + y;
    let result = async_closure(3).await;
    println!("异步闭包结果: {}", result);
}

// 闭包与错误处理
fn closures_with_error_handling() {
    let numbers = vec!["1", "2", "3", "not_a_number", "5"];
    
    let results: Vec<Result<i32, _>> = numbers
        .iter()
        .map(|s| s.parse::<i32>())
        .collect();
    
    println!("解析结果: {:?}", results);
    
    // 使用 filter_map 过滤错误
    let valid_numbers: Vec<i32> = numbers
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    
    println!("有效数字: {:?}", valid_numbers);
}

// 闭包与智能指针
fn closures_with_smart_pointers() {
    use std::rc::Rc;
    use std::cell::RefCell;
    
    let data = Rc::new(RefCell::new(5));
    let data2 = data.clone();
    
    let closure = move || {
        let mut value = data2.borrow_mut();
        *value += 1;
        *value
    };
    
    let result = closure();
    println!("闭包结果: {}", result);
    println!("原始数据: {}", data.borrow());
}

// 闭包与特征对象
fn closures_with_trait_objects() {
    let closures: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(|x| x * 2),
        Box::new(|x| x + 1),
        Box::new(|x| x * x),
    ];
    
    for (i, closure) in closures.iter().enumerate() {
        println!("闭包 {}: {}", i, closure(5));
    }
}

// 闭包与生命周期
fn closures_with_lifetimes() {
    let s = String::from("hello");
    let closure = |x: &str| x.len() > s.len();
    
    let result = closure("world");
    println!("字符串比较结果: {}", result);
    println!("原始字符串: {}", s);
}

// 闭包与泛型
fn closures_with_generics() {
    fn apply_operation<F, T>(f: F, value: T) -> T
    where
        F: Fn(T) -> T,
    {
        f(value)
    }
    
    let double = |x: i32| x * 2;
    let square = |x: f64| x * x;
    
    println!("双倍: {}", apply_operation(double, 5));
    println!("平方: {}", apply_operation(square, 3.0));
}

// 闭包与模式匹配
fn closures_with_pattern_matching() {
    let pairs = vec![(1, 2), (3, 4), (5, 6)];
    
    let sums: Vec<i32> = pairs
        .iter()
        .map(|(x, y)| x + y)
        .collect();
    
    println!("对的和: {:?}", sums);
    
    let evens: Vec<&(i32, i32)> = pairs
        .iter()
        .filter(|(x, y)| (x + y) % 2 == 0)
        .collect();
    
    println!("和为偶数的对: {:?}", evens);
}

// 闭包与递归
fn closures_with_recursion() {
    fn factorial(n: u32) -> u32 {
        if n <= 1 {
            1
        } else {
            n * factorial(n - 1)
        }
    }
    
    let factorial_closure = |n: u32| factorial(n);
    
    println!("5! = {}", factorial_closure(5));
}

// 闭包与缓存
fn closures_with_caching() {
    use std::collections::HashMap;
    
    let mut cache = HashMap::new();
    let mut expensive_closure = |x: i32| -> i32 {
        if let Some(&result) = cache.get(&x) {
            result
        } else {
            let result = x * x; // 模拟昂贵计算
            cache.insert(x, result);
            result
        }
    };
    
    println!("第一次计算 5^2: {}", expensive_closure(5));
    println!("第二次计算 5^2: {}", expensive_closure(5));
    println!("计算 3^2: {}", expensive_closure(3));
}

pub fn run_demo() {
    println!("🔒 闭包演示");
    println!("===========");
    
    // 基本闭包
    basic_closures();
    println!();
    
    // 闭包捕获环境
    closure_captures_environment();
    println!();
    
    // 闭包类型推断
    closure_type_inference();
    println!();
    
    // 闭包捕获方式
    closure_capture_modes();
    println!();
    
    // 闭包作为参数
    closure_as_parameter();
    println!();
    
    // 闭包作为返回值
    closure_as_return_value();
    println!();
    
    // 闭包与迭代器
    closures_with_iterators();
    println!();
    
    // 闭包与线程
    closures_with_threads();
    println!();
    
    // 闭包与错误处理
    closures_with_error_handling();
    println!();
    
    // 闭包与智能指针
    closures_with_smart_pointers();
    println!();
    
    // 闭包与特征对象
    closures_with_trait_objects();
    println!();
    
    // 闭包与生命周期
    closures_with_lifetimes();
    println!();
    
    // 闭包与泛型
    closures_with_generics();
    println!();
    
    // 闭包与模式匹配
    closures_with_pattern_matching();
    println!();
    
    // 闭包与递归
    closures_with_recursion();
    println!();
    
    // 闭包与缓存
    closures_with_caching();
}
