// 06. 结构体

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct AlwaysEqual;

impl User {
    fn new(username: String, email: String) -> User {
        User {
            username,
            email,
            active: true,
            sign_in_count: 0,
        }
    }

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

    fn get_info(&self) -> String {
        format!(
            "用户: {}, 邮箱: {}, 活跃: {}, 登录次数: {}",
            self.username, self.email, self.active, self.sign_in_count
        )
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub fn run_demo() {
    println!("🏗️ 结构体演示");
    println!("=============");

    // 基本结构体
    basic_structs();
    println!();

    // 元组结构体
    tuple_structs();
    println!();

    // 单元结构体
    unit_structs();
    println!();

    // 结构体方法
    struct_methods();
    println!();

    // 关联函数
    associated_functions();
    println!();

    // 结构体更新语法
    struct_update_syntax();
    println!();

    // 嵌套结构体
    nested_structs();
    println!();

    // 结构体所有权转移示例
    ownership_transfer();
    println!();

    // 结构体所有权转移示例
    borrow_ownership();
    println!();
}

fn basic_structs() {
    println!("=== 基本结构体 ===");

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

fn tuple_structs() {
    println!("=== 元组结构体 ===");

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

fn unit_structs() {
    println!("=== 单元结构体 ===");

    let subject = AlwaysEqual;
    println!("单元结构体: {:?}", subject);

    // 单元结构体通常用作标记类型
    let another_subject = AlwaysEqual;
    println!("另一个单元结构体: {:?}", another_subject);
}

fn struct_methods() {
    println!("=== 结构体方法 ===");

    // 使用关联函数创建新用户
    let mut user = User::new(String::from("john_doe"), String::from("john@example.com"));

    println!("新用户: {:?}", user);

    // 调用实例方法
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

fn associated_functions() {
    println!("=== 关联函数 ===");

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
    println!("矩形1能否包含矩形2: {}", rect1.can_hold(&rect2)); // true

    // 使用关联函数创建正方形
    let square = Rectangle::square(10);
    println!("正方形: {:?}, 面积: {}", square, square.area());
}

fn struct_update_syntax() {
    println!("=== 结构体更新语法 ===");

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
    println!("用户1的email: {}", user1.email);
    println!("用户1的username: {}", user1.username);
    println!("用户1的active: {}", user1.active);
}

fn nested_structs() {
    println!("=== 嵌套结构体 ===");

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
    }

    let person = Person {
        name: String::from("Alice"),
        age: 30,
        address: Address {
            street: String::from("123 Main St"),
            city: String::from("New York"),
            zip_code: String::from("10001"),
        },
    };

    println!("人员信息: {:?}", person);
    println!("姓名: {}", person.name);
    println!("年龄: {}", person.age);
    println!("街道: {}", person.address.street);
    println!("城市: {}", person.address.city);
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

// 结构体所有权转移示例
fn ownership_transfer() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    // 下面这行会报错, user1的username已经移动到user2了，所以无法访问user1， 但是可以访问其他字段
    // println!("{:?}", user1);
    // 同样无法访问username
    // println!("{}", user1.username);
    println!("{}", user1.active);
    println!("{}", user1.email);
    println!("{}", user1.sign_in_count);
}

fn borrow_ownership() {
    println!("=== 结构体所有权转移示例 ===");

    let f1 = File {
        name: String::from("f1.txt"),
        data: vec![1, 2, 3],
    };

    // 转义所有权，之后无法访问f1
    // let f1_name = f1.name;
    // let f1_len = f1.data.len();
    // println!("f1: {:?}", f1);
    // 借用所有权，之后可以访问f1
    let f1_name = &f1.name;
    let f1_len = &f1.data.len();
    print!("f1: {:?}", f1);
    println!("f1_name: {}", f1_name);
    println!("f1_len: {}", f1_len);
}
