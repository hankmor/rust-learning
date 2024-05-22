pub mod entrance {

    pub fn demo() {
        // 必须分号结尾，默认不可变
        let a = 10;
        // a = 20; // 编译错误，a不可变，无法继续赋值
        // 显示指定类型为 int32，注意冒号
        let b: i32 = 20;
        // mut 定义可变变量, 整数值可以定义类型，这里表示值为5，类型为i32，整数可以用下划线提升可读性
        let mut c = 5i32;
        let mut d = 50_i32;
        // 将函数的返回值作为参数
        let e = add(add(a, b), add(c, d));
        println!("(a + b) + (c + d) = {}", e);
        let f = add2(add2(a, b), add2(c, d));
        println!("(a + b) + (c + d) = {}", f)
    }

    fn add(a: i32, b: i32) -> i32 {
        return a + b;
    }

    // 直接简写，a = b后不能有分号
    fn add2(a: i32, b: i32) -> i32 {
        a + b // 这里不能带分号, 否则编译错误
    }
}
