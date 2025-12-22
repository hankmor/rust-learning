fn main() {
    println!("=== 枚举与模式匹配 (Enums & Match) Demo ===");

    // 1. 定义和使用枚举
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("IP Type: {:?}, {:?}", four, six);

    // 2. 带数据的枚举
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("Home IP: {:?}", home);

    // 3. Option<T> 枚举
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    println!("Option values: {:?}, {:?}, {:?}", some_number, some_string, absent_number);

    // 4. match 表达式
    let m = Message::Write(String::from("hello"));
    process_message(m);

    // 5. if let 语法糖
    let config_max = Some(3u8);
    if let Some(max) = config_max { // 只关心 Some 的情况，忽略 None
        println!("The maximum is configured to be {}", max);
    }
}

// 简单的枚举
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// 带数据的枚举
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("The Quit variant - no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b);
        }
    }
}

// 演示 Option 处理 (类似 Go 的 if err != nil)
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
