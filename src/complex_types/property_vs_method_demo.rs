// 结构体属性 vs 方法演示模块

#[derive(Debug)]
struct BankAccount {
    // 属性（字段）
    account_number: String,
    balance: f64,
    owner_name: String,
    is_active: bool,
}

impl BankAccount {
    // 关联函数（类似构造函数）
    fn new(account_number: String, owner_name: String) -> BankAccount {
        BankAccount {
            account_number,
            balance: 0.0,
            owner_name,
            is_active: true,
        }
    }
    
    // 方法：获取余额
    fn get_balance(&self) -> f64 {
        self.balance  // 访问属性
    }
    
    // 方法：存款
    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;  // 修改属性
            println!("存款成功：{} 元", amount);
        } else {
            println!("存款金额必须大于0");
        }
    }
    
    // 方法：取款
    fn withdraw(&mut self, amount: f64) -> bool {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;  // 修改属性
            println!("取款成功：{} 元", amount);
            true
        } else {
            println!("取款失败：余额不足或金额无效");
            false
        }
    }
    
    // 方法：获取账户信息
    fn get_account_info(&self) -> String {
        format!(
            "账户：{}，户主：{}，余额：{:.2} 元，状态：{}",
            self.account_number,
            self.owner_name,
            self.balance,
            if self.is_active { "活跃" } else { "冻结" }
        )
    }
    
    // 方法：冻结账户
    fn freeze_account(&mut self) {
        self.is_active = false;  // 修改属性
        println!("账户已被冻结");
    }
    
    // 方法：激活账户
    fn activate_account(&mut self) {
        self.is_active = true;  // 修改属性
        println!("账户已激活");
    }
}

// 属性直接访问演示
pub fn property_access_demo() {
    println!("=== 属性直接访问演示 ===");
    
    let mut account = BankAccount::new(
        String::from("1234567890"),
        String::from("张三")
    );
    
    // 直接访问属性
    println!("账户号：{}", account.account_number);
    println!("户主：{}", account.owner_name);
    println!("余额：{}", account.balance);
    println!("状态：{}", account.is_active);
    
    // 直接修改属性（不推荐）
    account.balance = 1000.0;
    account.is_active = false;
    
    println!("直接修改后：{:?}", account);
}

// 方法调用演示
pub fn method_call_demo() {
    println!("=== 方法调用演示 ===");
    
    let mut account = BankAccount::new(
        String::from("9876543210"),
        String::from("李四")
    );
    
    // 使用方法获取信息
    println!("初始余额：{}", account.get_balance());
    println!("账户信息：{}", account.get_account_info());
    
    // 使用方法修改状态
    account.deposit(500.0);
    account.deposit(300.0);
    println!("存款后余额：{}", account.get_balance());
    
    let success = account.withdraw(200.0);
    if success {
        println!("取款后余额：{}", account.get_balance());
    }
    
    account.freeze_account();
    println!("冻结后信息：{}", account.get_account_info());
}

// 属性 vs 方法对比演示
pub fn property_vs_method_comparison() {
    println!("=== 属性 vs 方法对比演示 ===");
    
    let mut account = BankAccount::new(
        String::from("1111111111"),
        String::from("王五")
    );
    
    println!("--- 直接访问属性 ---");
    // 直接访问属性
    println!("账户号：{}", account.account_number);
    println!("户主：{}", account.owner_name);
    println!("余额：{}", account.balance);
    println!("状态：{}", account.is_active);
    
    println!("\n--- 使用方法访问 ---");
    // 使用方法访问
    println!("余额：{}", account.get_balance());
    println!("账户信息：{}", account.get_account_info());
    
    println!("\n--- 直接修改属性 vs 使用方法修改 ---");
    
    // 直接修改属性（不推荐）
    println!("直接修改余额：");
    account.balance = 999.99;
    println!("余额：{}", account.balance);
    
    // 使用方法修改（推荐）
    println!("使用方法存款：");
    account.deposit(100.0);
    println!("余额：{}", account.get_balance());
    
    println!("使用方法取款：");
    account.withdraw(50.0);
    println!("余额：{}", account.get_balance());
}

// 封装性演示
pub fn encapsulation_demo() {
    println!("=== 封装性演示 ===");
    
    let mut account = BankAccount::new(
        String::from("2222222222"),
        String::from("赵六")
    );
    
    // 使用公共方法进行安全操作
    account.deposit(1000.0);
    
    // 尝试取款超过余额
    account.withdraw(1500.0);  // 会被拒绝
    
    // 尝试取款负数
    account.withdraw(-100.0);  // 会被拒绝
    
    // 正常取款
    account.withdraw(500.0);
    
    println!("最终余额：{}", account.get_balance());
}

// 只读属性演示
pub fn read_only_property_demo() {
    println!("=== 只读属性演示 ===");
    
    let account = BankAccount::new(
        String::from("3333333333"),
        String::from("钱七")
    );
    
    // 可以读取属性
    println!("账户号：{}", account.account_number);
    println!("户主：{}", account.owner_name);
    
    // 但无法修改（因为 account 是不可变的）
    // account.balance = 1000.0;  // 编译错误！
    
    // 只能通过方法获取信息
    println!("余额：{}", account.get_balance());
    println!("信息：{}", account.get_account_info());
}

// 主演示函数
pub fn property_vs_method_demo() {
    println!("🏦 结构体属性 vs 方法演示");
    println!("================================");
    
    property_access_demo();
    println!();
    
    method_call_demo();
    println!();
    
    property_vs_method_comparison();
    println!();
    
    encapsulation_demo();
    println!();
    
    read_only_property_demo();
    println!();
    
    println!("💡 总结：");
    println!("  - 属性：存储数据，直接访问");
    println!("  - 方法：提供行为，通过函数调用");
    println!("  - 属性可以读写，方法可以封装逻辑");
    println!("  - 方法提供更好的封装性和安全性");
    println!("  - 属性适合简单数据，方法适合复杂操作");
}
