// 内部可変性パターン RefCell<T>

// Rc<T> : 同じデータに複数の所有者を持つ。 Box<T>m RefCell<T> は単独所有者
// Box<T> : 不変借用、可変借用もコンパイル時に精査できる。 
//          Rc<T> では不変借用のみがコンパイル時に精査できる
//          RefCell<T> では不変借用も可変借用も実行時に精査できる
// RefCell<T> : 実行時に精査される可変借用を許可するので、RefCell<T> が不変でも、RefCell<T> 内の値を可変化できる

// 不変な値の中の値を可変化することは、内部可変性パターン

#[derive(Debug)]
enum List {

    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // let x = 5;
    // let y = &mut x; // 不変なローカル変数 `x` を可変で借用することはできない

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
