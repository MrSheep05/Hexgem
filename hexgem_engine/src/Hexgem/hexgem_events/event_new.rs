use crate::{bitOperations, Hexgem::core::bit};

pub struct CategoryBitFlag(u32);
bitOperations!(CategoryBitFlag);

const fn BIT(i: u8) -> CategoryBitFlag {
    CategoryBitFlag(bit(i))
}
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

pub trait Event {
    fn handled(&mut self) -> &mut bool;
    fn get_event_type(&self) -> EventType;
    fn get_category(&self) -> CategoryBitFlag;
    fn handle(&mut self) {
        *self.handled() = true;
    }
    fn is_in_category(&self, category: CategoryBitFlag) -> bool {
        return (self.get_category() & category).0 != 0;
    }
}

#[macro_export]
macro_rules! eventImpl {
    ($event:ident,$event_type:ident,$category:expr) => {
        impl Event for $event {
            fn handled(&mut self) -> &mut bool {
                &mut self.handled
            }

            fn get_event_type(&self) -> EventType {
                EventType::$event_type
            }

            fn get_category(&self) -> super::event_new::CategoryBitFlag {
                $category
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
