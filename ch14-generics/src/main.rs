fn main() {
    println!("=== 泛型 (Generics) Demo ===");

    // 1. 泛型函数
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = get_largest(&number_list);
    println!("The largest number is {}", largest);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let largest = get_largest(&char_list);
    println!("The largest char is {}", largest);

    // 2. 泛型结构体
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("Point Integer: {:?}", integer);
    println!("Point Float: {:?}", float);
    
    // 3. 混合泛型
    let mixed = PointMixed { x: 5, y: 4.0 };
    println!("Mixed Point: {:?}", mixed);

    // 4. 泛型方法
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}

// 泛型函数：找出切片中最大的元素
// T: PartialOrd + Copy
// 约束 T 必须实现 PartialOrd (可比较) 和 Copy (可拷贝)
fn get_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 泛型结构体
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// 泛型 impl 块
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 如果我们只想为 Point<f32> 实现方法
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 多泛型参数
#[derive(Debug)]
struct PointMixed<T, U> {
    x: T,
    y: U,
}
