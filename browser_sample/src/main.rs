use rand::Rng;
use std::io;

fn main() {

    println!("{}: Input Your URL:", random());
    let mut url = String::new();
    io::stdin().read_line(&mut url).expect("Input Open URL");

    let url = url.trim();
    match webbrowser::open(&url) {
        Ok(_) => println!("Success open: {}", &url),
        Err(_) => eprintln!("Failed open: {}", &url),
    }
}

fn random() -> String {
    let mut rng = rand::thread_rng();
    let val: i32 = rng.gen();
    
    base64::encode(val.to_string().as_bytes())
}