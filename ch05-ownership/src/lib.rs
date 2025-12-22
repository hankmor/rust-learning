// 05. 所有权

pub fn run_demo() {
    println!("🏠 所有权演示");
    println!("=============");
    
    // 所有权基础
    ownership_basics();
    println!();
    
    // 移动语义
    move_semantics();
    println!();
    
    // 克隆
    cloning();
    println!();
    
    // 引用和借用
    references_and_borrowing();
    println!();
    
    // 可变引用
    mutable_references();
    println!();
    
    // 悬垂引用
    dangling_references();
    println!();
    
    // 切片
    slices();
}

fn ownership_basics() {
    println!("=== 所有权基础 ===");
    
    // 栈上数据
    let x = 5;
    let y = x; // 复制
    println!("x = {}, y = {}", x, y);
    
    // 堆上数据
    let s1 = String::from("hello");
    let s2 = s1; // 移动
    // println!("s1 = {}", s1); // 编译错误！s1 已被移动
    println!("s2 = {}", s2);
    
    // 字符串字面量
    let s3 = "hello";
    let s4 = s3; // 复制（因为 &str 在栈上）
    println!("s3 = {}, s4 = {}", s3, s4);
}

fn move_semantics() {
    println!("=== 移动语义 ===");
    
    let s1 = String::from("hello");
    let s2 = s1; // s1 被移动到 s2
    
    // s1 不再有效
    // println!("s1 = {}", s1); // 编译错误！
    println!("s2 = {}", s2);
    
    // 函数调用中的移动
    let s3 = String::from("world");
    takes_ownership(s3);
    // println!("s3 = {}", s3); // 编译错误！s3 已被移动
    
    let x = 5;
    makes_copy(x);
    println!("x 仍然有效: {}", x); // x 仍然有效，因为 i32 实现了 Copy
}

fn takes_ownership(some_string: String) {
    println!("函数接收字符串: {}", some_string);
} // some_string 在这里离开作用域并被丢弃

fn makes_copy(some_integer: i32) {
    println!("函数接收整数: {}", some_integer);
} // some_integer 在这里离开作用域，但因为是 Copy 类型，所以没有特殊操作

fn cloning() {
    println!("=== 克隆 ===");
    
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 深拷贝
    
    println!("s1 = {}, s2 = {}", s1, s2);
    
    // 克隆 vs 移动
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("克隆后: s3 = {}, s4 = {}", s3, s4);
    
    let s5 = String::from("rust");
    let s6 = s5; // 移动
    // println!("移动后: s5 = {}", s5); // 编译错误！
    println!("移动后: s6 = {}", s6);
}

fn references_and_borrowing() {
    println!("=== 引用和借用 ===");
    
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 借用
    println!("字符串 '{}' 的长度是 {}", s1, len);
    
    // 多个不可变引用
    let s2 = String::from("hello");
    let r1 = &s2;
    let r2 = &s2;
    let r3 = &s2;
    println!("r1 = {}, r2 = {}, r3 = {}", r1, r2, r3);
    
    // 引用作用域
    let s3 = String::from("hello");
    {
        let r1 = &s3;
        println!("r1 = {}", r1);
    } // r1 在这里离开作用域
    let r2 = &s3;
    println!("r2 = {}", r2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s 离开作用域，但因为它是引用，所以不会丢弃它指向的数据

fn mutable_references() {
    println!("=== 可变引用 ===");
    
    let mut s = String::from("hello");
    change(&mut s);
    println!("修改后的字符串: {}", s);
    
    // 可变引用的限制
    let mut s1 = String::from("hello");
    let r1 = &mut s1;
    // let r2 = &mut s1; // 编译错误！不能有多个可变引用
    // let r3 = &s1; // 编译错误！不能同时有可变和不可变引用
    println!("r1 = {}", r1);
    
    // 作用域分离
    let mut s2 = String::from("hello");
    {
        let r1 = &mut s2;
        r1.push_str(", world");
    } // r1 在这里离开作用域
    let r2 = &s2;
    println!("r2 = {}", r2);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangling_references() {
    println!("=== 悬垂引用 ===");
    
    // 这个函数会编译错误，因为返回了悬垂引用
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s // 返回 s 的引用，但 s 在这里离开作用域
    // } // s 在这里被丢弃
    
    // 正确的做法是返回所有权
    let s = no_dangle();
    println!("没有悬垂引用: {}", s);
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s // 返回 s 的所有权
}

fn slices() {
    println!("=== 切片 ===");
    
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("原字符串: {}", s);
    println!("切片: '{}', '{}'", hello, world);
    
    // 字符串字面量就是切片
    let s2 = "Hello, world!";
    let slice = &s2[0..5];
    println!("字符串字面量切片: '{}'", slice);
    
    // 数组切片
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("数组切片: {:?}", slice);
    
    // 切片作为参数
    let s3 = String::from("hello world");
    let word = first_word(&s3);
    println!("第一个单词: '{}'", word);
    
    // 字符串字面量切片
    let s4 = "hello world";
    let word = first_word(s4);
    println!("第一个单词: '{}'", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}
