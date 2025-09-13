// 结构体演示模块

// 基本结构体定义
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 元组结构体
#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

// 单元结构体
#[derive(Debug)]
struct AlwaysEqual;

// 基本结构体演示
pub fn basic_struct_demo() {
    println!("=== 基本结构体演示 ===");

    // 创建结构体实例
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("用户1: {:?}", user1);

    // 修改结构体字段
    user1.email = String::from("anotheremail@example.com");
    user1.sign_in_count += 1;

    println!("修改后的用户1: {:?}", user1);

    // 使用字段初始化简写语法
    let email = String::from("test@example.com");
    let username = String::from("testuser");
    let user2 = User {
        email,
        username,
        active: true,
        sign_in_count: 0,
    };

    println!("用户2: {:?}", user2);
}

// 结构体更新语法演示
pub fn struct_update_demo() {
    println!("=== 结构体更新语法演示 ===");

    let user1 = User {
        email: String::from("user1@example.com"),
        username: String::from("user1"),
        active: true,
        sign_in_count: 5,
    };

    // 使用 .. 语法从其他实例创建新实例
    let user2 = User {
        email: String::from("user2@example.com"),
        username: String::from("user2"),
        ..user1 // 其余字段从 user1 复制
    };

    println!("用户1: {:?}", user1);
    println!("用户2: {:?}", user2);

    // 注意：user1 的 email 和 username 没有被移动，只有 active 和 sign_in_count 被复制
    println!("用户1的email: {}", user1.email); // 这行可以正常使用！
    println!("用户1的username: {}", user1.username); // 这行可以正常使用！
    println!("用户1的active: {}", user1.active); // 这行也可以，因为 bool 实现了 Copy

    // 真正的移动演示
    println!("\n--- 真正的移动演示 ---");
    let user3 = User {
        email: String::from("user3@example.com"),
        username: String::from("user3"),
        active: true,
        sign_in_count: 0,
    };

    // 这里会发生真正的移动
    let user4 = User {
        email: user3.email,                 // 移动 user3.email
        username: user3.username,           // 移动 user3.username
        active: user3.active,               // 复制 user3.active
        sign_in_count: user3.sign_in_count, // 复制 user3.sign_in_count
    };

    println!("用户4: {:?}", user4);
    println!("用户3的active: {}", user3.active); // 这行可以，因为 active 被复制了
}

// 元组结构体演示
pub fn tuple_struct_demo() {
    println!("=== 元组结构体演示 ===");

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("黑色: {:?}", black);
    println!("原点: {:?}", origin);

    // 访问元组结构体的字段
    println!("黑色的红色分量: {}", black.0);
    println!("黑色的绿色分量: {}", black.1);
    println!("黑色的蓝色分量: {}", black.2);

    // 元组结构体的解构
    let Color(r, g, b) = black;
    println!("解构颜色: R={}, G={}, B={}", r, g, b);
}

// 单元结构体演示
pub fn unit_struct_demo() {
    println!("=== 单元结构体演示 ===");

    let subject = AlwaysEqual;
    println!("单元结构体: {:?}", subject);

    // 单元结构体通常用作标记类型
    let another_subject = AlwaysEqual;
    println!("另一个单元结构体: {:?}", another_subject);
}

// 结构体方法演示
impl User {
    // 关联函数（类似静态方法）
    fn new(username: String, email: String) -> User {
        User {
            username,
            email,
            active: true,
            sign_in_count: 0,
        }
    }

    // 实例方法
    fn get_username(&self) -> &str {
        &self.username
    }

    fn get_email(&self) -> &str {
        &self.email
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn sign_in(&mut self) {
        self.sign_in_count += 1;
    }

    fn deactivate(&mut self) {
        self.active = false;
    }

    // 获取完整信息
    fn get_info(&self) -> String {
        format!(
            "用户: {}, 邮箱: {}, 活跃: {}, 登录次数: {}",
            self.username, self.email, self.active, self.sign_in_count
        )
    }
}

pub fn struct_methods_demo() {
    println!("=== 结构体方法演示 ===");

    // 使用关联函数创建新用户
    let mut user = User::new(String::from("john_doe"), String::from("john@example.com"));

    println!("新用户: {:?}", user);

    // 调用实例方法
    println!("用户名: {}", user.username);
    println!("用户名: {}", user.get_username());
    println!("邮箱: {}", user.get_email());
    println!("是否活跃: {}", user.is_active());
    println!("用户信息: {}", user.get_info());

    // 修改用户状态
    user.sign_in();
    user.sign_in();
    println!("登录两次后: {}", user.get_info());

    user.deactivate();
    println!("停用后: {}", user.get_info());
}

// 结构体所有权演示
pub fn struct_ownership_demo() {
    println!("=== 结构体所有权演示 ===");

    let user = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        active: true,
        sign_in_count: 10,
    };

    println!("原始用户: {:?}", user);

    // 移动所有权
    let moved_user = user;
    // println!("原始用户: {:?}", user); // 这行会编译错误！user 已被移动

    println!("移动后的用户: {:?}", moved_user);

    // 借用
    let borrowed_user = &moved_user;
    println!("借用的用户: {:?}", borrowed_user);
    println!("移动后的用户仍然可用: {:?}", moved_user);

    // 可变借用
    let mut mutable_user = User {
        username: String::from("bob"),
        email: String::from("bob@example.com"),
        active: true,
        sign_in_count: 5,
    };

    let mutable_ref = &mut mutable_user;
    mutable_ref.sign_in_count += 1;
    println!("修改后的用户: {:?}", mutable_user);
}

// 嵌套结构体演示
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct Window {
    title: String,
    rect: Rectangle,
    visible: bool,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Window {
    fn new(title: String, width: u32, height: u32) -> Window {
        Window {
            title,
            rect: Rectangle { width, height },
            visible: true,
        }
    }

    fn get_area(&self) -> u32 {
        self.rect.area()
    }

    fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }
}

pub fn nested_struct_demo() {
    println!("=== 嵌套结构体演示 ===");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("矩形1: {:?}, 面积: {}", rect1, rect1.area());
    println!("矩形2: {:?}, 面积: {}", rect2, rect2.area());
    println!("矩形1能否包含矩形2: {}", rect1.can_hold(&rect2));

    let mut window = Window::new(String::from("我的窗口"), 800, 600);
    println!("窗口: {:?}", window);
    println!("窗口面积: {}", window.get_area());

    window.toggle_visibility();
    println!("切换可见性后: {:?}", window);
}

// 主演示函数
pub fn struct_demo() {
    println!("🏗️ 结构体演示");
    println!("================================");

    basic_struct_demo();
    println!();

    struct_update_demo();
    println!();

    tuple_struct_demo();
    println!();

    unit_struct_demo();
    println!();

    struct_methods_demo();
    println!();

    struct_ownership_demo();
    println!();

    nested_struct_demo();
    println!();

    println!("💡 总结：");
    println!("  - 结构体是自定义数据类型");
    println!("  - 三种类型：普通结构体、元组结构体、单元结构体");
    println!("  - 使用 impl 块定义方法");
    println!("  - 关联函数用于创建实例");
    println!("  - 实例方法可以借用或获取所有权");
    println!("  - 结构体遵循 Rust 的所有权规则");
}
