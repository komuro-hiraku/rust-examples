use rand::distributions::Alphanumeric;
use rand::Rng;

fn main() {
    let s: String = rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    println!("generate: {}", s);
}
