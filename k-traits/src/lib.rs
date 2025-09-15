// 11. 特征 (Traits)

// 基本特征定义
trait Summary {
    fn summarize(&self) -> String;
}

// 带默认实现的特征
trait SummaryWithDefault {
    fn summarize(&self) -> String {
        String::from("(阅读更多...)")
    }
    
    fn summarize_author(&self) -> String;
}

// 特征作为参数
fn notify(item: &impl Summary) {
    println!("通知: {}", item.summarize());
}

// 特征约束语法
fn notify_trait_bound<T: Summary>(item: &T) {
    println!("通知: {}", item.summarize());
}

// 多个特征约束
fn notify_multiple_traits<T: Summary + std::fmt::Display>(item: &T) {
    println!("通知: {}", item.summarize());
}

// where 子句
fn notify_where_clause<T>(item: &T)
where
    T: Summary + std::fmt::Display,
{
    println!("通知: {}", item.summarize());
}

// 返回实现特征的类型
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("当然，正如你可能已经知道的，人们"),
        reply: false,
        retweet: false,
    }
}

// 特征对象
fn print_summary(item: &dyn Summary) {
    println!("摘要: {}", item.summarize());
}

// 新闻文章结构体
struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl SummaryWithDefault for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

impl std::fmt::Display for NewsArticle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.headline)
    }
}

// 推文结构体
struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl SummaryWithDefault for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

impl std::fmt::Display for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.username, self.content)
    }
}

// 博客文章结构体
struct BlogPost {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for BlogPost {
    fn summarize(&self) -> String {
        format!("博客: {} by {}", self.title, self.author)
    }
}

// 特征实现
trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("绘制半径为 {} 的圆", self.radius);
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("绘制 {}x{} 的矩形", self.width, self.height);
    }
}

// 特征继承
trait Animal {
    fn name(&self) -> &str;
    fn talk(&self) {
        println!("{} 发出声音", self.name());
    }
}

trait Pet: Animal {
    fn owner(&self) -> &str;
    fn greet(&self) {
        println!("{} 向 {} 打招呼", self.name(), self.owner());
    }
}

struct Dog {
    name: String,
    owner: String,
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn talk(&self) {
        println!("{} 汪汪叫", self.name);
    }
}

impl Pet for Dog {
    fn owner(&self) -> &str {
        &self.owner
    }
}

// 关联类型
trait Iterator {
    type Item;
    
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// 默认泛型参数
trait Add<RHS = Self> {
    type Output;
    
    fn add(self, rhs: RHS) -> Self::Output;
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// 运算符重载
use std::ops::Add as StdAdd;

impl StdAdd for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub fn run_demo() {
    println!("🎭 特征演示");
    println!("===========");
    
    // 基本特征
    basic_traits();
    println!();
    
    // 默认实现
    default_implementations();
    println!();
    
    // 特征作为参数
    traits_as_parameters();
    println!();
    
    // 特征约束
    trait_bounds();
    println!();
    
    // 返回特征
    returning_traits();
    println!();
    
    // 特征对象
    trait_objects();
    println!();
    
    // 特征继承
    trait_inheritance();
    println!();
    
    // 关联类型
    associated_types();
    println!();
    
    // 运算符重载
    operator_overloading();
}

fn basic_traits() {
    println!("=== 基本特征 ===");
    
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };
    
    println!("新闻摘要: {}", Summary::summarize(&article));
    
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("当然，正如你可能已经知道的，人们"),
        reply: false,
        retweet: false,
    };
    
    println!("推文摘要: {}", Summary::summarize(&tweet));
}

fn default_implementations() {
    println!("=== 默认实现 ===");
    
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };
    
    println!("新闻摘要: {}", Summary::summarize(&article));
    println!("作者: {}", article.summarize_author());
    
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("当然，正如你可能已经知道的，人们"),
        reply: false,
        retweet: false,
    };
    
    println!("推文摘要: {}", Summary::summarize(&tweet));
    println!("作者: {}", tweet.summarize_author());
}

fn traits_as_parameters() {
    println!("=== 特征作为参数 ===");
    
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };
    
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("当然，正如你可能已经知道的，人们"),
        reply: false,
        retweet: false,
    };
    
    notify(&article);
    notify(&tweet);
}

fn trait_bounds() {
    println!("=== 特征约束 ===");
    
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };
    
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("当然，正如你可能已经知道的，人们"),
        reply: false,
        retweet: false,
    };
    
    notify_trait_bound(&article);
    notify_trait_bound(&tweet);
    
    notify_where_clause(&article);
    notify_where_clause(&tweet);
}

fn returning_traits() {
    println!("=== 返回特征 ===");
    
    let item = returns_summarizable();
    println!("返回的摘要: {}", item.summarize());
}

fn trait_objects() {
    println!("=== 特征对象 ===");
    
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };
    
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("当然，正如你可能已经知道的，人们"),
        reply: false,
        retweet: false,
    };
    
    print_summary(&article);
    print_summary(&tweet);
    
    // 特征对象向量
    let drawables: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle { width: 10.0, height: 20.0 }),
    ];
    
    for drawable in drawables {
        drawable.draw();
    }
}

fn trait_inheritance() {
    println!("=== 特征继承 ===");
    
    let dog = Dog {
        name: String::from("Buddy"),
        owner: String::from("Alice"),
    };
    
    dog.talk();
    dog.greet();
}

fn associated_types() {
    println!("=== 关联类型 ===");
    
    let mut counter = Counter::new();
    
    while let Some(count) = counter.next() {
        println!("计数: {}", count);
    }
}

fn operator_overloading() {
    println!("=== 运算符重载 ===");
    
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;
    
    println!("p1 + p2 = {:?}", p3);
    
    // 使用标准库的 Add trait
    let p4 = Point { x: 5, y: 6 };
    let p5 = Point { x: 7, y: 8 };
    let p6 = p4 + p5;
    
    println!("p4 + p5 = {:?}", p6);
}
