use winit::{
    dpi::PhysicalPosition,
    event::{MouseButton, MouseScrollDelta, TouchPhase},
};

#[derive(Clone, Debug, Copy)]

pub struct MouseButtonEvent {
    pub button: MouseButton,
}
#[derive(Clone, Debug, Copy)]

pub struct MouseMovedEvent {
    pub position: PhysicalPosition<f64>,
}

#[derive(Clone, Debug, Copy)]

pub struct MouseScrollEvent {
    pub scroll_delta: MouseScrollDelta,
    pub phase: TouchPhase,
}
#[derive(Debug)]

pub enum EventCategoryMouse {
    MouseMoved(MouseMovedEvent),
    MouseScrolled(MouseScrollEvent),
}
#[derive(Debug)]

pub enum EventCategoryMouseButton {
    MouseButtonPressed(MouseButtonEvent),
    MouseButtonReleased(MouseButtonEvent),
}
