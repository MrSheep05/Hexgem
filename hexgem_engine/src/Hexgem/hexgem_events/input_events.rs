use winit::dpi::{PhysicalPosition, PhysicalSize};

#[derive(Debug, Clone, Copy)]

pub struct WindowResizeEvent {
    pub size: PhysicalSize<u32>,
}

#[derive(Debug, Clone, Copy)]

pub struct WindowMoveEvent {
    pub position: PhysicalPosition<i32>,
}

#[derive(Debug, Clone, Copy)]

pub struct WindowLostFocusEvent {}

#[derive(Debug, Clone, Copy)]

pub struct WindowFocusEvent {}

#[derive(Debug, Clone, Copy)]

pub struct WindowCloseEvent {}
#[derive(Debug)]

pub enum EventCategoryInput {
    WindowClose(WindowCloseEvent),
    WindowFocus(WindowFocusEvent),
    WindowLostFocus(WindowLostFocusEvent),
    WindowMoved(WindowMoveEvent),
    WindowResize(WindowResizeEvent),
}
