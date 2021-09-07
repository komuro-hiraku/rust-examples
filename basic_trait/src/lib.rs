// https://raw.githubusercontent.com/rust-lang/book/main/listings/ch17-oop/listing-17-07/src/lib.rs
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("width: {}, height: {}, label: {}", self.width, self.height, self.label);
    }
}
