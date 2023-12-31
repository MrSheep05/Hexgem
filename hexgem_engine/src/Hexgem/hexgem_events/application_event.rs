use crate::{
    eventImpl, toAnyImpl,
    Hexgem::core::{Position, Size},
};

use super::event::{Event, EventCategory, EventType};

pub struct WindowFocusEvent {
    is_focused: bool,
    handled: bool,
}

impl WindowFocusEvent {
    pub fn create(is_focused: bool) -> Self {
        Self {
            is_focused,
            handled: false,
        }
    }
}
toAnyImpl!(WindowFocusEvent);
impl Event for WindowFocusEvent {
    fn handled(&mut self) -> &mut bool {
        &mut self.handled
    }

    fn get_event_type(&self) -> EventType {
        if self.is_focused {
            EventType::WindowFocus
        } else {
            EventType::WindowLostFocus
        }
    }

    fn get_category(&self) -> super::event::CategoryBitFlag {
        EventCategory::Application
    }

    fn is_handled(&self) -> bool {
        self.handled
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
    pub size: Size<i32>,
}

impl WindowResizeEvent {
    pub fn create(size: Size<i32>) -> Self {
        Self {
            size,
            handled: false,
        }
    }
}
eventImpl!(WindowResizeEvent, WindowResize, EventCategory::Application);

pub struct WindowMoveEvent {
    handled: bool,
    pub position: Position<i32>,
}

impl WindowMoveEvent {
    pub fn create(position: Position<i32>) -> Self {
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
