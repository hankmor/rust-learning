// Debug trait 演示模块

// 没有 derive(Debug) 的结构体
struct UserWithoutDebug {
    name: String,
    age: u32,
}

// 有 derive(Debug) 的结构体
#[derive(Debug)]
struct UserWithDebug {
    name: String,
    age: u32,
}

// 自定义 Debug 实现的结构体
struct UserCustomDebug {
    name: String,
    age: u32,
}

// 手动实现 Debug trait
impl std::fmt::Debug for UserCustomDebug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UserCustomDebug")
            .field("name", &self.name)
            .field("age", &self.age)
            .field("is_adult", &(self.age >= 18))
            .finish()
    }
}

// 基本 Debug 演示
pub fn basic_debug_demo() {
    println!("=== 基本 Debug 演示 ===");
    
    let user_with_debug = UserWithDebug {
        name: String::from("张三"),
        age: 25,
    };
    
    let user_custom_debug = UserCustomDebug {
        name: String::from("李四"),
        age: 17,
    };
    
    // 使用 {:?} 格式化
    println!("用户 (自动Debug): {:?}", user_with_debug);
    println!("用户 (自定义Debug): {:?}", user_custom_debug);
    
    // 使用 {:#?} 美化格式
    println!("用户 (美化格式): {:#?}", user_with_debug);
    println!("用户 (自定义美化): {:#?}", user_custom_debug);
}

// 复杂结构体的 Debug 演示
#[derive(Debug)]
struct Address {
    street: String,
    city: String,
    zip_code: String,
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    address: Address,
    hobbies: Vec<String>,
}

pub fn complex_debug_demo() {
    println!("=== 复杂结构体 Debug 演示 ===");
    
    let person = Person {
        name: String::from("王五"),
        age: 30,
        address: Address {
            street: String::from("中山路123号"),
            city: String::from("北京"),
            zip_code: String::from("100000"),
        },
        hobbies: vec![
            String::from("编程"),
            String::from("阅读"),
            String::from("游泳"),
        ],
    };
    
    println!("人员信息: {:?}", person);
    println!("人员信息 (美化): {:#?}", person);
}

// 枚举的 Debug 演示
#[derive(Debug)]
enum Status {
    Active,
    Inactive,
    Suspended(String), // 带数据的变体
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub fn enum_debug_demo() {
    println!("=== 枚举 Debug 演示 ===");
    
    let status = Status::Suspended(String::from("违反规则"));
    let message = Message::Move { x: 10, y: 20 };
    
    println!("状态: {:?}", status);
    println!("消息: {:?}", message);
    println!("状态 (美化): {:#?}", status);
    println!("消息 (美化): {:#?}", message);
}

// 元组的 Debug 演示
pub fn tuple_debug_demo() {
    println!("=== 元组 Debug 演示 ===");
    
    let tuple = (1, "hello", 3.14, vec![1, 2, 3]);
    println!("元组: {:?}", tuple);
    println!("元组 (美化): {:#?}", tuple);
}

// 错误处理中的 Debug 演示
pub fn error_debug_demo() {
    println!("=== 错误处理 Debug 演示 ===");
    
    let result: Result<i32, String> = Ok(42);
    let error: Result<i32, String> = Err(String::from("出错了"));
    
    println!("成功结果: {:?}", result);
    println!("错误结果: {:?}", error);
    
    let option: Option<String> = Some(String::from("有值"));
    let none: Option<String> = None;
    
    println!("有值: {:?}", option);
    println!("无值: {:?}", none);
}

// 条件编译中的 Debug 演示
#[derive(Debug)]
struct Config {
    debug_mode: bool,
    log_level: String,
}

impl Config {
    fn new() -> Config {
        Config {
            debug_mode: cfg!(debug_assertions),
            log_level: if cfg!(debug_assertions) {
                "debug".to_string()
            } else {
                "info".to_string()
            },
        }
    }
}

pub fn conditional_debug_demo() {
    println!("=== 条件编译 Debug 演示 ===");
    
    let config = Config::new();
    println!("配置: {:?}", config);
    
    // 在调试模式下输出更多信息
    if cfg!(debug_assertions) {
        println!("当前运行在调试模式");
    } else {
        println!("当前运行在发布模式");
    }
}

// 主演示函数
pub fn debug_demo() {
    println!("🐛 Debug trait 演示");
    println!("================================");
    
    basic_debug_demo();
    println!();
    
    complex_debug_demo();
    println!();
    
    enum_debug_demo();
    println!();
    
    tuple_debug_demo();
    println!();
    
    error_debug_demo();
    println!();
    
    conditional_debug_demo();
    println!();
    
    println!("💡 总结：");
    println!("  - #[derive(Debug)] 自动生成 Debug trait 实现");
    println!("  - 让结构体可以使用 {{:?}} 和 {{:#?}} 格式化");
    println!("  - 支持嵌套结构体、枚举、元组等复杂类型");
    println!("  - 可以手动实现 Debug trait 来自定义输出格式");
    println!("  - 主要用于开发和调试阶段");
}
