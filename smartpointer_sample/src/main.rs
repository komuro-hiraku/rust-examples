#[derive(Debug)]
enum List { // 再帰的な構造のためコンパイルできない
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};
use std::ops::Deref;

fn main() {
    let b = Box::new(5);    // わざとヒープに置く。あんまり意味はない
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, 
        Box::new(Cons(3, 
            Box::new(Nil))))));

    println!("list: {:?}", list);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // *(y.deref()) と同等
    assert_eq!(5, *(y.deref()));

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&(*m)[..]); // 参照外し型強制がなかったらこうなる
    // (*m) で MyBox<String> を String に参照外し
    // &, [..] で String を &str （文字列スライス）に変換


    // Drop trait
    let _c = CustomSmartPointer{ data: String::from("my stuff")};
    let _d = CustomSmartPointer{ data: String::from("other stuff")};
    // _c.drop(); // デストラクタを明示的に呼び出せない。Compile Error
    
    println!("CustomSmartPointers created.");
    drop(_c);
    drop(_d);
    println!("CustomSmartPointers dropped before the end of main.");
}

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

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

// Drop trait
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: {} !", self.data);
    }
}