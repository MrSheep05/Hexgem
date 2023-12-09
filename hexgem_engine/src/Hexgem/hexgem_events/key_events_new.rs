use winit::keyboard::{KeyLocation, PhysicalKey};

use super::event_new::{Event, EventCategory};

pub struct KeyEvent {
    pressed: bool,
    handled: bool,
    pub key: PhysicalKey,
    pub location: KeyLocation,
    pub repeat: bool,
}

impl KeyEvent {
    pub fn create(pressed: bool, key: PhysicalKey, location: KeyLocation, repeat: bool) -> Self {
        Self {
            key,
            repeat,
            pressed,
            location,
            handled: false,
        }
    }

    pub fn handle(&mut self) {
        self.handled = true;
    }
}
impl Event for KeyEvent {
    fn handled(&self) -> bool {
        self.handled
    }

    fn get_event_type(&self) -> super::event_new::EventType {
        if self.pressed {
            super::event_new::EventType::KeyPressed
        } else {
            super::event_new::EventType::KeyReleased
        }
    }

    fn get_category(&self) -> super::event_new::CategoryBitFlag {
        EventCategory::EventCategoryKeyboard | EventCategory::EventCategoryInput
    }
}
