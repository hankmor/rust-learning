fn main() {
    println!("=== 变量与可变性练习 ===");

    // 1. 可变性 (Mutability)
    // let x = 1;
    // x = 2; // ❌ 编译错误：cannot assign twice to immutable variable `x`
    let mut x = 1;
    println!("(Mutable) x initial: {}", x);
    x = 2;
    println!("(Mutable) x updated: {}", x);

    // 2. 变量遮蔽 (Shadowing)
    println!("\n=== Shadowing 演示 ===");
    let spaces = "   "; // &str 类型
    println!("spaces (original): '{}'", spaces);

    // 💡 这里的 spaces 变成了一个全新的变量（usize类型），只是名字相同
    let spaces = spaces.len();
    println!("spaces (shadowed): {}", spaces);

    // 3. 常量 (Constants)
    const MAX_POINTS: u32 = 100_000;
    println!("\nConstant MAX_POINTS: {}", MAX_POINTS);

    // ==========================================
    // 🧠 思考题解答
    // ==========================================

    // 1. 为什么 Rust 允许 Shadowing 而 Go 不允许（或容易出错）？
    // 答：
    // - Rust: Shadowing 是为了方便**类型转换**。比如把输入的 "42"(str) 转为 42(int)。
    //   如果我们不能 Shadowing，就得起名叫 `input_str` 和 `input_int`，非常啰嗦。
    //   Rust 的强类型系统保证了这种转换是显式的，且旧变量在 Shadowing 后不可访问（除非在不同作用域），减少了误用风险。
    // - Go: Go 的 `:=` 语法在嵌套作用域中容易造成"意外的 Shadowing"，即开发者以为在给外部变量赋值，
    //   其实是声明了一个新的内部变量。这在错误处理 (`err`) 中特别常见。Rust 赋值必须用 `=`, 声明必须用 `let`，区分得更清楚。

    // 2. 在什么场景下 Shadowing 可能会导致混淆？
    // 答：
    // - 当作用域非常**长**时：如果在函数开头 Shadowing 了一次，几百行后你可能忘了这个变量的类型已经变了。
    // - 在**嵌套过深**的作用域中：如果在 `if/while` 内部 Shadowing 了外部变量，可能会导致意外的行为（虽然这在任何语言都存在）。
    // 最佳实践是：仅在进行类型转换或短生命周期的临时处理时使用 Shadowing。
}
