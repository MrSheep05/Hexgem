use std::fmt::Display;

use super::{
    application_events::EventCategoryApplication, input_events::*, keyboard_events::*,
    mouse_events::*,
};

#[derive(Clone, Debug)]
pub enum HexgemEvent {
    None,
    WindowClose,
    WindowResize {
        resize_event: WindowResizeEvent,
    },
    WindowFocus,
    WindowLostFocus,
    WindowMoved {
        move_event: WindowMoveEvent,
    },
    AppTick,
    AppUpdate,
    AppRender,
    KeyPressed {
        key_event: KeyEvent,
    },
    KeyReleased {
        key_event: KeyEvent,
    },
    MouseButtonPressed {
        mouse_button_event: MouseButtonEvent,
    },
    MouseButtonReleased {
        mouse_button_event: MouseButtonEvent,
    },
    MouseMoved {
        mouse_event: MouseMovedEvent,
    },
    MouseScrolled {
        mouse_event: MouseScrollEvent,
    },
}
#[derive(Debug)]

pub enum EventCategory {
    None,
    EventCategoryApplication { event: EventCategoryApplication },
    EventCategoryInput { event: EventCategoryInput },
    EventCategoryKeyboard { event: EventCategoryKeyboard },
    EventCategoryMouse { event: EventCategoryMouse },
    EventCategoryMouseButton { event: EventCategoryMouseButton },
}

macro_rules! event {
    ($Variant:ident,$($event:ident)*) => {
        HexgemEvent::$Variant { $($event)* }
    };
    ($Variant:ident) => {
        HexgemEvent::$Variant
    };
}
macro_rules! category {
    ($Category: ident,$Variant:ident,$($event:ident)*) => {
        EventCategory::$Category {
            event: $Category::$Variant { $($event)* },
        }
    };
    ($Category: ident,$Variant:ident) => {
        EventCategory::$Category {
            event: $Category::$Variant,
        }
    };
}

impl Display for EventCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EventCategory: {:?}", &self)
    }
}

impl HexgemEvent {
    pub fn get_category(self) -> EventCategory {
        return match self {
            HexgemEvent::None => EventCategory::None,
            event!(MouseButtonReleased, mouse_button_event) => category!(
                EventCategoryMouseButton,
                MouseButtonReleased,
                mouse_button_event
            ),
            event!(MouseButtonPressed, mouse_button_event) => category!(
                EventCategoryMouseButton,
                MouseButtonPressed,
                mouse_button_event
            ),
            event!(MouseMoved, mouse_event) => {
                category!(EventCategoryMouse, MouseMoved, mouse_event)
            }

            event!(MouseScrolled, mouse_event) => {
                category!(EventCategoryMouse, MouseScrolled, mouse_event)
            }
            event!(KeyPressed, key_event) => {
                category!(EventCategoryKeyboard, KeyPressed, key_event)
            }
            event!(KeyReleased, key_event) => {
                category!(EventCategoryKeyboard, KeyReleased, key_event)
            }
            event!(WindowClose) => {
                category!(EventCategoryInput, WindowClose)
            }
            event!(WindowResize, resize_event) => {
                category!(EventCategoryInput, WindowResize, resize_event)
            }

            event!(WindowFocus) => category!(EventCategoryInput, WindowFocus),
            event!(WindowLostFocus) => category!(EventCategoryInput, WindowLostFocus),
            event!(WindowMoved, move_event) => {
                category!(EventCategoryInput, WindowMoved, move_event)
            }
            event!(AppTick) => category!(EventCategoryApplication, AppTick),
            event!(AppUpdate) => category!(EventCategoryApplication, AppUpdate),
            event!(AppRender) => category!(EventCategoryApplication, AppRender),
        };
    }
}

impl Display for HexgemEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HexgemEvent: {:?}", &self)
    }
}
