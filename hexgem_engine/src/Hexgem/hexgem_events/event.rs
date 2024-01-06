use std::any::type_name;

use crate::{
    bitOperations,
    Hexgem::core::{bit, ToAny},
};
pub struct CategoryBitFlag(u32);
bitOperations!(CategoryBitFlag);

const fn BIT(i: u8) -> CategoryBitFlag {
    CategoryBitFlag(bit(i))
}
#[derive(PartialEq, Debug)]
pub enum EventType {
    None = 0,
    WindowClose,
    WindowResize,
    WindowFocus,
    WindowLostFocus,
    WindowMoved,
    AppTick,
    AppUpdate,
    AppRender,
    KeyPressed,
    KeyReleased,
    MouseButtonPressed,
    MouseButtonReleased,
    MouseMoved,
    MouseScrolled,
}

pub struct EventCategory {}
impl EventCategory {
    pub const None: CategoryBitFlag = CategoryBitFlag(0);
    pub const Application: CategoryBitFlag = BIT(0);
    pub const Input: CategoryBitFlag = BIT(1);
    pub const Keyboard: CategoryBitFlag = BIT(2);
    pub const Mouse: CategoryBitFlag = BIT(3);
    pub const MouseButton: CategoryBitFlag = BIT(4);
}

pub trait Event: ToAny {
    fn handled(&mut self) -> &mut bool;
    fn get_event_type(&self) -> EventType;
    fn get_category(&self) -> CategoryBitFlag;
    fn is_handled(&self) -> bool;
    fn is_in_category(&self, category: CategoryBitFlag) -> bool {
        return (self.get_category() & category).0 != 0;
    }
}

#[macro_export]
macro_rules! eventImpl {
    ($event:ident,$event_type:ident,$category:expr) => {
        crate::toAnyImpl!($event);
        impl Event for $event {
            fn handled(&mut self) -> &mut bool {
                &mut self.handled
            }

            fn get_event_type(&self) -> EventType {
                EventType::$event_type
            }

            fn get_category(&self) -> super::event::CategoryBitFlag {
                $category
            }

            fn is_handled(&self) -> bool {
                self.handled
            }
        }
    };
}

pub struct NoneEvent {
    handled: bool,
}

impl NoneEvent {
    pub fn create() -> Self {
        Self { handled: false }
    }
}
eventImpl!(NoneEvent, None, EventCategory::None);

pub struct EventDispatcher<'a> {
    event: &'a mut Box<dyn Event>,
}

impl<'a> EventDispatcher<'a> {
    pub fn from(event: &'a mut Box<dyn Event>) -> Self {
        Self { event }
    }

    pub fn dispatch<I: Event + 'static, F>(&self, event_type: EventType, mut callback: F) -> bool
    where
        F: FnMut(&I) -> Option<bool>,
    {
        if &self.event.get_event_type() == &event_type {
            let event_any = &self.event.as_any();
            let event = match event_any.downcast_ref::<I>() {
                Some(e) => e,
                None => panic!(
                    "Cannot downcast {:?} event to desired {} type",
                    event_type,
                    type_name::<I>()
                ),
            };
            let opt = callback(event);
            return opt.map_or(true, |result| result);
        }
        return false;
    }
}
