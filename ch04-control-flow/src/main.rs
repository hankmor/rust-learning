fn main() {
    println!("=== 控制流程 Demo ===");

    // 1. if 表达式 (if as an expression)
    let condition = true;
    let number = if condition { 5 } else { 6 }; // 类似 Go 的三元表达式
    println!("The value of number is: {}", number);

    // 2. loop 循环与返回值
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // break 可以带返回值！
        }
    };
    println!(" The result is: {}", result);

    // 3. while 循环
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // 4. for 循环 (最常用)
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }

    // 5. Range (范围)
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
