use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("=== 异步编程 (Async) Demo ===");

    // 1. 基本的 async/await
    let future1 = do_something();
    let future2 = do_something_else();

    println!("Futures created, but not executed yet.");
    
    // .await 才会驱动 Future 执行
    future1.await;
    future2.await;

    // 2. 并发执行 (tokio::spawn)
    // 类似于 Go 的 go func()
    let handle = tokio::spawn(async {
        for i in 1..5 {
            println!("Spawned task: {}", i);
            sleep(Duration::from_millis(50)).await;
        }
        "Task Done"
    });

    for i in 1..3 {
        println!("Main task: {}", i);
        sleep(Duration::from_millis(50)).await;
    }

    // 等待 spawn 的任务完成
    let result = handle.await.unwrap();
    println!("Spawned task result: {}", result);

    // 3. tokio::select!
    // 同时等待多个 Future，处理最先完成的那个
    tokio::select! {
        _ = sleep(Duration::from_millis(100)) => {
            println!("Timer 1 done");
        }
        _ = sleep(Duration::from_millis(200)) => {
            println!("Timer 2 done");
        }
    }
}

async fn do_something() {
    println!("Doing something...");
    sleep(Duration::from_millis(100)).await;
    println!("Done something.");
}

async fn do_something_else() {
    println!("Doing something else...");
    sleep(Duration::from_millis(100)).await;
    println!("Done something else.");
}
