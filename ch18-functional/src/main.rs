use std::thread;
use std::time::Duration;

fn main() {
    println!("=== 闭包与迭代器 (Closures & Iterators) Demo ===");

    // 1. 闭包 (Closures)
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    // 闭包捕获环境
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
    println!("Closure captured x: {}", x);

    // 2. 迭代器 (Iterators)
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // 创建迭代器，惰性的

    // 使用迭代器
    let total: i32 = v1_iter.sum(); // 消耗迭代器
    println!("Sum of v1: {}", total);

    // 适配器方法 (Iterator Adaptors)
    let v1: Vec<i32> = vec![1, 2, 3];
    // map 是惰性的，必须随后调用消耗的方法（如 collect）
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("v1 defined mapped to v2: {:?}", v2);

    // 过滤器
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_size(shoes, 10);
    println!("Shoes in size 10: {:?}", in_my_size);
}

// 模拟耗时计算
fn generate_workout(intensity: u32, random_number: u32) {
    // 定义一个缓存耗时结果的结构体（Cacher）的简化版：直接用闭包
    // 只有在需要时才执行
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(1));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

// 缓存器结构体
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}
