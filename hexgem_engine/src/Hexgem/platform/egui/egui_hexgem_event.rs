use crate::HexgemEvent::{
    Event, KeyboardEvent, Mod, Modifiers as Modifier, MouseButtonEvent, MouseMoveEvent,
    MouseScrollEvent,
};
use egui::{pos2, vec2, Key};

use super::{egui_clipboard::ClipboardProvider, egui_state::EguiStateInput};

pub fn translate_virtual_key_code(key: &crate::HexgemEvent::Key) -> egui::Key {
    match key {
        crate::HexgemEvent::Key::Space => Key::Space,
        crate::HexgemEvent::Key::Apostrophe => Key::Backtick,
        crate::HexgemEvent::Key::Comma => Key::Comma,
        crate::HexgemEvent::Key::Minus => Key::Minus,
        crate::HexgemEvent::Key::Period => Key::Period,
        crate::HexgemEvent::Key::Slash => Key::Slash,
        crate::HexgemEvent::Key::Num0 => Key::Num0,
        crate::HexgemEvent::Key::Num1 => Key::Num1,
        crate::HexgemEvent::Key::Num2 => Key::Num2,
        crate::HexgemEvent::Key::Num3 => Key::Num3,
        crate::HexgemEvent::Key::Num4 => Key::Num4,
        crate::HexgemEvent::Key::Num5 => Key::Num5,
        crate::HexgemEvent::Key::Num6 => Key::Num6,
        crate::HexgemEvent::Key::Num7 => Key::Num7,
        crate::HexgemEvent::Key::Num8 => Key::Num8,
        crate::HexgemEvent::Key::Num9 => Key::Num9,
        crate::HexgemEvent::Key::Semicolon => Key::Semicolon,
        crate::HexgemEvent::Key::Equal => Key::Equals,
        crate::HexgemEvent::Key::A => Key::A,
        crate::HexgemEvent::Key::B => Key::B,
        crate::HexgemEvent::Key::C => Key::C,
        crate::HexgemEvent::Key::D => Key::D,
        crate::HexgemEvent::Key::E => Key::E,
        crate::HexgemEvent::Key::F => Key::F,
        crate::HexgemEvent::Key::G => Key::G,
        crate::HexgemEvent::Key::H => Key::H,
        crate::HexgemEvent::Key::I => Key::I,
        crate::HexgemEvent::Key::J => Key::J,
        crate::HexgemEvent::Key::K => Key::K,
        crate::HexgemEvent::Key::L => Key::L,
        crate::HexgemEvent::Key::M => Key::M,
        crate::HexgemEvent::Key::N => Key::N,
        crate::HexgemEvent::Key::O => Key::O,
        crate::HexgemEvent::Key::P => Key::P,
        crate::HexgemEvent::Key::Q => Key::Q,
        crate::HexgemEvent::Key::R => Key::R,
        crate::HexgemEvent::Key::S => Key::S,
        crate::HexgemEvent::Key::T => Key::T,
        crate::HexgemEvent::Key::U => Key::U,
        crate::HexgemEvent::Key::V => Key::V,
        crate::HexgemEvent::Key::W => Key::W,
        crate::HexgemEvent::Key::X => Key::X,
        crate::HexgemEvent::Key::Y => Key::Y,
        crate::HexgemEvent::Key::Z => Key::Z,
        crate::HexgemEvent::Key::LeftBracket => Key::OpenBracket,
        crate::HexgemEvent::Key::Backslash => Key::Backslash,
        crate::HexgemEvent::Key::RightBracket => Key::CloseBracket,
        crate::HexgemEvent::Key::GraveAccent => Key::Backtick,
        crate::HexgemEvent::Key::Escape => Key::Escape,
        crate::HexgemEvent::Key::Enter => Key::Enter,
        crate::HexgemEvent::Key::Tab => Key::Tab,
        crate::HexgemEvent::Key::Backspace => Key::Backspace,
        crate::HexgemEvent::Key::Insert => Key::Insert,
        crate::HexgemEvent::Key::Delete => Key::Delete,
        crate::HexgemEvent::Key::Right => Key::ArrowRight,
        crate::HexgemEvent::Key::Left => Key::ArrowLeft,
        crate::HexgemEvent::Key::Down => Key::ArrowDown,
        crate::HexgemEvent::Key::Up => Key::ArrowUp,
        crate::HexgemEvent::Key::PageUp => Key::PageUp,
        crate::HexgemEvent::Key::PageDown => Key::PageDown,
        crate::HexgemEvent::Key::Home => Key::Home,
        crate::HexgemEvent::Key::End => Key::End,
        crate::HexgemEvent::Key::F1 => Key::F1,
        crate::HexgemEvent::Key::F2 => Key::F2,
        crate::HexgemEvent::Key::F3 => Key::F3,
        crate::HexgemEvent::Key::F4 => Key::F4,
        crate::HexgemEvent::Key::F5 => Key::F5,
        crate::HexgemEvent::Key::F6 => Key::F6,
        crate::HexgemEvent::Key::F7 => Key::F7,
        crate::HexgemEvent::Key::F8 => Key::F8,
        crate::HexgemEvent::Key::F9 => Key::F9,
        crate::HexgemEvent::Key::F10 => Key::F10,
        crate::HexgemEvent::Key::F11 => Key::F11,
        crate::HexgemEvent::Key::F12 => Key::F12,
        crate::HexgemEvent::Key::F13 => Key::F13,
        crate::HexgemEvent::Key::F14 => Key::F14,
        crate::HexgemEvent::Key::F15 => Key::F15,
        crate::HexgemEvent::Key::F16 => Key::F16,
        crate::HexgemEvent::Key::F17 => Key::F17,
        crate::HexgemEvent::Key::F18 => Key::F18,
        crate::HexgemEvent::Key::F19 => Key::F19,
        _ => Key::F20,
    }
}
pub fn translate_modifiers(keymod: &Modifier) -> egui::Modifiers {
    egui::Modifiers {
        alt: keymod.has(Mod::Alt),
        ctrl: keymod.has(Mod::Control),
        shift: keymod.has(Mod::Shift),
        command: keymod.has(Mod::ModeLock),
        ..Default::default()
    }
}
pub trait HexgemEventToEgui: Event {
    fn into_egui_event(&self, state: &mut EguiStateInput) -> egui::Event;
}

impl HexgemEventToEgui for MouseButtonEvent {
    fn into_egui_event(&self, state: &mut EguiStateInput) -> egui::Event {
        egui::Event::PointerButton {
            pos: state.pointer_pos,
            button: match self.button {
                crate::HexgemEvent::MouseButton::LeftButton => egui::PointerButton::Primary,
                crate::HexgemEvent::MouseButton::RightButton => egui::PointerButton::Secondary,
                crate::HexgemEvent::MouseButton::MiddleButton => egui::PointerButton::Middle,
                crate::HexgemEvent::MouseButton::Button4 => egui::PointerButton::Extra1,
                crate::HexgemEvent::MouseButton::Button5 => egui::PointerButton::Extra2,
                _ => egui::PointerButton::Extra2,
            },
            pressed: self.pressed,
            modifiers: translate_modifiers(&self.modifiers),
        }
    }
}

impl HexgemEventToEgui for MouseMoveEvent {
    fn into_egui_event(&self, state: &mut EguiStateInput) -> egui::Event {
        let pointer_pos = pos2(self.position.x as f32, self.position.y as f32);
        egui::Event::PointerMoved(pointer_pos)
    }
}

impl HexgemEventToEgui for KeyboardEvent {
    fn into_egui_event(&self, state: &mut EguiStateInput) -> egui::Event {
        let key = translate_virtual_key_code(&self.key);
        let modifiers = translate_modifiers(&self.modifiers);
        if self.pressed {
            if state.modifiers.command && key == egui::Key::X {
                return egui::Event::Cut;
            } else if state.modifiers.command && key == egui::Key::C {
                return egui::Event::Copy;
            } else if state.modifiers.command && key == egui::Key::V {
                if let Some(clipboard_ctx) = state.clipboard.as_mut() {
                    return egui::Event::Text(
                        clipboard_ctx
                            .get_contents()
                            .unwrap_or_else(|_| "".to_string()),
                    );
                }
            }
        }
        egui::Event::Key {
            key,
            physical_key: None,
            pressed: self.pressed,
            repeat: self.repeat,
            modifiers,
        }
    }
}

impl HexgemEventToEgui for MouseScrollEvent {
    fn into_egui_event(&self, state: &mut EguiStateInput) -> egui::Event {
        egui::Event::Scroll(vec2(self.dx as f32, self.dy as f32))
    }
}
