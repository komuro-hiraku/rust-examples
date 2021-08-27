enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5,
        Rc::new(Cons(10,
            Rc::new(Nil)))));
    
    // a 生成後のカウント
    println!("count after creating a = {}", Rc::strong_count(&a));  // 参照カウント

    let b = Cons(3, Rc::clone(&a));
    // b 生成後のカウント
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        // c 生成後のカウント
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    // let c = Cons(4, Box::new(a));

    // c がスコープ抜けたあとのカウント
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
