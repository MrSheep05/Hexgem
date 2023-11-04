use winit::keyboard::{KeyLocation, PhysicalKey};

#[derive(Clone, Debug, Copy)]
pub struct KeyEvent {
    pub key: PhysicalKey,
    pub location: KeyLocation,
    pub repeat: bool,
}
#[derive(Debug)]

pub enum EventCategoryKeyboard {
    KeyPressed { key_event: KeyEvent },
    KeyReleased { key_event: KeyEvent },
}
