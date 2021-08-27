#[derive(Debug)]
enum List { // 再帰的な構造のためコンパイルできない
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let b = Box::new(5);    // わざとヒープに置く。あんまり意味はない
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, 
        Box::new(Cons(3, 
            Box::new(Nil))))));

    println!("list: {:?}", list);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

