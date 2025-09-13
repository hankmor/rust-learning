// 数组和向量演示模块

// 数组演示
pub fn array_demo() {
    println!("=== 数组演示 ===");
    
    // 创建数组的几种方式
    let a = [1, 2, 3, 4, 5]; // 类型推断
    let b: [i32; 5] = [1, 2, 3, 4, 5]; // 显式类型声明
    let c = [3; 5]; // 创建包含5个3的数组
    
    println!("数组 a: {:?}", a);
    println!("数组 b: {:?}", b);
    println!("数组 c: {:?}", c);
    
    // 访问数组元素
    println!("第一个元素: {}", a[0]);
    println!("最后一个元素: {}", a[a.len() - 1]);
    
    // 数组长度
    println!("数组长度: {}", a.len());
}

// 向量演示
pub fn vector_demo() {
    println!("=== 向量演示 ===");
    
    // 创建向量的几种方式
    let mut v1 = Vec::new(); // 空向量
    let v2 = vec![1, 2, 3, 4, 5]; // 使用宏创建
    let mut v3: Vec<i32> = Vec::new(); // 显式类型声明
    
    // 向向量添加元素
    v1.push(1);
    v1.push(2);
    v1.push(3);
    
    v3.push(10);
    v3.push(20);
    v3.push(30);
    
    println!("向量 v1: {:?}", v1);
    println!("向量 v2: {:?}", v2);
    println!("向量 v3: {:?}", v3);
    
    // 访问向量元素
    println!("v2 的第一个元素: {}", v2[0]);
    println!("v2 的最后一个元素: {}", v2[v2.len() - 1]);
    
    // 使用 get 方法安全访问
    match v2.get(2) {
        Some(value) => println!("v2[2] = {}", value),
        None => println!("索引超出范围"),
    }
    
    // 向量长度和容量
    println!("v2 长度: {}", v2.len());
    println!("v2 容量: {}", v2.capacity());
}

// 切片演示
pub fn slice_demo() {
    println!("=== 切片演示 ===");
    
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 创建切片
    let slice1 = &data[0..5]; // 前5个元素
    let slice2 = &data[5..]; // 从第5个元素到末尾
    let slice3 = &data[2..7]; // 从第2个到第6个元素
    let slice4 = &data[..]; // 整个数组的切片
    
    println!("原数组: {:?}", data);
    println!("切片 [0..5]: {:?}", slice1);
    println!("切片 [5..]: {:?}", slice2);
    println!("切片 [2..7]: {:?}", slice3);
    println!("切片 [..]: {:?}", slice4);
    
    // 字符串切片
    let s = String::from("Hello, World!");
    let hello = &s[0..5];
    let world = &s[7..12];
    
    println!("原字符串: {}", s);
    println!("切片 'Hello': {}", hello);
    println!("切片 'World': {}", world);
}

// 主演示函数
pub fn arrays_demo() {
    println!("📊 数组和向量演示");
    println!("================================");
    
    array_demo();
    println!();
    
    vector_demo();
    println!();
    
    slice_demo();
    println!();
    
    println!("💡 总结：");
    println!("  - 数组：固定长度，栈上分配");
    println!("  - 向量：动态长度，堆上分配");
    println!("  - 切片：对数组或向量的引用");
    println!("  - 索引从0开始，使用[]访问元素");
}
