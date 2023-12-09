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
    pub const EventCategoryApplication: CategoryBitFlag = BIT(0);
    pub const EventCategoryInput: CategoryBitFlag = BIT(1);
    pub const EventCategoryKeyboard: CategoryBitFlag = BIT(2);
    pub const EventCategoryMouse: CategoryBitFlag = BIT(3);
    pub const EventCategoryMouseButton: CategoryBitFlag = BIT(4);
}

pub trait Event {
    fn handled(&self) -> bool;
    fn get_event_type(&self) -> EventType;
    fn get_category(&self) -> CategoryBitFlag;
    fn is_in_category(&self, category: CategoryBitFlag) -> bool {
        return (self.get_category() & category).0 != 0;
    }
}
