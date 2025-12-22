fn main() {
    println!("=== 函数基础 Demo ===");

    // 1. 调用带参数的函数
    print_labeled_measurement(5, "h");

    // 2. 表达式 vs 语句
    let y = {
        let x = 3;
        x + 1 // 这是一个表达式，没有分号，返回 4
    };
    println!("The value of y is: {}", y);

    // 3. 函数返回值
    let x = five();
    println!("The value of x is: {}", x);

    let z = plus_one(5);
    println!("The value of 5 + 1 is: {}", z);

    // 4. 发散函数 (Diverging functions) - 演示
    // diverging_function(); 
}

fn print_labeled_measurement(value: i32, unit_label: &str) {
    println!("The measurement is: {}{}", value, unit_label);
}

// -> i32 表示返回类型
fn five() -> i32 {
    5 // 隐式返回，等同于 return 5;
}

fn plus_one(x: i32) -> i32 {
    x + 1 // 注意：这里不能加分号，否则变成语句，返回 unit type ()
}


/*
fn diverging_function() -> ! {
    panic!("This function never returns!");
}
*/
