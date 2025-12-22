use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("=== 猜数字游戏 (Guessing Game) ===");
    println!("Can you guess the number between 1 and 100?");

    // 1. 生成随机数
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {}", secret_number); // Debug 用

    loop {
        println!("Please input your guess.");

        // 2. 获取用户输入
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 3. 处理输入 (String -> u32)
        // 使用 match 处理 parse 结果，防止非数字输入导致崩溃
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            },
        };

        println!("You guessed: {}", guess);

        // 4. 比较猜测结果
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // 猜对了退出循环
            }
        }
    }
}
