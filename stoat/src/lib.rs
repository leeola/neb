use action::Action;

pub mod buffer {
    pub struct Buffer;
}
pub mod session {}
pub mod action {
    pub enum Action {}
}
pub mod message {
    pub enum Message {}
}

#[derive(Default)]
pub struct Stoat;
impl Stoat {
    pub fn new() -> Self {
        Self
    }
    pub fn input(&self) {
        todo!()
    }
    pub fn input_echo(&self) -> Action {
        todo!()
    }
}
