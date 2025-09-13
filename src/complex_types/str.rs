// 字符串和切片演示模块

// 辅助函数：获取字符串的第一个单词
fn first_word(s: &String) -> &str {
    &s[..1]
}

// 演示借用冲突的错误代码（注释掉，因为会编译失败）
#[allow(dead_code)]
fn str_slice_borrow_error() {
    // Rust 的借用规则不允许同时存在可变借用和不可变借用
    // 当 word 变量持有对 s 的不可变借用时，s.clear() 无法获得可变借用
    let s = String::from("hello world"); // 不需要 mut，因为我们注释掉了 s.clear()
    let word = first_word(&s); // word 持有对 s 的不可变借用(字符串切片)
                               // s.clear(); // 这行会编译错误！创建对 s 的可变借用(会修改: 清空字符串)
    println!("the first word is: {}", word); // 对 word 的不可变借用
}

// 正确的解决方案1：先使用，后修改
fn str_slice_borrow_ok_last_clear() {
    println!("=== 解决方案1：先使用，后修改 ===");
    let mut s = String::from("hello world");
    let word = first_word(&s);
    // 先使用,后修改就可以解决借用规则的错误
    println!("the first word is: {}", word);
    s.clear();
    println!("字符串已清空，长度: {}", s.len());
}

// 正确的解决方案2：克隆数据
fn str_slice_borrow_ok_clone() {
    println!("=== 解决方案2：克隆数据 ===");
    let mut s = String::from("hello world");
    let word = first_word(&s).to_string(); // 克隆字符串切片
    s.clear(); // 可以修改，因为word拥有自己的数据
    println!("the first word is: {}", word);
    println!("原字符串已清空，长度: {}", s.len());
}

// 正确的解决方案3：使用作用域
fn str_slice_borrow_ok_scope() {
    println!("=== 解决方案3：使用作用域 ===");
    let mut s = String::from("hello world");

    {
        let word = first_word(&s);
        println!("the first word is: {}", word);
    } // word 在这里离开作用域，借用结束

    s.clear(); // 现在可以修改了
    println!("字符串已清空，长度: {}", s.len());
}

fn str_append() {
    println!("=== 字符串追加 ===");
    // 追加操作需要修改原字符串，所以原字符串需要是可变的
    let mut s = String::from("Hello ");

    // 追加字符串
    s.push_str("rust");
    println!("追加字符串 push_str() -> {}", s);

    // 追加字符
    s.push('!');
    println!("追加字符 push() -> {}", s);
}

fn str_replace() {
    println!("=== 字符串替换 ===");
    let s = String::from("Hello rust, l like rust");
    // replace() 替换整个字符串
    let s = s.replace("rust", "world"); // 替换字符串 replace() -> 返回新的字符串
    println!("替换字符串 replace() -> {}", s);

    let string_replace = "Hello rust, l like rust";
    // replacen() 替换指定次数
    let new_string_replacen = string_replace.replacen("rust", "RUST", 1);
    println!("替换指定次数 replacen() -> {}", new_string_replacen);

    let mut string_replace_range = String::from("I like rust!");
    // replace_range() 替换指定范围
    string_replace_range.replace_range(7..8, "R");
    println!("替换指定范围 replace_range() -> {}", string_replace_range);
}

fn str_delete() {
    println!("=== 字符串删除 ===");
    let mut s = String::from("Hellorust!");
    // pop() 删除最后一个字符
    s.pop();
    println!("删除最后一个字符 pop() -> {}", s);
    // remove() 删除指定索引的字符
    s.remove(0);
    println!("删除指定索引的字符 remove() -> {}", s);
    // truncate() 删除指定范围开始到末尾的字符
    s.truncate(5);
    println!("删除指定范围开始到末尾的字符 truncate() -> {}", s);
    // clear() 清空字符串
    s.clear();
    println!("清空字符串 clear() -> {}", s);
}

fn str_concat() {
    println!("=== 字符串拼接 ===");
    let s1 = String::from("Hello");
    let s2 = String::from("rust");
    // 字符串拼接 +, 底层调用的 add 方法, &s2会自动解引用为 &str
    // s1的所有权被转移走了，因此后面不能再使用s1
    let s3 = s1 + " " + &s2;
    println!("字符串拼接 + -> {}", s3);
    // 使用 format! 拼接字符串
    // 不能再使用 s1, 因为s1的所有权被转移走了
    // let s4 = format!("{} {}", s1, s2);
    let s4 = String::from("Hello");
    let s5 = String::from("rust");
    let s6 = format!("{} {}", s4, s5);
    println!("字符串拼接 format! -> {}", s6);
}

fn str_escape() {
    println!("=== 字符串转义 ===");
    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 换行了也会保持之前的字符串格式
    // 使用\忽略换行符
    let long_string = "String literals
                            can span multiple lines.
                            The linebreak and indentation here ->\
                            <- can be escaped too!";
    println!("{}", long_string);
    
    // 保持原样,不转义
    println!("=== 保持原样,不转义 ===");
    // 使用 \\ 输出原来的字符
    println!("{}", "hello \\x52\\x75\\x73\\x74");
    // 使用 r 保持原样,不转义
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果字符串中包含 # 号，可以在开头和结尾加多个 # 号，最多加255个，只需保证与字符串中连续 # 号的个数不超过开头和结尾的 # 号的个数即可
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
}

fn str_utf8() {
    println!("=== 字符串utf-8编码 ===");
    
    // 使用 chars() 方法可以遍历unicode编码的字符串, 返回的是每一个字符
    for c in "你好,世界!".chars() {
        println!("{}", c);
    }

    // 使用 bytes() 方法可以遍历utf-8编码的字符串, 返回的是每一个字节
    for b in "你好,世界!".bytes() {
        println!("{}", b);
    }
    println!();
}

// 主演示函数
pub fn str_demo() {
    println!("🔤 字符串借用规则演示");
    println!("================================");

    str_slice_borrow_ok_last_clear();
    println!();

    str_slice_borrow_ok_clone();
    println!();

    str_slice_borrow_ok_scope();
    println!();

    str_append();
    println!();

    str_replace();
    println!();

    str_delete();
    println!();

    str_concat();
    println!();

    str_escape();
    println!();

    str_utf8();
    println!();

    println!("💡 总结：Rust 的借用规则防止了数据竞争和悬垂引用");
}
