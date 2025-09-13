// Rust vs Go 可见性控制对比演示

// Rust 结构体可见性演示
pub struct PublicUser {
    pub username: String,    // 公开字段
    pub email: String,       // 公开字段
    sign_in_count: u64,      // 私有字段（模块内可见）
    active: bool,            // 私有字段（模块内可见）
}

// 私有结构体（模块内可见）
struct PrivateUser {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 公开结构体，但字段有不同可见性
pub struct MixedVisibilityUser {
    pub username: String,    // 公开
    pub email: String,       // 公开
    pub(crate) internal_id: u64,  // crate 内可见
    sign_in_count: u64,      // 模块内可见
    active: bool,            // 模块内可见
}

impl PublicUser {
    // 公开关联函数
    pub fn new(username: String, email: String) -> PublicUser {
        PublicUser {
            username,
            email,
            sign_in_count: 0,
            active: true,
        }
    }
    
    // 公开方法
    pub fn get_username(&self) -> &str {
        &self.username
    }
    
    pub fn get_email(&self) -> &str {
        &self.email
    }
    
    pub fn is_active(&self) -> bool {
        self.active
    }
    
    pub fn sign_in(&mut self) {
        self.increment_sign_in();
    }
    
    // 私有方法（模块内可见）
    fn increment_sign_in(&mut self) {
        self.sign_in_count += 1;
    }
    
    fn validate_email(&self) -> bool {
        self.email.contains('@')
    }
}

impl PrivateUser {
    // 私有结构体的方法
    fn new(username: String, email: String) -> PrivateUser {
        PrivateUser {
            username,
            email,
            sign_in_count: 0,
            active: true,
        }
    }
    
    fn get_username(&self) -> &str {
        &self.username
    }
}

// 模块级别的可见性演示
mod visibility_demo {
    use super::*;
    
    pub fn demonstrate_visibility() {
        println!("=== Rust 可见性控制演示 ===");
        
        // 可以访问公开结构体
        let mut public_user = PublicUser::new(
            String::from("alice"),
            String::from("alice@example.com")
        );
        
        // 可以访问公开字段
        println!("用户名: {}", public_user.username);
        println!("邮箱: {}", public_user.email);
        
        // 可以访问公开方法
        println!("用户名: {}", public_user.get_username());
        println!("是否活跃: {}", public_user.is_active());
        
        // 可以访问私有字段（在同一模块内）
        println!("登录次数: {}", public_user.sign_in_count);
        println!("活跃状态: {}", public_user.active);
        
        // 可以调用公开方法
        public_user.sign_in();
        println!("登录后次数: {}", public_user.sign_in_count);
        
        // 可以创建私有结构体（在同一模块内）
        let private_user = PrivateUser::new(
            String::from("bob"),
            String::from("bob@example.com")
        );
        println!("私有用户: {}", private_user.get_username());
        
        // 演示不同可见性级别的结构体
        let mixed_user = MixedVisibilityUser {
            username: String::from("charlie"),
            email: String::from("charlie@example.com"),
            internal_id: 12345,
            sign_in_count: 0,
            active: true,
        };
        
        println!("混合可见性用户: {}", mixed_user.username);
        println!("内部ID: {}", mixed_user.internal_id);
        println!("登录次数: {}", mixed_user.sign_in_count);
    }
}

// 跨模块可见性演示
pub mod cross_module_demo {
    use super::*;
    
    pub fn demonstrate_cross_module() {
        println!("=== 跨模块可见性演示 ===");
        
        // 可以访问公开结构体
        let user = PublicUser::new(
            String::from("david"),
            String::from("david@example.com")
        );
        
        // 可以访问公开字段和方法
        println!("用户名: {}", user.username);
        println!("邮箱: {}", user.get_email());
        
        // 无法访问私有字段（编译错误）
        // println!("登录次数: {}", user.sign_in_count);  // 编译错误！
        // println!("活跃状态: {}", user.active);        // 编译错误！
        
        // 无法访问私有方法（编译错误）
        // user.increment_sign_in();  // 编译错误！
        // user.validate_email();     // 编译错误！
        
        // 无法创建私有结构体（编译错误）
        // let private_user = PrivateUser::new(...);  // 编译错误！
    }
}

// 可见性级别总结
pub fn visibility_summary() {
    println!("=== Rust 可见性级别总结 ===");
    println!("1. pub: 公开，任何地方都可以访问");
    println!("2. pub(crate): crate 内可见");
    println!("3. pub(super): 父模块可见");
    println!("4. pub(in path): 指定路径内可见");
    println!("5. 无修饰符: 模块内可见（私有）");
    println!();
    
    println!("=== Go 可见性级别总结 ===");
    println!("1. 首字母大写: 公开，包外可见");
    println!("2. 首字母小写: 私有，包内可见");
    println!();
    
    println!("=== 主要区别 ===");
    println!("Rust: 基于关键字和路径的细粒度控制");
    println!("Go: 基于命名约定的简单控制");
    println!("Rust: 编译时检查，更安全");
    println!("Go: 运行时检查，更灵活");
}

// 主演示函数
pub fn visibility_comparison_demo() {
    println!("🔒 Rust vs Go 可见性控制对比");
    println!("================================");
    
    visibility_demo::demonstrate_visibility();
    println!();
    
    cross_module_demo::demonstrate_cross_module();
    println!();
    
    visibility_summary();
}
