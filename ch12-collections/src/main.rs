use std::collections::HashMap;

fn main() {
    println!("=== 常用集合 (Common Collections) Demo ===");

    // 1. Vector (动态数组)
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    // vec! 宏
    let v2 = vec![1, 2, 3];

    // 读取元素
    let third: &i32 = &v2[2]; // 索引访问，越界会 panic
    println!("The third element is {}", third);

    match v2.get(2) { // get 方法返回 Option<&T>，越界返回 None
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // 遍历
    for i in &v {
        println!("{}", i);
    }

    // 2. String (UTF-8 编码的字符串)
    let data = "initial contents";
    let s = data.to_string(); // 或者 String::from("initial contents")
    
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // append
    println!("s1 is {}", s1);

    // 字符串拼接
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // 注意：s1 被移动了，不能再用了
    let s3 = s1 + &s2; 
    println!("s3 is {}", s3);

    // format! 宏 (推荐，不移动所有权)
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("formatted: {}", s);

    // 3. HashMap (哈希表)
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 访问
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0); // get 返回 Option<&V>
    println!("Blue team score: {}", score);

    // 遍历
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 更新：只在键不存在时插入
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // 更新：基于旧值更新 (单词计数)
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
