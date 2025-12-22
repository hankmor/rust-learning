// 将库 crate 引入作用域
// 注意：在 main.rs 中使用 lib.rs 定义的内容，必须把 lib.rs 当作外部 crate
// 即使它们在同一个包 (package) 里。
// 包名在 Cargo.toml 中定义为 "ch11-modules"
// 但是 Rust 不允许 crate 名称包含减号，会自动转换为下划线 "ch11_modules"

use ch11_modules::eat_at_restaurant;
use ch11_modules::eat_breakfast;
use ch11_modules::front_of_house::hosting;

fn main() {
    println!("=== 包与模块 (Packages & Modules) Demo ===");
    
    eat_at_restaurant();
    eat_breakfast();

    hosting::add_to_waitlist();
}
