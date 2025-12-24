use std::fmt;
use std::ops::Add;

// 1. Advanced Traits: Associated Types
pub trait IteratorLike {
    type Item; // Associated Type
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

impl IteratorLike for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

// 2. Advanced Traits: Operator Overloading
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

// 3. Advanced Types: Newtype Pattern
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    println!("=== 高级特性 (Advanced Features) Demo ===");

    // 1. Unsafe Rust: Raw Pointers
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        *r2 = 6;
        println!("r1 is now: {}", *r1); // Unsafe allow multiple mutable raw pointers
    }

    // Unsafe function
    unsafe fn dangerous() {}
    unsafe {
        dangerous();
    }

    // 2. Advanced Traits
    let mut c = Counter { count: 0 };
    while let Some(i) = c.next() {
        println!("Counter: {}", i);
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
    println!("Point addition worked!");

    // 3. Newtype Pattern
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
