// 08. 集合

use std::collections::HashMap;

pub fn run_demo() {
    println!("📚 集合演示");
    println!("===========");
    
    // 向量
    vectors();
    println!();
    
    // 字符串
    strings();
    println!();
    
    // 哈希映射
    hash_maps();
    println!();
    
    // 数组
    arrays();
    println!();
    
    // 切片
    slices();
    println!();
    
    // 集合操作
    collection_operations();
}

fn vectors() {
    println!("=== 向量 (Vec) ===");
    
    // 创建向量
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("向量: {:?}", v);
    
    // 使用宏创建向量
    let v2 = vec![1, 2, 3, 4, 5];
    println!("向量2: {:?}", v2);
    
    // 访问向量元素
    let third: &i32 = &v2[2];
    println!("第三个元素: {}", third);
    
    // 安全访问
    match v2.get(2) {
        Some(third) => println!("第三个元素: {}", third),
        None => println!("没有第三个元素"),
    }
    
    // 遍历向量
    println!("遍历向量:");
    for i in &v2 {
        println!("  {}", i);
    }
    
    // 可变遍历
    let mut v3 = vec![100, 32, 57];
    for i in &mut v3 {
        *i += 50;
    }
    println!("修改后的向量: {:?}", v3);
    
    // 使用枚举存储不同类型
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("电子表格行: {:?}", row);
}

fn strings() {
    println!("=== 字符串 ===");
    
    // 创建字符串
    let mut s = String::new();
    s.push_str("hello");
    s.push_str(", world!");
    println!("字符串: {}", s);
    
    // 使用 from 创建字符串
    let s2 = String::from("initial contents");
    println!("字符串2: {}", s2);
    
    // 字符串更新
    let mut s3 = String::from("foo");
    s3.push_str("bar");
    s3.push('!');
    println!("字符串3: {}", s3);
    
    // 字符串拼接
    let s4 = String::from("Hello, ");
    let s5 = String::from("world!");
    let s6 = s4 + &s5; // s4 被移动了
    println!("拼接结果: {}", s6);
    
    // 使用 format! 宏
    let s7 = String::from("tic");
    let s8 = String::from("tac");
    let s9 = String::from("toe");
    let s10 = format!("{}-{}-{}", s7, s8, s9);
    println!("format! 结果: {}", s10);
    
    // 字符串索引
    let s11 = String::from("hello");
    // let h = s11[0]; // 编译错误！不能直接索引
    let h = &s11[0..1];
    println!("第一个字符: {}", h);
    
    // 遍历字符串
    println!("遍历字符串字符:");
    for c in s11.chars() {
        println!("  {}", c);
    }
    
    println!("遍历字符串字节:");
    for b in s11.bytes() {
        println!("  {}", b);
    }
}

fn hash_maps() {
    println!("=== 哈希映射 (HashMap) ===");
    
    // 创建哈希映射
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("分数: {:?}", scores);
    
    // 访问值
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(s) => println!("{} 队的分数: {}", team_name, s),
        None => println!("没有找到 {} 队的分数", team_name),
    }
    
    // 遍历哈希映射
    println!("遍历哈希映射:");
    for (key, value) in &scores {
        println!("  {}: {}", key, value);
    }
    
    // 更新哈希映射
    scores.insert(String::from("Blue"), 25); // 覆盖
    println!("更新后的分数: {:?}", scores);
    
    // 只在键不存在时插入
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(30);
    println!("插入后的分数: {:?}", scores);
    
    // 基于旧值更新
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("单词计数: {:?}", map);
}

fn arrays() {
    println!("=== 数组 ===");
    
    // 创建数组
    let a = [1, 2, 3, 4, 5];
    println!("数组: {:?}", a);
    
    // 指定类型和长度
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    println!("数组b: {:?}", b);
    
    // 相同值的数组
    let c = [3; 5];
    println!("数组c: {:?}", c);
    
    // 访问数组元素
    let first = a[0];
    let second = a[1];
    println!("第一个元素: {}, 第二个元素: {}", first, second);
    
    // 数组长度
    println!("数组长度: {}", a.len());
    
    // 遍历数组
    println!("遍历数组:");
    for element in a.iter() {
        println!("  {}", element);
    }
    
    // 使用索引遍历
    for i in 0..a.len() {
        println!("  a[{}] = {}", i, a[i]);
    }
}

fn slices() {
    println!("=== 切片 ===");
    
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("原字符串: {}", s);
    println!("切片: '{}', '{}'", hello, world);
    
    // 数组切片
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("数组切片: {:?}", slice);
    
    // 切片作为参数
    let s2 = String::from("hello world");
    let word = first_word(&s2);
    println!("第一个单词: '{}'", word);
    
    // 字符串字面量切片
    let s3 = "hello world";
    let word = first_word(s3);
    println!("第一个单词: '{}'", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

fn collection_operations() {
    println!("=== 集合操作 ===");
    
    // 向量操作
    let mut v = vec![1, 2, 3, 4, 5];
    println!("原向量: {:?}", v);
    
    // 添加元素
    v.push(6);
    println!("添加元素后: {:?}", v);
    
    // 删除元素
    let popped = v.pop();
    println!("删除元素: {:?}, 剩余: {:?}", popped, v);
    
    // 插入元素
    v.insert(2, 10);
    println!("插入元素后: {:?}", v);
    
    // 删除指定位置元素
    let removed = v.remove(2);
    println!("删除元素: {}, 剩余: {:?}", removed, v);
    
    // 向量容量
    println!("向量长度: {}, 容量: {}", v.len(), v.capacity());
    
    // 清空向量
    v.clear();
    println!("清空后: {:?}", v);
    
    // 字符串操作
    let mut s = String::from("hello");
    println!("原字符串: {}", s);
    
    // 追加字符串
    s.push_str(", world!");
    println!("追加后: {}", s);
    
    // 追加字符
    s.push('!');
    println!("追加字符后: {}", s);
    
    // 字符串替换
    let s2 = s.replace("hello", "hi");
    println!("替换后: {}", s2);
    
    // 字符串分割
    let s3 = "apple,banana,orange";
    let fruits: Vec<&str> = s3.split(',').collect();
    println!("分割结果: {:?}", fruits);
}
