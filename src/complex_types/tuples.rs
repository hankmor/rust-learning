// 元组演示模块

// 基本元组演示
pub fn basic_tuple_demo() {
    println!("=== 基本元组演示 ===");
    
    // 创建元组
    let tup1 = (500, 6.4, 1);
    let tup2: (i32, f64, u8) = (500, 6.4, 1); // 显式类型声明
    
    println!("元组 tup1: {:?}", tup1);
    println!("元组 tup2: {:?}", tup2);
    
    // 解构元组
    let (x, y, z) = tup1;
    println!("解构: x={}, y={}, z={}", x, y, z);
    
    // 通过索引访问元组元素
    println!("tup1.0 = {}", tup1.0);
    println!("tup1.1 = {}", tup1.1);
    println!("tup1.2 = {}", tup1.2);
}

// 元组作为函数参数和返回值
pub fn tuple_functions_demo() {
    println!("=== 元组函数演示 ===");
    
    let point1 = (3, 4);
    let point2 = (1, 2);
    
    let distance = calculate_distance(point1, point2);
    println!("点 {:?} 和点 {:?} 的距离: {}", point1, point2, distance);
    
    let (min, max) = get_min_max(&[1, 5, 3, 9, 2]);
    println!("数组 [1, 5, 3, 9, 2] 的最小值: {}, 最大值: {}", min, max);
}

// 计算两点之间的距离
fn calculate_distance(p1: (i32, i32), p2: (i32, i32)) -> f64 {
    let dx = (p1.0 - p2.0) as f64;
    let dy = (p1.1 - p2.1) as f64;
    (dx * dx + dy * dy).sqrt()
}

// 返回数组的最小值和最大值
fn get_min_max(arr: &[i32]) -> (i32, i32) {
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

// 嵌套元组演示
pub fn nested_tuple_demo() {
    println!("=== 嵌套元组演示 ===");
    
    let nested = ((1, 2), (3, 4), (5, 6));
    println!("嵌套元组: {:?}", nested);
    
    // 访问嵌套元组
    println!("nested.0 = {:?}", nested.0);
    println!("nested.0.0 = {}", nested.0.0);
    println!("nested.1.1 = {}", nested.1.1);
    
    // 解构嵌套元组
    let ((a, b), (c, d), (e, f)) = nested;
    println!("解构嵌套元组: a={}, b={}, c={}, d={}, e={}, f={}", a, b, c, d, e, f);
}

// 单元元组演示
pub fn unit_tuple_demo() {
    println!("=== 单元元组演示 ===");
    
    let unit = ();
    println!("单元元组: {:?}", unit);
    
    // 函数返回单元元组
    let result = print_and_return_unit();
    println!("函数返回的单元元组: {:?}", result);
    
    // 单元元组在模式匹配中的使用
    match some_function() {
        Ok(()) => println!("函数执行成功"),
        Err(()) => println!("函数执行失败"),
    }
}

fn print_and_return_unit() -> () {
    println!("这个函数返回单元元组");
    () // 显式返回单元元组
}

fn some_function() -> Result<(), ()> {
    // 模拟可能成功或失败的操作
    Ok(())
}

// 主演示函数
pub fn tuples_demo() {
    println!("🔗 元组演示");
    println!("================================");
    
    basic_tuple_demo();
    println!();
    
    tuple_functions_demo();
    println!();
    
    nested_tuple_demo();
    println!();
    
    unit_tuple_demo();
    println!();
    
    println!("💡 总结：");
    println!("  - 元组：固定长度的异构集合");
    println!("  - 可以通过索引或解构访问元素");
    println!("  - 常用作函数的多返回值");
    println!("  - 单元元组 () 表示空值");
}
