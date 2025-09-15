// 15. 模块

// 基本模块定义
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("添加到等待列表");
        }
        
        fn seat_at_table() {
            println!("安排座位");
        }
    }
    
    mod serving {
        fn take_order() {
            println!("接受订单");
        }
        
        fn serve_order() {
            println!("上菜");
        }
        
        fn take_payment() {
            println!("收银");
        }
    }
}

// 使用 use 关键字
use front_of_house::hosting;

// 相对路径
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    
    pub enum Appetizer {
        Soup,
        Salad,
    }
    
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    
    fn cook_order() {
        println!("烹饪订单");
    }
}

fn deliver_order() {
    println!("送餐");
}

// 模块重导出
mod front_of_house_reexport {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("添加到等待列表（重导出）");
        }
    }
}

pub use front_of_house_reexport::hosting as hosting_reexport;

// 嵌套模块
mod restaurant {
    pub mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {
                println!("餐厅添加到等待列表");
            }
        }
    }
    
    pub mod back_of_house {
        pub struct Breakfast {
            pub toast: String,
            seasonal_fruit: String,
        }
        
        impl Breakfast {
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("peaches"),
                }
            }
        }
    }
}

// 模块可见性
mod visibility {
    pub fn public_function() {
        println!("公共函数");
    }
    
    fn private_function() {
        println!("私有函数");
    }
    
    pub mod nested {
        pub fn public_nested_function() {
            println!("嵌套公共函数");
        }
        
        fn private_nested_function() {
            println!("嵌套私有函数");
        }
    }
}

// 模块与结构体
mod struct_visibility {
    pub struct PublicStruct {
        pub public_field: i32,
        private_field: i32,
    }
    
    impl PublicStruct {
        pub fn new(public: i32, private: i32) -> PublicStruct {
            PublicStruct {
                public_field: public,
                private_field: private,
            }
        }
        
        pub fn get_private(&self) -> i32 {
            self.private_field
        }
    }
    
    pub struct PrivateStruct {
        field: i32,
    }
    
    impl PrivateStruct {
        pub fn new(field: i32) -> PrivateStruct {
            PrivateStruct { field }
        }
        
        pub fn get_field(&self) -> i32 {
            self.field
        }
    }
}

// 模块与枚举
mod enum_visibility {
    pub enum PublicEnum {
        Variant1,
        Variant2,
    }
    
    impl PublicEnum {
        pub fn new_variant1() -> PublicEnum {
            PublicEnum::Variant1
        }
        
        pub fn new_variant2() -> PublicEnum {
            PublicEnum::Variant2
        }
    }
}

// 模块与特征
mod trait_visibility {
    pub trait PublicTrait {
        fn public_method(&self);
        
        fn default_method(&self) {
            println!("默认方法");
        }
    }
    
    pub struct PublicStruct;
    
    impl PublicTrait for PublicStruct {
        fn public_method(&self) {
            println!("实现公共方法");
        }
    }
}

// 模块与泛型
mod generic_visibility {
    pub struct GenericStruct<T> {
        pub value: T,
    }
    
    impl<T> GenericStruct<T> {
        pub fn new(value: T) -> GenericStruct<T> {
            GenericStruct { value }
        }
        
        pub fn get_value(&self) -> &T {
            &self.value
        }
    }
}

// 模块与生命周期
mod lifetime_visibility {
    pub struct LifetimeStruct<'a> {
        pub value: &'a str,
    }
    
    impl<'a> LifetimeStruct<'a> {
        pub fn new(value: &'a str) -> LifetimeStruct<'a> {
            LifetimeStruct { value }
        }
        
        pub fn get_value(&self) -> &'a str {
            self.value
        }
    }
}

// 模块与错误处理
mod error_visibility {
    use std::fmt;
    
    #[derive(Debug)]
    pub enum CustomError {
        NotFound,
        InvalidInput,
    }
    
    impl fmt::Display for CustomError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                CustomError::NotFound => write!(f, "未找到"),
                CustomError::InvalidInput => write!(f, "无效输入"),
            }
        }
    }
    
    impl std::error::Error for CustomError {}
    
    pub fn process_data(data: &str) -> Result<String, CustomError> {
        if data.is_empty() {
            Err(CustomError::InvalidInput)
        } else {
            Ok(data.to_uppercase())
        }
    }
}

// 模块与测试
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_public_function() {
        visibility::public_function();
    }
    
    #[test]
    fn test_nested_public_function() {
        visibility::nested::public_nested_function();
    }
    
    #[test]
    fn test_public_struct() {
        let s = struct_visibility::PublicStruct::new(1, 2);
        assert_eq!(s.public_field, 1);
        assert_eq!(s.get_private(), 2);
    }
    
    #[test]
    fn test_public_enum() {
        let e1 = enum_visibility::PublicEnum::new_variant1();
        let e2 = enum_visibility::PublicEnum::new_variant2();
        match e1 {
            enum_visibility::PublicEnum::Variant1 => println!("变体1"),
            enum_visibility::PublicEnum::Variant2 => println!("变体2"),
        }
    }
    
    #[test]
    fn test_public_trait() {
        use crate::trait_visibility::PublicTrait;
        
        let s = trait_visibility::PublicStruct;
        s.public_method();
        s.default_method();
    }
    
    #[test]
    fn test_generic_struct() {
        let s = generic_visibility::GenericStruct::new(42);
        assert_eq!(*s.get_value(), 42);
    }
    
    #[test]
    fn test_lifetime_struct() {
        let s = "hello";
        let ls = lifetime_visibility::LifetimeStruct::new(s);
        assert_eq!(ls.get_value(), "hello");
    }
    
    #[test]
    fn test_error_handling() {
        let result = error_visibility::process_data("hello");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "HELLO");
        
        let error_result = error_visibility::process_data("");
        assert!(error_result.is_err());
    }
}

pub fn run_demo() {
    println!("📦 模块演示");
    println!("===========");
    
    // 基本模块
    basic_modules();
    println!();
    
    // 模块可见性
    module_visibility();
    println!();
    
    // 模块与结构体
    modules_with_structs();
    println!();
    
    // 模块与枚举
    modules_with_enums();
    println!();
    
    // 模块与特征
    modules_with_traits();
    println!();
    
    // 模块与泛型
    modules_with_generics();
    println!();
    
    // 模块与生命周期
    modules_with_lifetimes();
    println!();
    
    // 模块与错误处理
    modules_with_error_handling();
    println!();
    
    // 模块最佳实践
    module_best_practices();
}

fn basic_modules() {
    println!("=== 基本模块 ===");
    
    // 使用绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    
    // 使用相对路径
    front_of_house::hosting::add_to_waitlist();
    
    // 使用 use 关键字
    hosting::add_to_waitlist();
    
    // 使用重导出
    hosting_reexport::add_to_waitlist();
    
    // 使用嵌套模块
    restaurant::front_of_house::hosting::add_to_waitlist();
}

fn module_visibility() {
    println!("=== 模块可见性 ===");
    
    // 公共函数
    visibility::public_function();
    
    // 嵌套公共函数
    visibility::nested::public_nested_function();
    
    // 私有函数不能直接访问
    // visibility::private_function(); // 编译错误！
    // visibility::nested::private_nested_function(); // 编译错误！
}

fn modules_with_structs() {
    println!("=== 模块与结构体 ===");
    
    // 公共结构体
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("我要 {} 面包", meal.toast);
    
    // 公共枚举
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    
    match order1 {
        back_of_house::Appetizer::Soup => println!("汤"),
        back_of_house::Appetizer::Salad => println!("沙拉"),
    }
    
    // 结构体可见性
    let s = struct_visibility::PublicStruct::new(1, 2);
    println!("公共字段: {}", s.public_field);
    println!("私有字段: {}", s.get_private());
    
    let ps = struct_visibility::PrivateStruct::new(42);
    println!("私有结构体字段: {}", ps.get_field());
}

fn modules_with_enums() {
    println!("=== 模块与枚举 ===");
    
    let e1 = enum_visibility::PublicEnum::new_variant1();
    let e2 = enum_visibility::PublicEnum::new_variant2();
    
    match e1 {
        enum_visibility::PublicEnum::Variant1 => println!("变体1"),
        enum_visibility::PublicEnum::Variant2 => println!("变体2"),
    }
    
    match e2 {
        enum_visibility::PublicEnum::Variant1 => println!("变体1"),
        enum_visibility::PublicEnum::Variant2 => println!("变体2"),
    }
}

fn modules_with_traits() {
    println!("=== 模块与特征 ===");
    
    use crate::trait_visibility::PublicTrait;
    
    let s = trait_visibility::PublicStruct;
    s.public_method();
    s.default_method();
}

fn modules_with_generics() {
    println!("=== 模块与泛型 ===");
    
    let s = generic_visibility::GenericStruct::new(42);
    println!("泛型结构体值: {}", s.get_value());
    
    let s2 = generic_visibility::GenericStruct::new("hello");
    println!("泛型结构体值: {}", s2.get_value());
}

fn modules_with_lifetimes() {
    println!("=== 模块与生命周期 ===");
    
    let s = "hello";
    let ls = lifetime_visibility::LifetimeStruct::new(s);
    println!("生命周期结构体值: {}", ls.get_value());
}

fn modules_with_error_handling() {
    println!("=== 模块与错误处理 ===");
    
    let result = error_visibility::process_data("hello");
    match result {
        Ok(data) => println!("处理结果: {}", data),
        Err(e) => println!("错误: {}", e),
    }
    
    let error_result = error_visibility::process_data("");
    match error_result {
        Ok(data) => println!("处理结果: {}", data),
        Err(e) => println!("错误: {}", e),
    }
}

fn module_best_practices() {
    println!("=== 模块最佳实践 ===");
    
    // 1. 使用 pub 关键字控制可见性
    visibility::public_function();
    
    // 2. 使用 use 关键字简化路径
    hosting::add_to_waitlist();
    
    // 3. 使用 pub use 重导出
    hosting_reexport::add_to_waitlist();
    
    // 4. 模块组织
    restaurant::front_of_house::hosting::add_to_waitlist();
    
    // 5. 结构体字段可见性
    let s = struct_visibility::PublicStruct::new(1, 2);
    println!("公共字段: {}", s.public_field);
    println!("私有字段: {}", s.get_private());
    
    // 6. 枚举变体可见性
    let e = enum_visibility::PublicEnum::new_variant1();
    match e {
        enum_visibility::PublicEnum::Variant1 => println!("变体1"),
        enum_visibility::PublicEnum::Variant2 => println!("变体2"),
    }
    
    // 7. 特征可见性
    use crate::trait_visibility::PublicTrait;
    let s = trait_visibility::PublicStruct;
    s.public_method();
    s.default_method();
    
    // 8. 泛型可见性
    let s = generic_visibility::GenericStruct::new(42);
    println!("泛型值: {}", s.get_value());
    
    // 9. 生命周期可见性
    let s = "hello";
    let ls = lifetime_visibility::LifetimeStruct::new(s);
    println!("生命周期值: {}", ls.get_value());
    
    // 10. 错误处理可见性
    let result = error_visibility::process_data("hello");
    match result {
        Ok(data) => println!("处理结果: {}", data),
        Err(e) => println!("错误: {}", e),
    }
}
