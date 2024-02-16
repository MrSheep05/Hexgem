use egui_gl_glfw::EguiInputState;

use crate::HexgemEvent::{EventDispatcher, EventType, MouseButtonEvent};

pub trait HexgemEventHandler {
    fn handle_event(&mut self, event: &mut Box<dyn crate::HexgemEvent::Event>);
}

impl HexgemEventHandler for EguiInputState {
    fn handle_event(&mut self, event: &mut Box<dyn crate::HexgemEvent::Event>) {
        let handler = EventDispatcher::from(event);
        //
    }
}
