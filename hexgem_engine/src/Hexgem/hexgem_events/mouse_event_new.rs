use winit::{
    dpi::PhysicalPosition,
    event::{MouseButton, MouseScrollDelta, TouchPhase},
};

use crate::eventImpl;

use super::event_new::{Event, EventCategory, EventType};

pub struct MouseButtonEvent {
    pressed: bool,
    handled: bool,
    pub button: MouseButton,
}

impl MouseButtonEvent {
    pub fn create(pressed: bool, button: MouseButton) -> Self {
        Self {
            button,
            pressed,
            handled: false,
        }
    }
}

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

    fn get_category(&self) -> super::event_new::CategoryBitFlag {
        EventCategory::Mouse | EventCategory::MouseButton | EventCategory::Input
    }
}

pub struct MouseScrollEvent {
    handled: bool,
    pub scroll_delta: MouseScrollDelta,
    pub phase: TouchPhase,
}

impl MouseScrollEvent {
    pub fn create(scroll_delta: MouseScrollDelta, phase: TouchPhase) -> Self {
        Self {
            phase,
            scroll_delta,
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
    pub position: PhysicalPosition<f64>,
}

impl MouseMoveEvent {
    pub fn create(position: PhysicalPosition<f64>) -> Self {
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
