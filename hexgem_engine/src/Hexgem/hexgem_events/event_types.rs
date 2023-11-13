use super::{application_events::*, input_events::*, keyboard_events::*, mouse_events::*};
use std::{any::Any, fmt::Display};

#[derive(Clone, Debug, Copy)]
pub enum HexgemEvent {
    None,
    WindowClose(WindowCloseEvent),
    WindowResize(WindowResizeEvent),
    WindowFocus(WindowFocusEvent),
    WindowLostFocus(WindowLostFocusEvent),
    WindowMoved(WindowMoveEvent),
    AppTick(AppTickEvent),
    AppUpdate(AppUpdateEvent),
    AppRender(AppRenderEvent),
    KeyPressed(KeyEvent),
    KeyReleased(KeyEvent),
    MouseButtonPressed(MouseButtonEvent),
    MouseButtonReleased(MouseButtonEvent),
    MouseMoved(MouseMovedEvent),
    MouseScrolled(MouseScrollEvent),
}

#[derive(Debug)]

pub enum EventCategory {
    None,
    EventCategoryApplication(EventCategoryApplication),
    EventCategoryInput(EventCategoryInput),
    EventCategoryKeyboard(EventCategoryKeyboard),
    EventCategoryMouse(EventCategoryMouse),
    EventCategoryMouseButton(EventCategoryMouseButton),
}

macro_rules! event {
    ($Variant:ident,$event:ident) => {
        HexgemEvent::$Variant($event)
    };
    ($Variant:ident) => {
        HexgemEvent::$Variant
    };
}
macro_rules! category {
    ($Category: ident,$Variant:ident,$event:ident) => {
        EventCategory::$Category($Category::$Variant($event))
    };
    ($Category: ident,$Variant:ident) => {
        EventCategory::$Category($Category::$Variant)
    };
}

impl Display for EventCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EventCategory: {:?}", &self)
    }
}

impl HexgemEvent {
    pub fn get_event(&self) -> &dyn Any {
        match self {
            HexgemEvent::None => &(),
            HexgemEvent::WindowClose(e) => e,
            HexgemEvent::WindowResize(e) => e,
            HexgemEvent::WindowFocus(e) => e,
            HexgemEvent::WindowLostFocus(e) => e,
            HexgemEvent::WindowMoved(e) => e,
            HexgemEvent::AppTick(e) => e,
            HexgemEvent::AppUpdate(e) => e,
            HexgemEvent::AppRender(e) => e,
            HexgemEvent::KeyPressed(e) => e,
            HexgemEvent::KeyReleased(e) => e,
            HexgemEvent::MouseButtonPressed(e) => e,
            HexgemEvent::MouseButtonReleased(e) => e,
            HexgemEvent::MouseMoved(e) => e,
            HexgemEvent::MouseScrolled(e) => e,
        }
    }
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
            event!(WindowClose, close_event) => {
                category!(EventCategoryInput, WindowClose, close_event)
            }
            event!(WindowResize, resize_event) => {
                category!(EventCategoryInput, WindowResize, resize_event)
            }

            event!(WindowFocus, focus_event) => {
                category!(EventCategoryInput, WindowFocus, focus_event)
            }
            event!(WindowLostFocus, focus_event) => {
                category!(EventCategoryInput, WindowLostFocus, focus_event)
            }
            event!(WindowMoved, move_event) => {
                category!(EventCategoryInput, WindowMoved, move_event)
            }
            event!(AppTick, app_event) => category!(EventCategoryApplication, AppTick, app_event),
            event!(AppUpdate, app_event) => {
                category!(EventCategoryApplication, AppUpdate, app_event)
            }
            event!(AppRender, app_event) => {
                category!(EventCategoryApplication, AppRender, app_event)
            }
        };
    }
}

impl Display for HexgemEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HexgemEvent: {:?}", &self)
    }
}
