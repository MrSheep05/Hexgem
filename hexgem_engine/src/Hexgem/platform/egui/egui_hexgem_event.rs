use egui_gl_glfw::EguiInputState;
use glfw::ffi;

use crate::HexgemEvent::{Event, MouseButtonEvent};

fn translate_modifiers(modifiers: glfw::Modifiers) -> egui::Modifiers {
    let bits = modifiers.bits();
    egui::Modifiers {
        alt: bits & ffi::MOD_ALT != 0,
        ctrl: bits & ffi::MOD_CONTROL != 0,
        shift: bits & ffi::MOD_SHIFT != 0,
        mac_cmd: bits & ffi::MOD_SUPER != 0,
        command: bits & ffi::MOD_CAPS_LOCK != 0,
    }
}
pub trait HexgemEventToEgui: Event {
    fn into_egui_event(&mut self, state: &EguiInputState) -> egui::Event;
}

impl HexgemEventToEgui for MouseButtonEvent {
    fn into_egui_event(&mut self, state: &EguiInputState) -> egui::Event {
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
