// 定义一个名为 my_vec 的宏
// #[macro_export] // 如果是一个库，需要加上这个注解才能被外部使用
macro_rules! my_vec {
    // 模式匹配分支
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    println!("=== 宏入门 (Macros) Demo ===");

    let v: Vec<u32> = my_vec![1, 2, 3];
    println!("v = {:?}", v);

    let v2: Vec<&str> = my_vec!("Hello", "Macro");
    println!("v2 = {:?}", v2);
}
