use egui::pos2;

use crate::HexgemEvent::{
    EventDispatcher, EventType, KeyboardEvent, MouseButtonEvent, MouseMoveEvent, MouseScrollEvent,
};

use super::{
    egui_hexgem_event::{translate_modifiers, HexgemEventToEgui},
    egui_state::EguiStateInput,
};

pub trait HexgemEventHandler {
    fn handle_event(&mut self, event: &mut Box<dyn crate::HexgemEvent::Event>);
}

impl HexgemEventHandler for EguiStateInput {
    fn handle_event(&mut self, event: &mut Box<dyn crate::HexgemEvent::Event>) {
        let handler = EventDispatcher::from(event);
        //MOUSE BUTTON
        handler.dispatch::<MouseButtonEvent, _>(EventType::MouseButtonPressed, |e| {
            let event = e.into_egui_event(self);
            self.input.events.push(event);
            None
        });
        handler.dispatch::<MouseButtonEvent, _>(EventType::MouseButtonReleased, |e| {
            let event = e.into_egui_event(self);
            self.input.events.push(event);
            None
        });
        //CURS POS
        handler.dispatch::<MouseMoveEvent, _>(EventType::MouseMoved, |e| {
            let event = e.into_egui_event(self);
            self.pointer_pos = pos2(e.position.x as f32, e.position.y as f32);
            self.input.events.push(event);
            None
        });
        //KEY
        handler.dispatch::<KeyboardEvent, _>(EventType::KeyPressed, |e| {
            let event = e.into_egui_event(self);
            self.modifiers = translate_modifiers(&e.modifiers);
            self.input.events.push(event);
            None
        });
        handler.dispatch::<KeyboardEvent, _>(EventType::KeyReleased, |e| {
            let event = e.into_egui_event(self);
            self.modifiers = translate_modifiers(&e.modifiers);
            self.input.events.push(event);
            None
        });
        //SCROLL
        handler.dispatch::<MouseScrollEvent, _>(EventType::MouseScrolled, |e| {
            let event = e.into_egui_event(self);
            self.input.events.push(event);
            None
        });
    }
}
