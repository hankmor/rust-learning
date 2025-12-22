use std::fs::File;
use std::io::{self, Read};
use std::io::ErrorKind;

fn main() {
    println!("=== 错误处理 (Error Handling) Demo ===");

    // 1. Unrecoverable Errors with panic!
    // panic!("crash and burn"); // 取消注释这行来尝试 panic

    // 2. Recoverable Errors with Result
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
    println!("File opened successfully: {:?}", f);
    
    // 清理创建的文件
    std::fs::remove_file("hello.txt").unwrap_or(());

    // 3. Shortcuts for Panic on Error: unwrap and expect
    // let f = File::open("hello.txt").unwrap(); // 如果出错直接 panic
    // let f = File::open("hello.txt").expect("Failed to open hello.txt"); 

    // 4. Propagating Errors (传播错误)
    match read_username_from_file() {
        Ok(s) => println!("Read username: {}", s),
        Err(e) => println!("Error reading username: {}", e),
    }

    // 5. ? Operator (传播错误的语法糖)
    match read_username_from_file_short() {
        Ok(s) => println!("Read username (short): {}", s),
        Err(e) => println!("Error reading username (short): {}", e),
    }
}

// 手动传播错误
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 使用 ? 运算符传播错误
fn read_username_from_file_short() -> Result<String, io::Error> {
    // try to open file, if failed, return Err immediately
    let mut f = File::open("hello.txt")?; 
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
    
    // 甚至可以链式调用：
    // File::open("hello.txt")?.read_to_string(&mut s)?;
}
