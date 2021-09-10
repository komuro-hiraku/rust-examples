pub struct Post {
    state: Option<Box<State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

trait State {
    // Box<Self> で与えられた引数の所有権を奪って新しいのにする
    fn request_review(self: Box<Self>) -> Box<State>;
    fn approve(self: Box<Self>) -> Box<State>;
    fn reject(self: Box<Self>) -> Box<State>;

    // デフォルト実装は空文字で返す
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<State> {
        // Draft ステータスで request_review すると PendingReview ステータスを返す
        Box::new(PendingReview {})
    }

    // Draft で approve 読んでもステータス変更なし
    fn approve(self: Box<Self>) -> Box<State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<State> {
        // PendingReviewステータスで request_review しても自身を返すだけ
        self
    }

    fn approve(self: Box<Self>) -> Box<State> {
        Box::new(FinallyReview {})
    }

    fn reject(self: Box<Self>) -> Box<State> {
        Box::new(Draft {})
    }
}

struct FinallyReview {}

impl State for FinallyReview {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<State> {
        Box::new(Published {})
    }

    fn reject(self: Box<Self>) -> Box<State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<State> {
        self
    }

    // Published ステータスのときのみ Content 文字スライスを返す
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn reject(self: Box<Self>) -> Box<State> {
        self
    }

    // 追加課題
    // 記事の状態をPendingReviewからDraftに戻すrejectメソッドを追加する → Pass
    // 状態がPublishedに変化させられる前にapproveを2回呼び出す必要があるようにする → 
    // 記事がDraft状態の時のみテキスト内容をユーザが追加できるようにする。 ヒント: ステートオブジェクトに内容について変わる可能性のあるものの責任を持たせつつも、 Postを変更することには責任を持たせない。
}
