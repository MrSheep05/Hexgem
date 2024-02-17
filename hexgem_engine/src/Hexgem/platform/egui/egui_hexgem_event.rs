use cli_clipboard::ClipboardProvider;
use egui::{pos2, vec2};
use egui_gl_glfw::{translate_virtual_key_code, EguiInputState};
use glfw::Modifiers as Mod;

use crate::HexgemEvent::{
    Event, KeyboardEvent, MouseButtonEvent, MouseMoveEvent, MouseScrollEvent,
};

pub fn translate_modifiers(keymod: Mod) -> egui::Modifiers {
    egui::Modifiers {
        alt: (keymod & Mod::Alt == Mod::Alt),
        ctrl: (keymod & Mod::Control == Mod::Control),
        shift: (keymod & Mod::Shift == Mod::Shift),
        command: (keymod & Mod::Control == Mod::Control),
        ..Default::default()
    }
}
pub trait HexgemEventToEgui: Event {
    fn into_egui_event(&self, state: &mut EguiInputState) -> egui::Event;
}

impl HexgemEventToEgui for MouseButtonEvent {
    fn into_egui_event(&self, state: &mut EguiInputState) -> egui::Event {
        egui::Event::PointerButton {
            pos: state.pointer_pos,
            button: match self.button {
                glfw::MouseButton::Button1 => egui::PointerButton::Primary,
                glfw::MouseButton::Button2 => egui::PointerButton::Secondary,
                glfw::MouseButton::Button3 => egui::PointerButton::Middle,
                glfw::MouseButton::Button4 => egui::PointerButton::Extra1,
                _ => egui::PointerButton::Extra2,
            },
            pressed: self.pressed,
            modifiers: translate_modifiers(self.modifiers),
        }
    }
}

impl HexgemEventToEgui for MouseMoveEvent {
    fn into_egui_event(&self, state: &mut EguiInputState) -> egui::Event {
        let pointer_pos = pos2(self.position.x as f32, self.position.y as f32);
        egui::Event::PointerMoved(pointer_pos)
    }
}

impl HexgemEventToEgui for KeyboardEvent {
    fn into_egui_event(&self, state: &mut EguiInputState) -> egui::Event {
        let key = translate_virtual_key_code(self.key).unwrap_or(egui::Key::F20);
        let modifiers = translate_modifiers(self.modifiers);
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
    fn into_egui_event(&self, state: &mut EguiInputState) -> egui::Event {
        egui::Event::Scroll(vec2(self.dx as f32, self.dy as f32))
    }
}
