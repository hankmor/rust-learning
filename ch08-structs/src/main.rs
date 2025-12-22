fn main() {
    println!("=== 结构体 (Structs) Demo ===");

    // 1. 定义和实例化结构体
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // 2. 修改字段 (实例必须是 mut 的)
    user1.email = String::from("anotheremail@example.com");
    println!("User 1 email: {}", user1.email);

    // 3. 结构体更新语法 (Struct Update Syntax)
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // 剩下的字段从 user1拷贝（如果是 Copy 类型）或移动（如果是 Move 类型）
    };
    println!("User 2 username: {}", user2.username);
    // 注意：由于 String 是 Move 的，user1.username 已经被 Move 给 user2 了
    // println!("User 1 username: {}", user1.username); // 报错！

    // 4. 元组结构体 (Tuple Structs)
    let black = Color(0, 0, 0);
    println!("Color: {}, {}, {}", black.0, black.1, black.2);

    // 5. 方法 (Methods)
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}

// 定义结构体
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 元组结构体
struct Color(i32, i32, i32);

// 带有方法的结构体
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 方法：第一个参数是 &self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 可以定义与字段同名的方法（Getters）
    fn width(&self) -> bool {
        self.width > 0
    }
}
