# 初识Rust

这个示例简单地尝试了 rust 编程，包括三个工程：

- hello-lib: 一个 rust 库，被 `hello-rust` 依赖
- hello-rust: hello world，依赖 `hello-lib` 调用其 `add` 函数
- hello-rust-plus: 一个更复杂的 hello-world 示例
