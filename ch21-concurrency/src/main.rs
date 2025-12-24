use std::thread;
use std::time::Duration;

fn main() {
    println!("=== 并发编程 (Concurrency) Demo ===");

    // 1. 创建新线程
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 2. 等待线程结束
    handle.join().unwrap();

    // 3. 使用 move 闭包
    let v = vec![1, 2, 3];

    // 强制获取 v 的所有权
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
