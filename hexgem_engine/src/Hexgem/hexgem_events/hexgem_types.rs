use crate::Hexgem::core;

pub enum MouseButton {
    LeftButton,
    RightButton,
    MiddleButton,
    Button4,
    Button5,
    Button6,
    Button7,
    Button8,
    Unknown,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Key {
    Space,
    Apostrophe,
    Comma,
    Minus,
    Period,
    Slash,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Semicolon,
    Equal,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    LeftBracket,
    Backslash,
    RightBracket,
    GraveAccent,
    World1,
    World2,

    Escape,
    Enter,
    Tab,
    Backspace,
    Insert,
    Delete,
    Right,
    Left,
    Down,
    Up,
    PageUp,
    PageDown,
    Home,
    End,
    CapsLock,
    ScrollLock,
    NumLock,
    PrintScreen,
    Pause,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    Kp0,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    KpDecimal,
    KpDivide,
    KpMultiply,
    KpSubtract,
    KpAdd,
    KpEnter,
    KpEqual,
    LeftShift,
    LeftControl,
    LeftAlt,
    LeftSuper,
    RightShift,
    RightControl,
    RightAlt,
    RightSuper,
    Menu,
    Unknown,
}
#[derive(Clone)]
pub enum Mod {
    None = bit(0),
    LeftShift = bit(1),
    RightShift = bit(2),
    Shift = bit(1) | bit(2),
    LeftControl = bit(3),
    RightControl = bit(4),
    Control = bit(3) | bit(4),
    LeftAlt = bit(5),
    RightAlt = bit(6),
    Alt = bit(5) | bit(6),
    LeftGui = bit(7),
    RightGui = bit(8),
    Gui = bit(7) | bit(8),
    NumLock = bit(9),
    CapsLock = bit(10),
    ModeLock = bit(11),
    ReserveLock = bit(12),
}

impl Mod {
    fn get(self) -> isize {
        self as isize
    }
}
#[derive(Debug)]
pub struct Modifiers(isize);

impl Modifiers {
    pub fn includes(&self, m: Mod) -> bool {
        let size = m.get();
        self.0 & size == size
    }

    pub fn has(&self, m: Mod) -> bool {
        self.0 & m.get() > 0
    }

    pub fn create(modifiers: Vec<Mod>) -> Self {
        if modifiers.is_empty() {
            return Self(1);
        }
        let size = Self::append_mod(modifiers, 0);
        Self(size)
    }

    fn append_mod(mut modifiers: Vec<Mod>, current_size: isize) -> isize {
        if let Some(curr_mod) = modifiers.pop() {
            return Self::append_mod(modifiers, current_size | curr_mod.get());
        } else {
            return current_size;
        }
    }
}

const fn bit(i: u8) -> isize {
    core::bit(i) as isize
}
