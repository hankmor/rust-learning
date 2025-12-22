fn main() {
    println!("=== 所有权 (Ownership) Demo ===");

    // 1. 作用域 (Scope)
    {                      
        let s = "hello";   // s 进入作用域
        println!("Inside scope: {}", s);
    }                      // s 离开作用域，消亡

    // 2. String 类型 (堆上分配)
    let s1 = String::from("hello");
    // let s2 = s1; // 错误！所有权发生了 Move (移动)，s1 失效了
    
    // println!("{}, world!", s1); // 编译错误：borrow of moved value: `s1`
    
    // 正确做法：Clone (深拷贝)
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // 3. Copy Trait (栈上数据拷贝)
    let x = 5;
    let y = x; // 整数实现了 Copy，所以只是拷贝，x 依然有效
    println!("x = {}, y = {}", x, y);

    // 4. 函数传参的所有权转移
    let s = String::from("hello");
    takes_ownership(s); 
    // println!("{}", s); // 错误！s 的所有权已经交给了函数，s 在这里无效了

    let x = 5;
    makes_copy(x); 
    println!("{}", x); // 正确！i32 是 Copy 的

    // 5. 返回值移交所有权
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s1 = {}, s3 = {}", s1, s3);
}

fn takes_ownership(some_string: String) { 
    println!("takes_ownership: {}", some_string);
} // some_string 离开作用域，drop 被调用，内存释放

fn makes_copy(some_integer: i32) { 
    println!("makes_copy: {}", some_integer);
} 

fn gives_ownership() -> String {             
    let some_string = String::from("yours"); 
    some_string                              
}

fn takes_and_gives_back(a_string: String) -> String { 
    a_string  
}
