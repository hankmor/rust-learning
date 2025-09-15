// Rust 学习主程序 - 运行所有示例

fn main() {
    println!("🚀 Rust 学习示例集合");
    println!("=====================");
    println!();

    // 01. Hello World
    println!("📝 01. Hello World 示例");
    println!("------------------------");
    a_hello_world::run_demo();
    println!();

    // 02. 变量和类型
    println!("📝 02. 变量和类型示例");
    println!("------------------------");
    b_vars_and_types::run_demo();
    println!();

    // 03. 函数
    println!("📝 03. 函数示例");
    println!("------------------------");
    c_functions::run_demo();
    println!();

    // 04. 控制流
    println!("📝 04. 控制流示例");
    println!("------------------------");
    d_control_flow::run_demo();
    println!();

    // 05. 所有权
    println!("📝 05. 所有权示例");
    println!("------------------------");
    e_ownership::run_demo();
    println!();

    // 06. 结构体
    println!("📝 06. 结构体示例");
    println!("------------------------");
    f_structs::run_demo();
    println!();

    // 07. 枚举
    println!("📝 07. 枚举示例");
    println!("------------------------");
    g_enums::run_demo();
    println!();

    // 08. 集合
    println!("📝 08. 集合示例");
    println!("------------------------");
    h_collections::run_demo();
    println!();

    // 09. 错误处理
    println!("📝 09. 错误处理示例");
    println!("------------------------");
    i_error_handling::run_demo();
    println!();

    // 10. 泛型
    println!("📝 10. 泛型示例");
    println!("------------------------");
    j_generics::run_demo();
    println!();

    // 11. 特征
    println!("📝 11. 特征示例");
    println!("------------------------");
    k_traits::run_demo();
    println!();

    // 12. 生命周期
    println!("📝 12. 生命周期示例");
    println!("------------------------");
    l_lifetimes::run_demo();
    println!();

    // 13. 闭包
    println!("📝 13. 闭包示例");
    println!("------------------------");
    m_closures::run_demo();
    println!();

    // 14. 迭代器
    println!("📝 14. 迭代器示例");
    println!("------------------------");
    n_iterators::run_demo();
    println!();

    // 15. 模块
    println!("📝 15. 模块示例");
    println!("------------------------");
    o_modules::run_demo();
    println!();

    // 16. 包和 crate
    println!("📝 16. 包和 crate 示例");
    println!("------------------------");
    p_crates::run_demo();
    println!();

    println!("🎉 所有示例运行完成！");
    println!("💡 提示：你可以使用以下命令单独运行每个示例：");
    println!("   cargo run -p a-hello-world");
    println!("   cargo run -p b-vars-and-types");
    println!("   cargo run -p c-functions");
    println!("   ... 等等");
}