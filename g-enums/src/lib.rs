// 07. 枚举

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddrDetailed {
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

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("退出程序"),
            Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
            Message::Write(text) => println!("写入文本: {}", text),
            Message::ChangeColor(r, g, b) => println!("改变颜色为 RGB({}, {}, {})", r, g, b),
        }
    }
}

// 使用标准库的 Option 和 Result

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    // ... 其他州
}

#[derive(Debug)]
enum CoinWithState {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl CoinWithState {
    fn value_in_cents(&self) -> u8 {
        match self {
            CoinWithState::Penny => 1,
            CoinWithState::Nickel => 5,
            CoinWithState::Dime => 10,
            CoinWithState::Quarter(state) => {
                println!("来自 {:?} 的 25 美分硬币!", state);
                25
            }
        }
    }
}

pub fn run_demo() {
    println!("🎯 枚举演示");
    println!("===========");
    
    // 基本枚举
    basic_enums();
    println!();
    
    // 带数据的枚举
    enums_with_data();
    println!();
    
    // 枚举方法
    enum_methods();
    println!();
    
    // Option 枚举
    option_enum();
    println!();
    
    // Result 枚举
    result_enum();
    println!();
    
    // match 表达式
    match_expressions();
    println!();
    
    // if let 语法
    if_let_syntax();
    println!();
    
    // 复杂枚举
    complex_enums();
}

fn basic_enums() {
    println!("=== 基本枚举 ===");
    
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    println!("IPv4: {:?}", four);
    println!("IPv6: {:?}", six);
    
    route(four);
    route(six);
}

fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => println!("路由 IPv4 地址"),
        IpAddrKind::V6 => println!("路由 IPv6 地址"),
    }
}

fn enums_with_data() {
    println!("=== 带数据的枚举 ===");
    
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    
    println!("家庭地址: {:?}", home);
    println!("回环地址: {:?}", loopback);
    
    let home_detailed = IpAddrDetailed::V4(127, 0, 0, 1);
    let loopback_detailed = IpAddrDetailed::V6(String::from("::1"));
    
    println!("详细家庭地址: {:?}", home_detailed);
    println!("详细回环地址: {:?}", loopback_detailed);
}

fn enum_methods() {
    println!("=== 枚举方法 ===");
    
    let m = Message::Write(String::from("hello"));
    m.call();
    
    let m2 = Message::Move { x: 10, y: 20 };
    m2.call();
    
    let m3 = Message::ChangeColor(255, 0, 0);
    m3.call();
    
    let m4 = Message::Quit;
    m4.call();
}

fn option_enum() {
    println!("=== Option 枚举 ===");
    
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    
    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);
    println!("absent_number: {:?}", absent_number);
    
    // 使用 match 处理 Option
    match some_number {
        Some(value) => println!("有值: {}", value),
        None => println!("没有值"),
    }
    
    match absent_number {
        Some(value) => println!("有值: {}", value),
        None => println!("没有值"),
    }
    
    // 使用 if let 处理 Option
    if let Some(value) = some_number {
        println!("if let 有值: {}", value);
    }
}

fn result_enum() {
    println!("=== Result 枚举 ===");
    
    let success: Result<i32, &str> = Ok(42);
    let error: Result<i32, &str> = Err("出错了");
    
    println!("成功: {:?}", success);
    println!("错误: {:?}", error);
    
    // 使用 match 处理 Result
    match success {
        Ok(value) => println!("成功: {}", value),
        Err(e) => println!("错误: {}", e),
    }
    
    match error {
        Ok(value) => println!("成功: {}", value),
        Err(e) => println!("错误: {}", e),
    }
    
    // 使用 if let 处理 Result
    if let Ok(value) = success {
        println!("if let 成功: {}", value);
    }
    
    if let Err(e) = error {
        println!("if let 错误: {}", e);
    }
}

fn match_expressions() {
    println!("=== match 表达式 ===");
    
    let coin = Coin::Quarter;
    println!("硬币价值: {} 美分", coin.value_in_cents());
    
    let coin2 = Coin::Penny;
    println!("硬币价值: {} 美分", coin2.value_in_cents());
    
    // 带状态的硬币
    let coin3 = CoinWithState::Quarter(UsState::California);
    println!("硬币价值: {} 美分", coin3.value_in_cents());
    
    // 复杂 match
    let number = 5;
    match number {
        1 => println!("一"),
        2 => println!("二"),
        3 => println!("三"),
        4 | 5 | 6 | 7 | 8 | 9 => println!("四到九"),
        10..=19 => println!("十到十九"),
        _ => println!("其他"),
    }
}

fn if_let_syntax() {
    println!("=== if let 语法 ===");
    
    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("三"),
        _ => (),
    }
    
    // 使用 if let 简化
    if let Some(3) = some_u8_value {
        println!("if let 三");
    }
    
    // 复杂 if let
    let mut count = 0;
    let coin = CoinWithState::Quarter(UsState::Alaska);
    
    if let CoinWithState::Quarter(state) = coin {
        println!("来自 {:?} 的 25 美分硬币!", state);
    } else {
        count += 1;
    }
    
    println!("计数: {}", count);
}

fn complex_enums() {
    println!("=== 复杂枚举 ===");
    
    #[derive(Debug)]
    enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(char),
        Paste(String),
        Click { x: i64, y: i64 },
    }
    
    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("页面加载"),
            WebEvent::PageUnload => println!("页面卸载"),
            WebEvent::KeyPress(c) => println!("按键: '{}'", c),
            WebEvent::Paste(s) => println!("粘贴: \"{}\"", s),
            WebEvent::Click { x, y } => println!("点击位置: ({}, {})", x, y),
        }
    }
    
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;
    
    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}
