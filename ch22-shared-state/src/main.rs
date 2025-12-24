use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("=== 共享状态并发 (Shared-State Concurrency) Demo ===");

    // 1. 单线程中的 Mutex
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    } // 锁在这里自动释放

    println!("m = {:?}", m);

    // 2. 多线程共享 Mutex (使用 Arc)
    // Counter 是一个被 Mutex 保护的整数
    // 为了在多线程间共享所有权，我们需要用 Arc (Atomic Reference Counting) 包裹住 Mutex
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // 克隆 Arc，增加引用计数
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
