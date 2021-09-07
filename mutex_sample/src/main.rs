use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 原子的参照カウント
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // counter は不変なのに中身操作できる。RefCell<T> と同じような感じ
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

// fn ng_loop_mutex() {
//     let counter = Rc::new(Mutex::new(0));
//     let mut handles = vec![];
//
//     for _ in 0..10 {
//         // for loop にすると一回目の loop で counter が move されるので
//         // 以降使えないので Rc::clone で回避しようとするがNG
//         // "残念ながら、Rc<T>はスレッド間で共有するには安全ではないのです。" とのこと
//         let counter = Rc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();
//             // lock が取得できたらカウントアップ
//             *num += 1;
//         });
//
//         // thread 作ったら handle を Vector で管理
//         handles.push(handle);
//     }
//
//     // 全 Thread を待つ
//     for handle in handles {
//         handle.join().unwrap();
//     }
//
//     // 初期値 0 なので 10 が最終結果?
//     println!("Result: {}", *counter.lock().unwrap());
// }
