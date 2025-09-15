// 12. 生命周期

// 基本生命周期注解
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 结构体中的生命周期
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("注意！{}", announcement);
        self.part
    }
}

// 生命周期省略规则
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// 多个生命周期参数
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: std::fmt::Display,
{
    println!("通知！{}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 静态生命周期
fn string_literal() -> &'static str {
    "I have a static lifetime."
}

// 生命周期与泛型
struct Point<'a, T> {
    x: &'a T,
    y: &'a T,
}

impl<'a, T> Point<'a, T> {
    fn new(x: &'a T, y: &'a T) -> Point<'a, T> {
        Point { x, y }
    }
    
    fn get_x(&self) -> &'a T {
        self.x
    }
    
    fn get_y(&self) -> &'a T {
        self.y
    }
}

// 生命周期与特征
trait LifetimeTrait<'a> {
    fn get_value(&self) -> &'a str;
}

struct LifetimeStruct<'a> {
    value: &'a str,
}

impl<'a> LifetimeTrait<'a> for LifetimeStruct<'a> {
    fn get_value(&self) -> &'a str {
        self.value
    }
}

// 生命周期与闭包
fn lifetime_with_closure() {
    let s = String::from("hello");
    let closure = || &s;
    let result = closure();
    println!("闭包结果: {}", result);
}

// 生命周期与迭代器
fn lifetime_with_iterator() {
    let v = vec![1, 2, 3, 4, 5];
    let iter = v.iter();
    
    for item in iter {
        println!("迭代器项: {}", item);
    }
}

// 生命周期与引用计数
use std::rc::Rc;

fn lifetime_with_rc() {
    let s = Rc::new(String::from("hello"));
    let s2 = s.clone();
    println!("Rc 引用计数: {}", Rc::strong_count(&s));
    println!("s: {}, s2: {}", s, s2);
}

// 生命周期与智能指针
use std::cell::RefCell;

fn lifetime_with_refcell() {
    let data = RefCell::new(5);
    {
        let mut borrow = data.borrow_mut();
        *borrow += 1;
    }
    println!("RefCell 值: {}", data.borrow());
}

// 生命周期与异步
async fn lifetime_with_async() {
    let s = String::from("async lifetime");
    let result = async_process(&s).await;
    println!("异步结果: {}", result);
}

async fn async_process(s: &str) -> String {
    format!("处理: {}", s)
}

// 生命周期与错误处理
fn lifetime_with_error_handling() -> Result<&'static str, &'static str> {
    Ok("成功")
}

// 生命周期与模式匹配
fn lifetime_with_pattern_matching() {
    let s = String::from("hello world");
    let first_word = match s.split_whitespace().next() {
        Some(word) => word,
        None => "",
    };
    println!("第一个单词: {}", first_word);
}

pub fn run_demo() {
    println!("⏰ 生命周期演示");
    println!("===============");
    
    // 基本生命周期
    basic_lifetimes();
    println!();
    
    // 结构体中的生命周期
    lifetimes_in_structs();
    println!();
    
    // 生命周期省略
    lifetime_elision();
    println!();
    
    // 多个生命周期参数
    multiple_lifetime_parameters();
    println!();
    
    // 静态生命周期
    static_lifetimes();
    println!();
    
    // 生命周期与泛型
    lifetimes_with_generics();
    println!();
    
    // 生命周期与特征
    lifetimes_with_traits();
    println!();
    
    // 生命周期与智能指针
    lifetimes_with_smart_pointers();
    println!();
    
    // 生命周期最佳实践
    lifetime_best_practices();
}

fn basic_lifetimes() {
    println!("=== 基本生命周期 ===");
    
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest(string1.as_str(), string2);
    println!("最长的字符串是: {}", result);
    
    // 生命周期注解确保返回的引用在参数的生命周期内有效
    let string3 = String::from("long string is long");
    {
        let string4 = String::from("xyz");
        let result = longest(string3.as_str(), string4.as_str());
        println!("最长的字符串是: {}", result);
    } // string4 在这里离开作用域
    // result 仍然有效，因为它引用的是 string3
}

fn lifetimes_in_structs() {
    println!("=== 结构体中的生命周期 ===");
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("找不到 '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("重要摘录: {:?}", i);
    println!("级别: {}", i.level());
    
    let part = i.announce_and_return_part("这是一个重要通知");
    println!("返回的部分: {}", part);
    
    // 生命周期注解确保 ImportantExcerpt 的实例不能比它包含的引用活得更久
}

fn lifetime_elision() {
    println!("=== 生命周期省略 ===");
    
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("第一个单词: {}", word);
    
    // 编译器可以推断生命周期，所以不需要显式注解
    let s2 = "hello world";
    let word2 = first_word(s2);
    println!("第一个单词: {}", word2);
}

fn multiple_lifetime_parameters() {
    println!("=== 多个生命周期参数 ===");
    
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        "比较字符串长度",
    );
    println!("最长的字符串是: {}", result);
    
    // 多个生命周期参数允许函数处理不同生命周期的引用
}

fn static_lifetimes() {
    println!("=== 静态生命周期 ===");
    
    let s: &'static str = "I have a static lifetime.";
    println!("静态字符串: {}", s);
    
    let s2 = string_literal();
    println!("静态字符串字面量: {}", s2);
    
    // 静态生命周期表示引用在整个程序运行期间都有效
}

fn lifetimes_with_generics() {
    println!("=== 生命周期与泛型 ===");
    
    let x = 5;
    let y = 10;
    let point = Point::new(&x, &y);
    
    println!("点的 x 坐标: {}", point.get_x());
    println!("点的 y 坐标: {}", point.get_y());
    
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let point2 = Point::new(&s1, &s2);
    
    println!("点的 x 坐标: {}", point2.get_x());
    println!("点的 y 坐标: {}", point2.get_y());
}

fn lifetimes_with_traits() {
    println!("=== 生命周期与特征 ===");
    
    let s = String::from("hello");
    let lifetime_struct = LifetimeStruct { value: &s };
    
    println!("特征值: {}", lifetime_struct.get_value());
    
    // 生命周期与特征结合使用，确保特征方法返回的引用有效
}

fn lifetimes_with_smart_pointers() {
    println!("=== 生命周期与智能指针 ===");
    
    lifetime_with_rc();
    println!();
    
    lifetime_with_refcell();
    println!();
    
    // 智能指针可以延长数据的生命周期
    let s = Rc::new(String::from("hello"));
    let s2 = s.clone();
    println!("克隆后的字符串: {}", s2);
}

fn lifetime_best_practices() {
    println!("=== 生命周期最佳实践 ===");
    
    // 1. 尽可能让编译器推断生命周期
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("第一个单词: {}", word);
    
    // 2. 使用引用计数智能指针
    let s = Rc::new(String::from("hello"));
    let s2 = s.clone();
    println!("引用计数字符串: {}", s2);
    
    // 3. 使用生命周期注解明确关系
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("最长的字符串: {}", result);
    
    // 4. 避免悬垂引用
    // let reference_to_nothing = dangle(); // 这会编译错误
    
    // 5. 使用生命周期与泛型结合
    let x = 5;
    let y = 10;
    let point = Point::new(&x, &y);
    println!("泛型点: x={}, y={}", point.get_x(), point.get_y());
}

// 这个函数会编译错误，因为它返回悬垂引用
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // 返回 s 的引用，但 s 在这里离开作用域
// } // s 在这里被丢弃
