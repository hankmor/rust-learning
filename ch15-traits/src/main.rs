use std::fmt::Display;

fn main() {
    println!("=== Traits Demo ===");

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best \
                               hockey team in the NHL."),
    };

    println!("New article available! {}", article.summarize());

    notify(&tweet);
}

// 1. 定义 Trait
pub trait Summary {
    // 抽象方法
    fn summarize_author(&self) -> String;

    // 默认实现
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// 2. 为 struct 实现 Trait
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    // 重写默认实现
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // 使用默认的 summarize 实现
}

// 3. Trait 作为参数 (impl Trait)
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 4. Trait Bound (啰嗦写法，但更灵活)
pub fn notify_bound<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// 5. where 从句 (让签名更清晰)
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Summary,
{
    // ...
    0
}

// 6. 返回 Trait (impl Trait)
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
