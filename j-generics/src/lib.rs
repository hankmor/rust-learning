// 10. 泛型

// 泛型结构体
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// 多个泛型参数
#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

// 泛型枚举
#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

#[derive(Debug)]
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 泛型方法
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    
    fn y(&self) -> &T {
        &self.y
    }
}

// 特定类型的实现
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 多个泛型参数的方法
impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

// 泛型函数
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

// 泛型结构体
#[derive(Debug)]
struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Container<T> {
        Container { value }
    }
    
    fn get_value(&self) -> &T {
        &self.value
    }
}

// 泛型 trait
trait Display {
    fn display(&self);
}

impl<T: std::fmt::Debug> Display for Container<T> {
    fn display(&self) {
        println!("Container: {:?}", self.value);
    }
}

// 泛型函数
fn print_generic<T: std::fmt::Debug>(value: T) {
    println!("值: {:?}", value);
}

// 泛型闭包
fn apply_operation<F, T>(f: F, value: T) -> T 
where
    F: Fn(T) -> T,
{
    f(value)
}

// 泛型结构体与生命周期
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

pub fn run_demo() {
    println!("🔧 泛型演示");
    println!("===========");
    
    // 泛型结构体
    generic_structs();
    println!();
    
    // 泛型枚举
    generic_enums();
    println!();
    
    // 泛型函数
    generic_functions();
    println!();
    
    // 泛型方法
    generic_methods();
    println!();
    
    // 泛型约束
    generic_constraints();
    println!();
    
    // 泛型与生命周期
    generics_and_lifetimes();
    println!();
    
    // 泛型性能
    generic_performance();
}

fn generic_structs() {
    println!("=== 泛型结构体 ===");
    
    // 整数点
    let integer = Point { x: 5, y: 10 };
    println!("整数点: {:?}", integer);
    
    // 浮点数点
    let float = Point { x: 1.0, y: 4.0 };
    println!("浮点数点: {:?}", float);
    
    // 混合类型点
    let mixed = Point2 { x: 5, y: 4.0 };
    println!("混合类型点: {:?}", mixed);
    
    // 泛型容器
    let container = Container::new(42);
    println!("容器: {:?}", container);
    println!("容器值: {}", container.get_value());
    
    let string_container = Container::new("Hello".to_string());
    println!("字符串容器: {:?}", string_container);
}

fn generic_enums() {
    println!("=== 泛型枚举 ===");
    
    // Option 枚举
    let some_number = Option::Some(5);
    let some_string = Option::Some("a string");
    let absent_number: Option<i32> = Option::None;
    
    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);
    println!("absent_number: {:?}", absent_number);
    
    // Result 枚举
    let success: Result<i32, &str> = Result::Ok(42);
    let error: Result<i32, &str> = Result::Err("出错了");
    
    println!("success: {:?}", success);
    println!("error: {:?}", error);
}

fn generic_functions() {
    println!("=== 泛型函数 ===");
    
    // 整数列表
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("最大数字: {}", result);
    
    // 字符列表
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("最大字符: {}", result);
    
    // 浮点数列表
    let float_list = vec![1.1, 2.2, 3.3, 4.4];
    let result = largest(&float_list);
    println!("最大浮点数: {}", result);
    
    // 泛型函数调用
    print_generic(42);
    print_generic("Hello");
    print_generic(3.14);
}

fn generic_methods() {
    println!("=== 泛型方法 ===");
    
    let p = Point { x: 5, y: 10 };
    println!("点的 x 坐标: {}", p.x());
    println!("点的 y 坐标: {}", p.y());
    
    // 特定类型的实现
    let p2 = Point { x: 3.0, y: 4.0 };
    println!("点到原点的距离: {}", p2.distance_from_origin());
    
    // 混合类型点
    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("混合后的点: {:?}", p3);
}

fn generic_constraints() {
    println!("=== 泛型约束 ===");
    
    // 使用 trait 约束
    let container = Container::new(42);
    container.display();
    
    let string_container = Container::new("Hello".to_string());
    string_container.display();
    
    // 泛型闭包
    let double = |x: i32| x * 2;
    let result = apply_operation(double, 5);
    println!("双倍结果: {}", result);
    
    let square = |x: f64| x * x;
    let result = apply_operation(square, 3.0);
    println!("平方结果: {}", result);
}

fn generics_and_lifetimes() {
    println!("=== 泛型与生命周期 ===");
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("找不到 '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("重要摘录: {:?}", i);
    println!("级别: {}", i.level());
    let part = i.announce_and_return_part("这是一个重要通知");
    println!("返回的部分: {}", part);
}

fn generic_performance() {
    println!("=== 泛型性能 ===");
    
    // Rust 的泛型是零成本抽象
    // 编译时单态化（monomorphization）
    
    let numbers = vec![1, 2, 3, 4, 5];
    let largest_num = largest(&numbers);
    println!("最大数字: {}", largest_num);
    
    let chars = vec!['a', 'b', 'c', 'd'];
    let largest_char = largest(&chars);
    println!("最大字符: {}", largest_char);
    
    // 这些调用在编译时会被展开为具体的函数
    // 没有运行时开销
    
    // 泛型结构体的性能
    let point_int = Point { x: 1, y: 2 };
    let point_float = Point { x: 1.0, y: 2.0 };
    
    println!("整数点: {:?}", point_int);
    println!("浮点数点: {:?}", point_float);
    
    // 编译时类型检查
    // let point_mixed = Point { x: 1, y: 2.0 }; // 编译错误！
    
    println!("泛型性能演示完成");
}
