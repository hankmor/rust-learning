// 16. 包和 crate

// 基本 crate 结构
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("添加到等待列表");
        }
    }
}

// 使用 use 关键字
use front_of_house::hosting;

// 重导出
pub use front_of_house::hosting as hosting_reexport;

// 库 crate 示例
pub mod library {
    pub fn public_function() {
        println!("库的公共函数");
    }
    
    fn private_function() {
        println!("库的私有函数");
    }
    
    pub mod nested {
        pub fn public_nested_function() {
            println!("嵌套公共函数");
        }
    }
}

// 二进制 crate 示例
mod binary_crate {
    pub fn run() {
        println!("二进制 crate 运行");
    }
}

// 包结构示例
mod package_structure {
    pub mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {
                println!("包结构：添加到等待列表");
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

// 依赖管理示例
mod dependency_management {
    use std::collections::HashMap;
    
    pub fn demonstrate_dependencies() {
        let mut map = HashMap::new();
        map.insert("key", "value");
        println!("依赖管理示例: {:?}", map);
    }
}

// 特征和实现示例
mod traits_and_implementations {
    pub trait Drawable {
        fn draw(&self);
    }
    
    pub struct Circle {
        pub radius: f64,
    }
    
    impl Drawable for Circle {
        fn draw(&self) {
            println!("绘制半径为 {} 的圆", self.radius);
        }
    }
    
    pub struct Rectangle {
        pub width: f64,
        pub height: f64,
    }
    
    impl Drawable for Rectangle {
        fn draw(&self) {
            println!("绘制 {}x{} 的矩形", self.width, self.height);
        }
    }
}

// 错误处理示例
mod error_handling {
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

// 测试模块
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_library_function() {
        library::public_function();
    }
    
    #[test]
    fn test_nested_function() {
        library::nested::public_nested_function();
    }
    
    #[test]
    fn test_binary_crate() {
        binary_crate::run();
    }
    
    #[test]
    fn test_package_structure() {
        package_structure::front_of_house::hosting::add_to_waitlist();
    }
    
    #[test]
    fn test_dependency_management() {
        dependency_management::demonstrate_dependencies();
    }
    
    #[test]
    fn test_traits_and_implementations() {
        use crate::traits_and_implementations::Drawable;
        
        let circle = traits_and_implementations::Circle { radius: 5.0 };
        circle.draw();
        
        let rectangle = traits_and_implementations::Rectangle { width: 10.0, height: 20.0 };
        rectangle.draw();
    }
    
    #[test]
    fn test_error_handling() {
        let result = error_handling::process_data("hello");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "HELLO");
        
        let error_result = error_handling::process_data("");
        assert!(error_result.is_err());
    }
}

// 文档注释示例
/// 这是一个公共函数的文档注释
/// 
/// # 示例
/// 
/// ```
/// # use p_crates::public_function_with_docs;
/// let result = public_function_with_docs();
/// println!("结果: {}", result);
/// ```
pub fn public_function_with_docs() -> String {
    String::from("这是一个有文档的函数")
}

/// 计算两个数的和
/// 
/// # 参数
/// 
/// * `a` - 第一个数
/// * `b` - 第二个数
/// 
/// # 返回值
/// 
/// 返回两个数的和
/// 
/// # 示例
/// 
/// ```
/// # use p_crates::add;
/// let sum = add(2, 3);
/// assert_eq!(sum, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 计算两个数的乘积
/// 
/// # 参数
/// 
/// * `a` - 第一个数
/// * `b` - 第二个数
/// 
/// # 返回值
/// 
/// 返回两个数的乘积
/// 
/// # 示例
/// 
/// ```
/// # use p_crates::multiply;
/// let product = multiply(2, 3);
/// assert_eq!(product, 6);
/// ```
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// 条件编译示例
#[cfg(target_os = "windows")]
fn windows_specific_function() {
    println!("这是 Windows 特定的函数");
}

#[cfg(target_os = "linux")]
fn linux_specific_function() {
    println!("这是 Linux 特定的函数");
}

#[cfg(target_os = "macos")]
fn macos_specific_function() {
    println!("这是 macOS 特定的函数");
}

// 特性标志示例
#[cfg(feature = "advanced")]
mod advanced_features {
    pub fn advanced_function() {
        println!("高级功能");
    }
}

// 工作空间示例
mod workspace_example {
    pub fn workspace_function() {
        println!("工作空间函数");
    }
}

// 发布示例
mod publish_example {
    pub fn publish_function() {
        println!("发布函数");
    }
}

// 版本管理示例
mod version_management {
    pub fn version_function() {
        println!("版本管理函数");
    }
}

// 依赖版本示例
mod dependency_versions {
    pub fn dependency_version_function() {
        println!("依赖版本函数");
    }
}

// 构建脚本示例
mod build_script_example {
    pub fn build_script_function() {
        println!("构建脚本函数");
    }
}

// 发布配置示例
mod publish_configuration {
    pub fn publish_configuration_function() {
        println!("发布配置函数");
    }
}

pub fn run_demo() {
    println!("📦 包和 crate 演示");
    println!("==================");
    
    // 基本 crate
    basic_crates();
    println!();
    
    // 库 crate
    library_crates();
    println!();
    
    // 二进制 crate
    binary_crates();
    println!();
    
    // 包结构
    package_structures();
    println!();
    
    // 依赖管理
    dependency_managements();
    println!();
    
    // 特征和实现
    traits_and_implementations_demo();
    println!();
    
    // 错误处理
    error_handling_demo();
    println!();
    
    // 文档注释
    documentation_comments();
    println!();
    
    // 条件编译
    conditional_compilation();
    println!();
    
    // 特性标志
    feature_flags();
    println!();
    
    // 工作空间
    workspace_demo();
    println!();
    
    // 发布
    publish_demo();
    println!();
    
    // 版本管理
    version_management_demo();
    println!();
    
    // 依赖版本
    dependency_versions_demo();
    println!();
    
    // 构建脚本
    build_script_demo();
    println!();
    
    // 发布配置
    publish_configuration_demo();
    println!();
    
    // 最佳实践
    best_practices();
}

fn basic_crates() {
    println!("=== 基本 crate ===");
    
    // 使用绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    
    // 使用相对路径
    front_of_house::hosting::add_to_waitlist();
    
    // 使用 use 关键字
    hosting::add_to_waitlist();
    
    // 使用重导出
    hosting_reexport::add_to_waitlist();
}

fn library_crates() {
    println!("=== 库 crate ===");
    
    library::public_function();
    library::nested::public_nested_function();
    
    // 私有函数不能直接访问
    // library::private_function(); // 编译错误！
}

fn binary_crates() {
    println!("=== 二进制 crate ===");
    
    binary_crate::run();
}

fn package_structures() {
    println!("=== 包结构 ===");
    
    package_structure::front_of_house::hosting::add_to_waitlist();
    
    let mut meal = package_structure::back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("我要 {} 面包", meal.toast);
}

fn dependency_managements() {
    println!("=== 依赖管理 ===");
    
    dependency_management::demonstrate_dependencies();
}

fn traits_and_implementations_demo() {
    println!("=== 特征和实现 ===");
    
    use crate::traits_and_implementations::Drawable;
    
    let circle = traits_and_implementations::Circle { radius: 5.0 };
    circle.draw();
    
    let rectangle = traits_and_implementations::Rectangle { width: 10.0, height: 20.0 };
    rectangle.draw();
}

fn error_handling_demo() {
    println!("=== 错误处理 ===");
    
    let result = error_handling::process_data("hello");
    match result {
        Ok(data) => println!("处理结果: {}", data),
        Err(e) => println!("错误: {}", e),
    }
    
    let error_result = error_handling::process_data("");
    match error_result {
        Ok(data) => println!("处理结果: {}", data),
        Err(e) => println!("错误: {}", e),
    }
}

fn documentation_comments() {
    println!("=== 文档注释 ===");
    
    let result = public_function_with_docs();
    println!("文档函数结果: {}", result);
    
    let sum = add(2, 3);
    println!("加法结果: {}", sum);
    
    let product = multiply(4, 5);
    println!("乘法结果: {}", product);
}

fn conditional_compilation() {
    println!("=== 条件编译 ===");
    
    #[cfg(target_os = "windows")]
    windows_specific_function();
    
    #[cfg(target_os = "linux")]
    linux_specific_function();
    
    #[cfg(target_os = "macos")]
    macos_specific_function();
}

fn feature_flags() {
    println!("=== 特性标志 ===");
    
    #[cfg(feature = "advanced")]
    advanced_features::advanced_function();
    
    #[cfg(not(feature = "advanced"))]
    println!("高级功能未启用");
}

fn workspace_demo() {
    println!("=== 工作空间 ===");
    
    workspace_example::workspace_function();
}

fn publish_demo() {
    println!("=== 发布 ===");
    
    publish_example::publish_function();
}

fn version_management_demo() {
    println!("=== 版本管理 ===");
    
    version_management::version_function();
}

fn dependency_versions_demo() {
    println!("=== 依赖版本 ===");
    
    dependency_versions::dependency_version_function();
}

fn build_script_demo() {
    println!("=== 构建脚本 ===");
    
    build_script_example::build_script_function();
}

fn publish_configuration_demo() {
    println!("=== 发布配置 ===");
    
    publish_configuration::publish_configuration_function();
}

fn best_practices() {
    println!("=== 最佳实践 ===");
    
    // 1. 使用 pub 关键字控制可见性
    library::public_function();
    
    // 2. 使用 use 关键字简化路径
    hosting::add_to_waitlist();
    
    // 3. 使用 pub use 重导出
    hosting_reexport::add_to_waitlist();
    
    // 4. 组织模块结构
    package_structure::front_of_house::hosting::add_to_waitlist();
    
    // 5. 使用特征和实现
    use crate::traits_and_implementations::Drawable;
    let circle = traits_and_implementations::Circle { radius: 5.0 };
    circle.draw();
    
    // 6. 错误处理
    let result = error_handling::process_data("hello");
    match result {
        Ok(data) => println!("处理结果: {}", data),
        Err(e) => println!("错误: {}", e),
    }
    
    // 7. 文档注释
    let result = public_function_with_docs();
    println!("文档函数结果: {}", result);
    
    // 8. 条件编译
    #[cfg(target_os = "windows")]
    windows_specific_function();
    
    // 9. 特性标志
    #[cfg(feature = "advanced")]
    advanced_features::advanced_function();
    
    // 10. 工作空间
    workspace_example::workspace_function();
    
    // 11. 发布
    publish_example::publish_function();
    
    // 12. 版本管理
    version_management::version_function();
    
    // 13. 依赖版本
    dependency_versions::dependency_version_function();
    
    // 14. 构建脚本
    build_script_example::build_script_function();
    
    // 15. 发布配置
    publish_configuration::publish_configuration_function();
}
