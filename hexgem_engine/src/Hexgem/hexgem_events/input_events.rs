use winit::dpi::{PhysicalPosition, PhysicalSize};

#[derive(Debug, Clone, Copy)]

pub struct WindowResizeEvent {
    pub size: PhysicalSize<u32>,
}

#[derive(Debug, Clone, Copy)]

pub struct WindowMoveEvent {
    pub position: PhysicalPosition<i32>,
}
#[derive(Debug)]

pub enum EventCategoryInput {
    WindowClose,
    WindowFocus,
    WindowLostFocus,
    WindowMoved { move_event: WindowMoveEvent },
    WindowResize { resize_event: WindowResizeEvent },
}
