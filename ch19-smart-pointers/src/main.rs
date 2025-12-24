use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    println!("=== 智能指针 (Smart Pointers) Demo ===");

    // 1. Box<T>
    let b = Box::new(5);
    println!("b = {}", b);

    // 2. Deref Trait
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // *y 实际上运行的是 *(y.deref())
    println!("MyBox deref works!");

    // 3. Drop Trait
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    // c 和 d 会在 main 结束时自动 drop

    // 4. Rc<T> (Reference Counted)
    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    
    let _b = List::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    
    {
        let _c = List::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // 5. RefCell<T> (Interior Mutability)
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Rc::new(Cons(Rc::new(RefCell::new(3)), Rc::clone(&a)));
    let c = Rc::new(Cons(Rc::new(RefCell::new(4)), Rc::clone(&a)));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

// 自定义 Box
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

// 自定义 Drop
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

// 用于 Rc 的 List
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

// 用于 RefCell 的 List
#[derive(Debug)]
enum ListRefCell {
    Cons(Rc<RefCell<i32>>, Rc<ListRefCell>),
    Nil,
}

use crate::ListRefCell::{Cons, Nil};
