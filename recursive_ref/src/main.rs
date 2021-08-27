use std::rc::Rc;
use std::cell::RefCell;
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    // a の最初の参照カウント
    println!("a initial rc count = {}", Rc::strong_count(&a));
    // a の次の要素
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    // b 作成後の a の参照カウント
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    // b の次の要素
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    // a 変更後の b の参照カウント
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    // a 変更後の a 参照カウント
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // スタックオーバーフローする
    // https://doc.rust-jp.rs/book-ja/img/trpl15-04.svg
    // println!("a next item = {:?}", a.tail());


    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![])
    });

    let branch = Rc::new(Node {
        
    })
}
