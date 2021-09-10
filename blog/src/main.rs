extern crate blog;

use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    // Draft へ戻す
    post.reject();
    post.approve();
    assert_eq!("", post.content()); // Approve しても変わらない

    post.request_review();
    assert_eq!("", post.content());

    // 1 回目
    post.approve();
    assert_eq!("", post.content());

    // 2 回目
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
