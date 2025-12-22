use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    println!("=== 练习题运行 ===");

    // 💡 练习 1：打印名字
    println!("Hello, Hank!");

    // ✨ 进阶：使用 Ferris (Rust 的吉祥物) 说话
    let stdout = stdout();
    let message = String::from("Hello, Hank! I am Ferris.");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

    // 🧠 思考题回答：
    // 为什么 println! 是宏？
    // 1.因为 Rust 的函数不支持变参 (Variadic Arguments)，而宏可以接受任意数量的参数。
    // 2.宏在编译期展开，可以进行格式说明符 (如 {}) 的静态检查。
}
