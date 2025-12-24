# Rust Learning 学习笔记代码库

这是 [HankMo](https://hankmo.com) 的 Rust 学习笔记配套代码库。
本项目包含 30 个章节的示例代码，涵盖了从基础语法到高级特性，以及最终的 Web Server 项目实战。

## 📂 项目结构

本项目使用 Cargo Workspace 组织，包含以下 crate：

### 基础篇
- `ch01-hello-world`: 简介与环境搭建
- `ch02-vars`: 变量与可变性
- `ch03-functions`: 函数
- `ch04-control-flow`: 控制流程
- `ch05-ownership`: 所有权
- `ch06-references`: 引用与借用
- `ch07-slices`: Slice 类型
- `ch08-structs`: 结构体

### 进阶篇
- `ch09-enums`: 枚举与模式匹配
- `ch10-guessing-game`: **项目实战一：猜数字**
- `ch11-modules`: 包与模块
- `ch12-collections`: 常用集合
- `ch13-error-handling`: 错误处理
- `ch14-generics`: 泛型
- `ch15-traits`: Traits
- `ch16-lifetimes`: 生命周期

### 高级篇
- `ch17-testing`: 自动化测试
- `ch18-functional`: 闭包与迭代器
- `ch19-smart-pointers`: 智能指针
- `ch20-minigrep`: **项目实战二：minigrep**
- `ch21-concurrency`: 并发编程
- `ch22-shared-state`: 共享状态并发
- `ch23-oop`: 面向对象特性
- `ch24-patterns`: 模式匹配详情
- `ch25-advanced`: 高级特性 (Unsafe, Advanced Types)
- `ch26-macros`: 宏入门
- `ch27-ffi`: FFI 交互
- `ch28-web-hello`: Web 开发初探 (Axum + Tokio)
- `ch29-async`: 异步编程

### 毕业实战
- `ch30-web-server`: **项目实战三：多线程 Web Server** (基于 TCPListener 手写)

## 🚀 如何运行

你可以进入任意目录运行特定的示例：

```bash
cd ch10-guessing-game
cargo run
```

或者在根目录指定 package 运行：

```bash
cargo run -p ch30-web-server
```

## 📝 学习笔记

所有代码对应的详细讲解文章，请访问博客：[Rust 学习笔记系列](https://hankmo.com/categories/rust/)。

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 License

MIT