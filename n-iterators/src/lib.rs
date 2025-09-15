// 14. 迭代器

// 自定义迭代器
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// 自定义迭代器 - 斐波那契数列
struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

// 自定义迭代器 - 范围
struct Range {
    start: i32,
    end: i32,
    step: i32,
}

impl Range {
    fn new(start: i32, end: i32, step: i32) -> Range {
        Range { start, end, step }
    }
}

impl Iterator for Range {
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let current = self.start;
            self.start += self.step;
            Some(current)
        } else {
            None
        }
    }
}

pub fn run_demo() {
    println!("🔄 迭代器演示");
    println!("=============");
    
    // 基本迭代器
    basic_iterators();
    println!();
    
    // 迭代器适配器
    iterator_adapters();
    println!();
    
    // 迭代器消费者
    iterator_consumers();
    println!();
    
    // 自定义迭代器
    custom_iterators();
    println!();
    
    // 迭代器与闭包
    iterators_with_closures();
    println!();
    
    // 迭代器与错误处理
    iterators_with_error_handling();
    println!();
    
    // 迭代器与生命周期
    iterators_with_lifetimes();
    println!();
    
    // 迭代器性能
    iterator_performance();
    println!();
    
    // 迭代器最佳实践
    iterator_best_practices();
}

fn basic_iterators() {
    println!("=== 基本迭代器 ===");
    
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    
    for val in v1_iter {
        println!("值: {}", val);
    }
    
    // 手动迭代
    let v2 = vec![1, 2, 3];
    let mut v2_iter = v2.iter();
    
    println!("手动迭代:");
    while let Some(val) = v2_iter.next() {
        println!("值: {}", val);
    }
    
    // 可变迭代器
    let mut v3 = vec![1, 2, 3];
    let v3_iter = v3.iter_mut();
    
    for val in v3_iter {
        *val *= 2;
    }
    println!("翻倍后: {:?}", v3);
    
    // 所有权迭代器
    let v4 = vec![1, 2, 3];
    let v4_iter = v4.into_iter();
    
    for val in v4_iter {
        println!("所有权值: {}", val);
    }
    // v4 不再可用
}

fn iterator_adapters() {
    println!("=== 迭代器适配器 ===");
    
    let v1 = vec![1, 2, 3, 4, 5];
    
    // map
    let v2: Vec<i32> = v1.iter().map(|x| x * 2).collect();
    println!("map 翻倍: {:?}", v2);
    
    // filter
    let v3: Vec<&i32> = v1.iter().filter(|&&x| x % 2 == 0).collect();
    println!("filter 偶数: {:?}", v3);
    
    // filter_map
    let v4: Vec<i32> = v1.iter().filter_map(|&x| if x % 2 == 0 { Some(x * 2) } else { None }).collect();
    println!("filter_map 偶数翻倍: {:?}", v4);
    
    // take
    let v5: Vec<&i32> = v1.iter().take(3).collect();
    println!("take 前3个: {:?}", v5);
    
    // skip
    let v6: Vec<&i32> = v1.iter().skip(2).collect();
    println!("skip 跳过前2个: {:?}", v6);
    
    // chain
    let v7: Vec<&i32> = v1.iter().chain([6, 7, 8].iter()).collect();
    println!("chain 连接: {:?}", v7);
    
    // zip
    let v8: Vec<(&i32, &i32)> = v1.iter().zip([10, 20, 30, 40, 50].iter()).collect();
    println!("zip 配对: {:?}", v8);
    
    // enumerate
    let v9: Vec<(usize, &i32)> = v1.iter().enumerate().collect();
    println!("enumerate 枚举: {:?}", v9);
    
    // rev
    let v10: Vec<&i32> = v1.iter().rev().collect();
    println!("rev 反转: {:?}", v10);
    
    // cycle
    let v11: Vec<&i32> = v1.iter().cycle().take(10).collect();
    println!("cycle 循环: {:?}", v11);
}

fn iterator_consumers() {
    println!("=== 迭代器消费者 ===");
    
    let v1 = vec![1, 2, 3, 4, 5];
    
    // collect
    let v2: Vec<i32> = v1.iter().map(|&x| x * 2).collect();
    println!("collect 收集: {:?}", v2);
    
    // fold
    let sum: i32 = v1.iter().fold(0, |acc, &x| acc + x);
    println!("fold 求和: {}", sum);
    
    // reduce
    let max = v1.iter().reduce(|acc, &x| if x > *acc { acc } else { acc });
    println!("reduce 最大值: {:?}", max);
    
    // any
    let has_even = v1.iter().any(|&x| x % 2 == 0);
    println!("any 有偶数: {}", has_even);
    
    // all
    let all_positive = v1.iter().all(|&x| x > 0);
    println!("all 都为正数: {}", all_positive);
    
    // find
    let first_even = v1.iter().find(|&&x| x % 2 == 0);
    println!("find 第一个偶数: {:?}", first_even);
    
    // position
    let pos = v1.iter().position(|&x| x == 3);
    println!("position 3的位置: {:?}", pos);
    
    // count
    let count = v1.iter().count();
    println!("count 元素个数: {}", count);
    
    // last
    let last = v1.iter().last();
    println!("last 最后一个: {:?}", last);
    
    // nth
    let nth = v1.iter().nth(2);
    println!("nth 第3个: {:?}", nth);
    
    // max/min
    let max = v1.iter().max();
    let min = v1.iter().min();
    println!("max: {:?}, min: {:?}", max, min);
    
    // sum
    let sum: i32 = v1.iter().sum();
    println!("sum 求和: {}", sum);
    
    // product
    let product: i32 = v1.iter().product();
    println!("product 乘积: {}", product);
}

fn custom_iterators() {
    println!("=== 自定义迭代器 ===");
    
    // Counter 迭代器
    let mut counter = Counter::new();
    println!("Counter 迭代器:");
    while let Some(count) = counter.next() {
        println!("  计数: {}", count);
    }
    
    // 使用 for 循环
    println!("Counter for 循环:");
    for count in Counter::new() {
        println!("  计数: {}", count);
    }
    
    // 斐波那契迭代器
    println!("斐波那契数列:");
    for (i, fib) in Fibonacci::new().take(10).enumerate() {
        println!("  F{} = {}", i + 1, fib);
    }
    
    // 范围迭代器
    println!("范围迭代器:");
    for i in Range::new(0, 10, 2) {
        println!("  范围值: {}", i);
    }
    
    // 自定义迭代器方法
    let sum: u32 = Counter::new().sum();
    println!("Counter 求和: {}", sum);
    
    let doubled: Vec<u32> = Counter::new().map(|x| x * 2).collect();
    println!("Counter 翻倍: {:?}", doubled);
}

fn iterators_with_closures() {
    println!("=== 迭代器与闭包 ===");
    
    let v1 = vec![1, 2, 3, 4, 5];
    
    // 闭包捕获环境
    let multiplier = 2;
    let v2: Vec<i32> = v1.iter().map(|&x| x * multiplier).collect();
    println!("闭包捕获环境: {:?}", v2);
    
    // 闭包作为参数
    let v3: Vec<i32> = v1.iter().map(|&x| x.pow(2)).collect();
    println!("闭包平方: {:?}", v3);
    
    // 闭包与 filter
    let threshold = 3;
    let v4: Vec<&i32> = v1.iter().filter(|&&x| x > threshold).collect();
    println!("闭包过滤: {:?}", v4);
    
    // 闭包与 fold
    let initial = 0;
    let sum: i32 = v1.iter().fold(initial, |acc, &x| acc + x);
    println!("闭包折叠: {}", sum);
}

fn iterators_with_error_handling() {
    println!("=== 迭代器与错误处理 ===");
    
    let numbers = vec!["1", "2", "3", "not_a_number", "5"];
    
    // 使用 map 处理错误
    let results: Vec<Result<i32, _>> = numbers
        .iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("解析结果: {:?}", results);
    
    // 使用 filter_map 过滤错误
    let valid_numbers: Vec<i32> = numbers
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("有效数字: {:?}", valid_numbers);
    
    // 使用 try_fold 处理错误
    let sum_result: Result<i32, std::num::ParseIntError> = numbers
        .iter()
        .try_fold(0, |acc, s| {
            let num = s.parse::<i32>()?;
            Ok(acc + num)
        });
    println!("求和结果: {:?}", sum_result);
}

fn iterators_with_lifetimes() {
    println!("=== 迭代器与生命周期 ===");
    
    let s = String::from("hello world");
    let words: Vec<&str> = s.split_whitespace().collect();
    println!("单词: {:?}", words);
    
    // 迭代器返回的引用与原始数据有相同的生命周期
    let first_word = s.split_whitespace().next();
    println!("第一个单词: {:?}", first_word);
    println!("原始字符串: {}", s);
}

fn iterator_performance() {
    println!("=== 迭代器性能 ===");
    
    let v1 = vec![1, 2, 3, 4, 5];
    
    // 迭代器是零成本抽象
    let sum: i32 = v1.iter().sum();
    println!("迭代器求和: {}", sum);
    
    // 链式调用
    let result: Vec<i32> = v1
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * 2)
        .collect();
    println!("链式调用结果: {:?}", result);
    
    // 惰性求值
    let lazy_iter = v1.iter().map(|&x| {
        println!("处理: {}", x);
        x * 2
    });
    
    println!("惰性迭代器创建完成，但还没有执行");
    
    let result: Vec<i32> = lazy_iter.collect();
    println!("收集后结果: {:?}", result);
}

fn iterator_best_practices() {
    println!("=== 迭代器最佳实践 ===");
    
    let v1 = vec![1, 2, 3, 4, 5];
    
    // 1. 使用迭代器而不是循环
    let sum: i32 = v1.iter().sum();
    println!("使用迭代器求和: {}", sum);
    
    // 2. 链式调用
    let result: Vec<i32> = v1
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * 2)
        .collect();
    println!("链式调用: {:?}", result);
    
    // 3. 使用 collect 收集结果
    let doubled: Vec<i32> = v1.iter().map(|&x| x * 2).collect();
    println!("收集结果: {:?}", doubled);
    
    // 4. 使用 find 而不是 filter().next()
    let first_even = v1.iter().find(|&&x| x % 2 == 0);
    println!("第一个偶数: {:?}", first_even);
    
    // 5. 使用 any/all 进行条件检查
    let has_even = v1.iter().any(|&x| x % 2 == 0);
    let all_positive = v1.iter().all(|&x| x > 0);
    println!("有偶数: {}, 都为正数: {}", has_even, all_positive);
    
    // 6. 使用 enumerate 获取索引
    let indexed: Vec<(usize, &i32)> = v1.iter().enumerate().collect();
    println!("带索引: {:?}", indexed);
    
    // 7. 使用 zip 组合迭代器
    let combined: Vec<(&i32, &i32)> = v1.iter().zip([10, 20, 30, 40, 50].iter()).collect();
    println!("组合: {:?}", combined);
}
