fn main() {
    println!("=== 变量与可变性 demo ===");

    // 1. 不可变变量 (Immutable by default)
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // 编译错误：cannot assign twice to immutable variable `x`

    // 2. 可变变量 (Mutable variables)
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is now: {}", y);

    // 3. 常量 (Constants)
    const MAX_POINTS: u32 = 100_000;
    println!("Constant MAX_POINTS: {}", MAX_POINTS);

    // 4. Shadowing (变量遮蔽)
    let z = 5;
    let z = z + 1; // z is now 6
    {
        let z = z * 2; // z is now 12 in this scope
        println!("The value of z in the inner scope is: {}", z);
    }
    println!("The value of z is: {}", z); // z is 6

    // 5. 简单类型演示
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Parsed guess: {}", guess);
}
