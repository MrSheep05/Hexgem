use crate::{
    toAnyImpl,
    HexgemEvent::{Key, Modifiers},
};

use super::event::{Event, EventCategory};

pub struct KeyboardEvent {
    pub pressed: bool,
    handled: bool,
    pub key: Key,
    pub repeat: bool,
    pub modifiers: Modifiers,
}

impl KeyboardEvent {
    pub fn create(pressed: bool, key: Key, repeat: bool, modifiers: Modifiers) -> Self {
        Self {
            key,
            repeat,
            pressed,
            modifiers,
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
