use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize)]
pub struct PostMessage<'a> {
    r#type: &'a str,
    title: &'a str,
    message: &'a str,
}

#[derive(Debug, Deserialize)]
struct ExampleResponse {
    success: String
}

fn post_example(message: &str) {
 
    let body = PostMessage {
        r#type: "Message",
        title: "Sample",
        message: message
    };

    let client = reqwest::blocking::Client::new();
    let resp = client.post("https://reqbin.com/sample/post/json")
        .json(&body)
        .send()
        .unwrap();

    let js = resp.json::<HashMap<String, String>>();
    println!("{:?}", js);
}

fn main() {
    post_example("Hello, reqwest world!");
}
