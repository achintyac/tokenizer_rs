pub const VOCAB_SIZE: u32 = 270;

pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Rectangle {
        Rectangle{width, height}
    }
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}