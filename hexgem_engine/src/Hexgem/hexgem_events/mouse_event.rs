use super::event::{Event, EventCategory, EventType};
use crate::{eventImpl, toAnyImpl, Hexgem::core::Position};

pub struct MouseButtonEvent {
    pressed: bool,
    pub repeated: bool,
    handled: bool,
    pub modifiers: glfw::Modifiers,
    pub button: glfw::MouseButton,
}

impl MouseButtonEvent {
    pub fn create(
        pressed: bool,
        repeated: bool,
        button: glfw::MouseButton,
        modifiers: glfw::Modifiers,
    ) -> Self {
        Self {
            button,
            pressed,
            repeated,
            modifiers,
            handled: false,
        }
    }
}

toAnyImpl!(MouseButtonEvent);
impl Event for MouseButtonEvent {
    fn handled(&mut self) -> &mut bool {
        &mut self.handled
    }

    fn get_event_type(&self) -> EventType {
        if self.pressed {
            EventType::MouseButtonPressed
        } else {
            EventType::MouseButtonReleased
        }
    }

    fn get_category(&self) -> super::event::CategoryBitFlag {
        EventCategory::Mouse | EventCategory::MouseButton | EventCategory::Input
    }

    fn is_handled(&self) -> bool {
        self.handled
    }
}

pub struct MouseScrollEvent {
    handled: bool,
    pub dx: f64,
    pub dy: f64,
}

impl MouseScrollEvent {
    pub fn create(dx: f64, dy: f64) -> Self {
        Self {
            dx,
            dy,
            handled: false,
        }
    }
}
eventImpl!(
    MouseScrollEvent,
    MouseScrolled,
    EventCategory::Mouse | EventCategory::Input
);

pub struct MouseMoveEvent {
    handled: bool,
    pub position: Position<f64>,
}

impl MouseMoveEvent {
    pub fn create(position: Position<f64>) -> Self {
        Self {
            position,
            handled: false,
        }
    }
}
eventImpl!(
    MouseMoveEvent,
    MouseMoved,
    EventCategory::Mouse | EventCategory::Input
);
