use std::any::Any;

use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::eventImpl;

use super::event_new::{Event, EventCategory, EventType};

pub struct WindowFocusEvent {
    lost_focus: bool,
    handled: bool,
}

impl WindowFocusEvent {
    pub fn create(lost_focus: bool) -> Self {
        Self {
            lost_focus,
            handled: false,
        }
    }
}

impl Event for WindowFocusEvent {
    fn handled(&mut self) -> &mut bool {
        &mut self.handled
    }

    fn get_event_type(&self) -> EventType {
        if (self.lost_focus) {
            EventType::WindowLostFocus
        } else {
            EventType::WindowFocus
        }
    }

    fn get_category(&self) -> super::event_new::CategoryBitFlag {
        EventCategory::Application
    }
}

pub struct WindowCloseEvent {
    handled: bool,
}

impl WindowCloseEvent {
    pub fn create() -> Self {
        Self { handled: false }
    }
}
eventImpl!(WindowCloseEvent, WindowClose, EventCategory::Application);

pub struct WindowResizeEvent {
    handled: bool,
    pub size: PhysicalSize<u32>,
}

impl WindowResizeEvent {
    pub fn create(size: PhysicalSize<u32>) -> Self {
        Self {
            size,
            handled: false,
        }
    }
}
eventImpl!(WindowResizeEvent, WindowResize, EventCategory::Application);

pub struct WindowMoveEvent {
    handled: bool,
    pub position: PhysicalPosition<i32>,
}

impl WindowMoveEvent {
    pub fn create(position: PhysicalPosition<i32>) -> Self {
        Self {
            position,
            handled: false,
        }
    }
}
eventImpl!(WindowMoveEvent, WindowMoved, EventCategory::Application);

pub struct AppTickEvent {
    handled: bool,
}

impl AppTickEvent {
    pub fn create() -> Self {
        Self { handled: false }
    }
}
eventImpl!(AppTickEvent, AppTick, EventCategory::Application);

pub struct AppUpdateEvent {
    handled: bool,
}

impl AppUpdateEvent {
    pub fn create() -> Self {
        Self { handled: false }
    }
}
eventImpl!(AppUpdateEvent, AppUpdate, EventCategory::Application);

pub struct AppRenderEvent {
    handled: bool,
}

impl AppRenderEvent {
    pub fn create() -> Self {
        Self { handled: false }
    }
}
eventImpl!(AppRenderEvent, AppRender, EventCategory::Application);
