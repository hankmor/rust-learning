use std::fmt::Display;

fn main() {
    println!("=== 生命周期 (Lifetimes) Demo ===");

    // 1. 生命周期避免悬垂引用
    /*
    let r; 
    {
        let x = 5;
        r = &x; // 错误！x 在这里结束生命周期
    }
    println!("r: {}", r); // r 指向了无效内存
    */

    // 2. 泛型生命周期
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // 3. 结构体定义中的生命周期
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("Important excerpt: {:?}", i);

    // 4. 静态生命周期
    let s: &'static str = "I have a static lifetime.";
    println!("{}", s);
}

// 这里的 'a 是生命周期注解
// 它告诉编译器：返回值的生命周期至少和 x, y 中较短的那个一样长
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 如果结构体持有引用，必须标注生命周期
// 这表示 ImportantExcerpt 的实例不能比 part 引用的数据活得更长
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // 第一个规则：编译器自动给每个引用参数分配生命周期 'a, 'b
    //arly 第二个规则：如果只有一个输入生命周期，所有输出生命周期都是它
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// 综合：泛型类型参数、Trait Bound 和生命周期
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
