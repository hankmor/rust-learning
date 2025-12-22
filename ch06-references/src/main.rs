fn main() {
    println!("=== 引用与借用 (References & Borrowing) Demo ===");

    // 1. 不可变引用 (Immutable Reference)
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 传引用，不转移所有权
    println!("The length of '{}' is {}.", s1, len); // s1 依然有效

    // 2. 可变引用 (Mutable Reference)
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("Changed s2: {}", s2);

    // 3. 数据竞争与借用规则
    let mut s = String::from("hello");

    // 规则一：在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用。
    // let r1 = &mut s;
    // let r2 = &mut s; // 错误！不能同时存在两个可变引用
    // println!("{}, {}", r1, r2);

    // 规则二：不可变引用和可变引用不能并存
    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    // let r3 = &mut s; // 错误！已经有了 r1, r2 (immutable borrow)，不能再有 r3 (mutable borrow)
    println!("{} and {}", r1, r2);
    // r1 和 r2 的作用域在这里结束（因为后面不再使用了 - NLL Non-Lexical Lifetimes）

    let r3 = &mut s; // 这里可以，因为 r1, r2 不再使用了
    println!("r3: {}", r3);

    // 4. 悬垂引用 (Dangling Reference)
    // let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize { // s 是对 String 的引用
    s.len()
} // s 离开作用域，但因为它不拥有所有权，所以什么也没发生

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
fn dangle() -> &String { // 试图返回引用
    let s = String::from("hello"); // s 是新创建的
    &s // 返回 s 的引用
} // s 离开作用域，内存被释放 -> 引用指向无效内存 -> 编译器报错
*/
