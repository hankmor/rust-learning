fn main() {
    println!("=== Slice (切片) Demo ===");

    // 1. 字符串切片 (String Slices)
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("Full string: {}", s);
    println!("Slice 1: {}", hello);
    println!("Slice 2: {}", world);

    // 2. 切片只是引用，不持有所有权
    // s.clear(); // 错误！s 被借用为切片 hello/world 了，不能修改

    println!("Use slice: {}", hello); // 这里最后一次使用 hello

    // 3. 数组切片 (Array Slices)
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("Array slice: {:?}", slice);
    assert_eq!(slice, &[2, 3]);

    // 4. 字符串字面量就是切片
    let s_literal = "Hello"; // s_literal 的类型是 &str
    println!("Type of s_literal is &str: {}", s_literal);

    // 5. 编写接收切片的函数（更通用）
    let word = first_word(&s); 
    // let word = first_word("Hello Literal"); // 也能接收字面量
    println!("The first word is: {}", word);
}

fn first_word(s: &str) -> &str { // 参数用 &str 而不是 &String，更通用
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
