use winit::{
    dpi::PhysicalPosition,
    event::{MouseButton, MouseScrollDelta, TouchPhase},
};

#[derive(Clone, Debug)]

pub struct MouseButtonEvent {
    pub button: MouseButton,
}
#[derive(Clone, Debug)]

pub struct MouseMovedEvent {
    pub position: PhysicalPosition<f64>,
}

#[derive(Clone, Debug)]

pub struct MouseScrollEvent {
    pub scroll_delta: MouseScrollDelta,
    pub phase: TouchPhase,
}
#[derive(Debug)]

pub enum EventCategoryMouse {
    MouseMoved { mouse_event: MouseMovedEvent },
    MouseScrolled { mouse_event: MouseScrollEvent },
}
#[derive(Debug)]

pub enum EventCategoryMouseButton {
    MouseButtonPressed {
        mouse_button_event: MouseButtonEvent,
    },
    MouseButtonReleased {
        mouse_button_event: MouseButtonEvent,
    },
}
