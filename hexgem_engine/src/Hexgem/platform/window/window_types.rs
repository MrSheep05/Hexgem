use std::collections::HashMap;

use crate::HexgemEvent::{Key, Mod, Modifiers, MouseButton};

impl From<glfw::MouseButton> for MouseButton {
    fn from(value: glfw::MouseButton) -> Self {
        match value {
            glfw::MouseButton::Button1 => Self::LeftButton,
            glfw::MouseButton::Button2 => Self::RightButton,
            glfw::MouseButton::Button3 => Self::MiddleButton,
            glfw::MouseButton::Button4 => Self::Button4,
            glfw::MouseButton::Button5 => Self::Button5,
            glfw::MouseButton::Button6 => Self::Button6,
            glfw::MouseButton::Button7 => Self::Button7,
            glfw::MouseButton::Button8 => Self::Button8,
        }
    }
}

impl From<sdl2::mouse::MouseButton> for MouseButton {
    fn from(value: sdl2::mouse::MouseButton) -> Self {
        match value {
            sdl2::mouse::MouseButton::Unknown => Self::Unknown,
            sdl2::mouse::MouseButton::Left => Self::LeftButton,
            sdl2::mouse::MouseButton::Middle => Self::MiddleButton,
            sdl2::mouse::MouseButton::Right => Self::RightButton,
            sdl2::mouse::MouseButton::X1 => Self::Button4,
            sdl2::mouse::MouseButton::X2 => Self::Button5,
        }
    }
}

impl From<sdl2::keyboard::Keycode> for Key {
    fn from(value: sdl2::keyboard::Keycode) -> Self {
        match value {
            sdl2::keyboard::Keycode::Space => Key::Space,
            sdl2::keyboard::Keycode::Quote => Key::Apostrophe,
            sdl2::keyboard::Keycode::Comma => Key::Comma,
            sdl2::keyboard::Keycode::Minus => Key::Minus,
            sdl2::keyboard::Keycode::Period => Key::Period,
            sdl2::keyboard::Keycode::Slash => Key::Slash,
            sdl2::keyboard::Keycode::Num0 => Key::Num0,
            sdl2::keyboard::Keycode::Num1 => Key::Num1,
            sdl2::keyboard::Keycode::Num2 => Key::Num2,
            sdl2::keyboard::Keycode::Num3 => Key::Num3,
            sdl2::keyboard::Keycode::Num4 => Key::Num4,
            sdl2::keyboard::Keycode::Num5 => Key::Num5,
            sdl2::keyboard::Keycode::Num6 => Key::Num6,
            sdl2::keyboard::Keycode::Num7 => Key::Num7,
            sdl2::keyboard::Keycode::Num8 => Key::Num8,
            sdl2::keyboard::Keycode::Num9 => Key::Num9,
            sdl2::keyboard::Keycode::Semicolon => Key::Semicolon,
            sdl2::keyboard::Keycode::Equals => Key::Equal,
            sdl2::keyboard::Keycode::A => Key::A,
            sdl2::keyboard::Keycode::B => Key::B,
            sdl2::keyboard::Keycode::C => Key::C,
            sdl2::keyboard::Keycode::D => Key::D,
            sdl2::keyboard::Keycode::E => Key::E,
            sdl2::keyboard::Keycode::F => Key::F,
            sdl2::keyboard::Keycode::G => Key::G,
            sdl2::keyboard::Keycode::H => Key::H,
            sdl2::keyboard::Keycode::I => Key::I,
            sdl2::keyboard::Keycode::J => Key::J,
            sdl2::keyboard::Keycode::K => Key::K,
            sdl2::keyboard::Keycode::L => Key::L,
            sdl2::keyboard::Keycode::M => Key::M,
            sdl2::keyboard::Keycode::N => Key::N,
            sdl2::keyboard::Keycode::O => Key::O,
            sdl2::keyboard::Keycode::P => Key::P,
            sdl2::keyboard::Keycode::Q => Key::Q,
            sdl2::keyboard::Keycode::R => Key::R,
            sdl2::keyboard::Keycode::S => Key::S,
            sdl2::keyboard::Keycode::T => Key::T,
            sdl2::keyboard::Keycode::U => Key::U,
            sdl2::keyboard::Keycode::V => Key::V,
            sdl2::keyboard::Keycode::W => Key::W,
            sdl2::keyboard::Keycode::X => Key::X,
            sdl2::keyboard::Keycode::Y => Key::Y,
            sdl2::keyboard::Keycode::Z => Key::Z,
            sdl2::keyboard::Keycode::LeftBracket => Key::LeftBracket,
            sdl2::keyboard::Keycode::Backslash => Key::Backslash,
            sdl2::keyboard::Keycode::RightBracket => Key::RightBracket,
            sdl2::keyboard::Keycode::Backquote => Key::GraveAccent,
            sdl2::keyboard::Keycode::Escape => Key::Escape,
            sdl2::keyboard::Keycode::Return => Key::Enter,
            sdl2::keyboard::Keycode::Tab => Key::Tab,
            sdl2::keyboard::Keycode::Backspace => Key::Backspace,
            sdl2::keyboard::Keycode::Insert => Key::Insert,
            sdl2::keyboard::Keycode::Delete => Key::Delete,
            sdl2::keyboard::Keycode::Right => Key::Right,
            sdl2::keyboard::Keycode::Left => Key::Left,
            sdl2::keyboard::Keycode::Down => Key::Down,
            sdl2::keyboard::Keycode::Up => Key::Up,
            sdl2::keyboard::Keycode::PageUp => Key::PageUp,
            sdl2::keyboard::Keycode::PageDown => Key::PageDown,
            sdl2::keyboard::Keycode::Home => Key::Home,
            sdl2::keyboard::Keycode::End => Key::End,
            sdl2::keyboard::Keycode::CapsLock => Key::CapsLock,
            sdl2::keyboard::Keycode::ScrollLock => Key::ScrollLock,
            sdl2::keyboard::Keycode::PrintScreen => Key::PrintScreen,
            sdl2::keyboard::Keycode::Pause => Key::Pause,
            sdl2::keyboard::Keycode::F1 => Key::F1,
            sdl2::keyboard::Keycode::F2 => Key::F2,
            sdl2::keyboard::Keycode::F3 => Key::F3,
            sdl2::keyboard::Keycode::F4 => Key::F4,
            sdl2::keyboard::Keycode::F5 => Key::F5,
            sdl2::keyboard::Keycode::F6 => Key::F6,
            sdl2::keyboard::Keycode::F7 => Key::F7,
            sdl2::keyboard::Keycode::F8 => Key::F8,
            sdl2::keyboard::Keycode::F9 => Key::F9,
            sdl2::keyboard::Keycode::F10 => Key::F10,
            sdl2::keyboard::Keycode::F11 => Key::F11,
            sdl2::keyboard::Keycode::F12 => Key::F12,
            _ => Key::Unknown,
        }
    }
}

impl From<glfw::Key> for Key {
    fn from(value: glfw::Key) -> Self {
        match value {
            glfw::Key::Space => Key::Space,
            glfw::Key::Apostrophe => Key::Apostrophe,
            glfw::Key::Comma => Key::Comma,
            glfw::Key::Minus => Key::Minus,
            glfw::Key::Period => Key::Period,
            glfw::Key::Slash => Key::Slash,
            glfw::Key::Num0 => Key::Num0,
            glfw::Key::Num1 => Key::Num1,
            glfw::Key::Num2 => Key::Num2,
            glfw::Key::Num3 => Key::Num3,
            glfw::Key::Num4 => Key::Num4,
            glfw::Key::Num5 => Key::Num5,
            glfw::Key::Num6 => Key::Num6,
            glfw::Key::Num7 => Key::Num7,
            glfw::Key::Num8 => Key::Num8,
            glfw::Key::Num9 => Key::Num9,
            glfw::Key::Semicolon => Key::Semicolon,
            glfw::Key::Equal => Key::Equal,
            glfw::Key::A => Key::A,
            glfw::Key::B => Key::B,
            glfw::Key::C => Key::C,
            glfw::Key::D => Key::D,
            glfw::Key::E => Key::E,
            glfw::Key::F => Key::F,
            glfw::Key::G => Key::G,
            glfw::Key::H => Key::H,
            glfw::Key::I => Key::I,
            glfw::Key::J => Key::J,
            glfw::Key::K => Key::K,
            glfw::Key::L => Key::L,
            glfw::Key::M => Key::M,
            glfw::Key::N => Key::N,
            glfw::Key::O => Key::O,
            glfw::Key::P => Key::P,
            glfw::Key::Q => Key::Q,
            glfw::Key::R => Key::R,
            glfw::Key::S => Key::S,
            glfw::Key::T => Key::T,
            glfw::Key::U => Key::U,
            glfw::Key::V => Key::V,
            glfw::Key::W => Key::W,
            glfw::Key::X => Key::X,
            glfw::Key::Y => Key::Y,
            glfw::Key::Z => Key::Z,
            glfw::Key::LeftBracket => Key::LeftBracket,
            glfw::Key::Backslash => Key::Backslash,
            glfw::Key::RightBracket => Key::RightBracket,
            glfw::Key::GraveAccent => Key::GraveAccent,
            glfw::Key::World1 => Key::World1,
            glfw::Key::World2 => Key::World2,
            glfw::Key::Escape => Key::Escape,
            glfw::Key::Enter => Key::Enter,
            glfw::Key::Tab => Key::Tab,
            glfw::Key::Backspace => Key::Backspace,
            glfw::Key::Insert => Key::Insert,
            glfw::Key::Delete => Key::Delete,
            glfw::Key::Right => Key::Right,
            glfw::Key::Left => Key::Left,
            glfw::Key::Down => Key::Down,
            glfw::Key::Up => Key::Up,
            glfw::Key::PageUp => Key::PageUp,
            glfw::Key::PageDown => Key::PageDown,
            glfw::Key::Home => Key::Home,
            glfw::Key::End => Key::End,
            glfw::Key::CapsLock => Key::CapsLock,
            glfw::Key::ScrollLock => Key::ScrollLock,
            glfw::Key::NumLock => Key::NumLock,
            glfw::Key::PrintScreen => Key::PrintScreen,
            glfw::Key::Pause => Key::Pause,
            glfw::Key::F1 => Key::F1,
            glfw::Key::F2 => Key::F2,
            glfw::Key::F3 => Key::F3,
            glfw::Key::F4 => Key::F4,
            glfw::Key::F5 => Key::F5,
            glfw::Key::F6 => Key::F6,
            glfw::Key::F7 => Key::F7,
            glfw::Key::F8 => Key::F8,
            glfw::Key::F9 => Key::F9,
            glfw::Key::F10 => Key::F10,
            glfw::Key::F11 => Key::F11,
            glfw::Key::F12 => Key::F12,
            glfw::Key::F13 => Key::F13,
            glfw::Key::F14 => Key::F14,
            glfw::Key::F15 => Key::F15,
            glfw::Key::F16 => Key::F16,
            glfw::Key::F17 => Key::F17,
            glfw::Key::F18 => Key::F18,
            glfw::Key::F19 => Key::F19,
            glfw::Key::F20 => Key::F20,
            glfw::Key::F21 => Key::F21,
            glfw::Key::F22 => Key::F22,
            glfw::Key::F23 => Key::F23,
            glfw::Key::F24 => Key::F24,
            glfw::Key::F25 => Key::F25,
            glfw::Key::Kp0 => Key::Kp0,
            glfw::Key::Kp1 => Key::Kp1,
            glfw::Key::Kp2 => Key::Kp2,
            glfw::Key::Kp3 => Key::Kp3,
            glfw::Key::Kp4 => Key::Kp4,
            glfw::Key::Kp5 => Key::Kp5,
            glfw::Key::Kp6 => Key::Kp6,
            glfw::Key::Kp7 => Key::Kp7,
            glfw::Key::Kp8 => Key::Kp8,
            glfw::Key::Kp9 => Key::Kp9,
            glfw::Key::KpDecimal => Key::KpDecimal,
            glfw::Key::KpDivide => Key::KpDivide,
            glfw::Key::KpMultiply => Key::KpMultiply,
            glfw::Key::KpSubtract => Key::KpSubtract,
            glfw::Key::KpAdd => Key::KpAdd,
            glfw::Key::KpEnter => Key::KpEnter,
            glfw::Key::KpEqual => Key::KpEqual,
            glfw::Key::LeftShift => Key::LeftShift,
            glfw::Key::LeftControl => Key::LeftControl,
            glfw::Key::LeftAlt => Key::LeftAlt,
            glfw::Key::LeftSuper => Key::LeftSuper,
            glfw::Key::RightShift => Key::RightShift,
            glfw::Key::RightControl => Key::RightControl,
            glfw::Key::RightAlt => Key::RightAlt,
            glfw::Key::RightSuper => Key::RightSuper,
            glfw::Key::Menu => Key::Menu,
            glfw::Key::Unknown => Key::Unknown,
        }
    }
}

const GLFW_MODIFIERS: &[(glfw::Modifiers, Mod)] = &[
    (glfw::Modifiers::Alt, Mod::Alt),
    (glfw::Modifiers::CapsLock, Mod::CapsLock),
    (glfw::Modifiers::Control, Mod::Control),
    (glfw::Modifiers::NumLock, Mod::NumLock),
    (glfw::Modifiers::Shift, Mod::Shift),
    (glfw::Modifiers::Super, Mod::ModeLock),
];
impl From<glfw::Modifiers> for Modifiers {
    fn from(value: glfw::Modifiers) -> Self {
        let map: Vec<Mod> =
            GLFW_MODIFIERS
                .iter()
                .fold(vec![] as Vec<Mod>, |mut prev, (key, id)| {
                    if value.contains(key.clone()) {
                        prev.push(id.clone());
                    }
                    return prev;
                });
        Self::create(map)
    }
}

const SDL2_MODIFIERS: &[(sdl2::keyboard::Mod, Mod)] = &[
    (sdl2::keyboard::Mod::CAPSMOD, Mod::CapsLock),
    (sdl2::keyboard::Mod::LALTMOD, Mod::LeftAlt),
    (sdl2::keyboard::Mod::LCTRLMOD, Mod::LeftControl),
    (sdl2::keyboard::Mod::LGUIMOD, Mod::LeftGui),
    (sdl2::keyboard::Mod::LSHIFTMOD, Mod::LeftShift),
    (sdl2::keyboard::Mod::MODEMOD, Mod::ModeLock),
    (sdl2::keyboard::Mod::RALTMOD, Mod::RightAlt),
    (sdl2::keyboard::Mod::RCTRLMOD, Mod::RightControl),
    (sdl2::keyboard::Mod::RESERVEDMOD, Mod::ReserveLock),
    (sdl2::keyboard::Mod::RGUIMOD, Mod::RightGui),
    (sdl2::keyboard::Mod::RSHIFTMOD, Mod::RightShift),
];
impl From<sdl2::keyboard::Mod> for Modifiers {
    fn from(value: sdl2::keyboard::Mod) -> Self {
        let map: Vec<Mod> =
            SDL2_MODIFIERS
                .iter()
                .fold(vec![] as Vec<Mod>, |mut prev, (key, id)| {
                    if value.contains(key.clone()) {
                        prev.push(id.clone());
                    }
                    return prev;
                });
        Self::create(map)
    }
}
