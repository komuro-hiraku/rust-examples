pub trait Draw {
    fn draw(&self);
}

pub struct Screen<T: Draw> {
    pub components: Vec<Box<Draw>>,
}

impl<T> Screen<T> {
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
        todo!()
    }
}
