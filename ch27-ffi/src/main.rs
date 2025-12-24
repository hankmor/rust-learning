use std::ffi::CString;
use std::os::raw::{c_char, c_int};

// 1. 调用 C 函数 (Linking to libc)
// 告诉 Rust 编译器：这几个函数是在外部定义的，链接的时候去找它们。
extern "C" {
    fn abs(input: i32) -> i32;
    fn cos(input: f64) -> f64;
    fn puts(s: *const c_char) -> c_int;
}

fn main() {
    println!("=== FFI 交互 (FFI) Demo ===");

    // 调用 C 的函数
    unsafe {
        let abs_val = abs(-3);
        println!("abs(-3) from C: {}", abs_val);

        let cos_val = cos(0.0);
        println!("cos(0.0) from C: {}", cos_val);

        // 调用 puts 需要 C 风格的字符串 (以 null 结尾)
        let c_to_print = CString::new("Hello from C puts!").expect("CString::new failed");
        puts(c_to_print.as_ptr());
    }

    // 调用被导出的 Rust 函数
    call_from_c();
}

// 2. 暴露给 C 函数
// #[no_mangle] 禁止编译器修改函数名
// extern "C" 使用 C 的 ABI (Application Binary Interface)
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
