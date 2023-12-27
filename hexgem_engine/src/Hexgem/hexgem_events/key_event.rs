use winit::{
    event_loop::EventLoopWindowTarget,
    keyboard::{KeyLocation, PhysicalKey},
};

use crate::toAnyImpl;

use super::event::{Event, EventCategory};

pub struct KeyboardEvent {
    pressed: bool,
    handled: bool,
    pub key: PhysicalKey,
    pub location: KeyLocation,
    pub repeat: bool,
}

impl KeyboardEvent {
    pub fn create(pressed: bool, key: PhysicalKey, location: KeyLocation, repeat: bool) -> Self {
        Self {
            key,
            repeat,
            pressed,
            location,
            handled: false,
        }
    }
}
toAnyImpl!(KeyboardEvent);
impl Event for KeyboardEvent {
    fn handled(&mut self) -> &mut bool {
        &mut self.handled
    }

    fn get_event_type(&self) -> super::event::EventType {
        if self.pressed {
            super::event::EventType::KeyPressed
        } else {
            super::event::EventType::KeyReleased
        }
    }

    fn get_category(&self) -> super::event::CategoryBitFlag {
        EventCategory::Keyboard | EventCategory::Input
    }

    fn is_handled(&self) -> bool {
        self.handled
    }
}
