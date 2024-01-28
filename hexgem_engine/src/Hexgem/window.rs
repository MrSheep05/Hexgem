use glfw::{flush_messages, Glfw, WindowEvent};
use log::info;

use crate::HexgemEvent::{
    Event, KeyboardEvent, MouseButtonEvent, MouseMoveEvent, MouseScrollEvent, NoneEvent,
    WindowCloseEvent, WindowFocusEvent, WindowMoveEvent, WindowResizeEvent,
};

use super::core::{Position, Size};

pub struct WindowProps {
    pub title: &'static str,
    pub width: u32,
    pub height: u32,
}

impl Default for WindowProps {
    fn default() -> Self {
        Self {
            title: "HexgemApp",
            width: 1280,
            height: 720,
        }
    }
}

pub trait Window {
    fn create(props: WindowProps) -> Box<dyn Window>
    where
        Self: Sized;
    fn is_vsync(&self) -> bool;
    fn get_width(&self) -> i32;
    fn get_height(&self) -> i32;
    fn get_mut(&mut self) -> Box<&mut dyn Window>;
    fn get_glfw(&self) -> &Glfw;
    fn get_window(&mut self) -> &mut glfw::PWindow;
    fn on_update(&mut self, callback: &mut dyn FnMut(Box<dyn Event>, Box<&mut dyn Window>));
    fn set_vsync(&mut self, enabled: bool);
    // fn event_callback(&self) -> &dyn FnMut(Box<dyn Event>);
    // fn set_event_callback(&mut self, callback: Box<dyn FnMut(Box<dyn Event>)>);

    fn get_event(&self, event: Option<WindowEvent>) -> Box<dyn Event> {
        let hexgemEvent: Box<dyn Event> = if let Some(event_some) = event {
            match event_some {
                WindowEvent::Pos(x, y) => Box::new(WindowMoveEvent::create(Position { x, y })),
                WindowEvent::Size(width, height) => {
                    Box::new(WindowResizeEvent::create(Size { width, height }))
                }
                WindowEvent::Close => Box::new(WindowCloseEvent::create()),

                // WindowEvent::Refresh => todo!(),
                WindowEvent::Focus(is_focused) => Box::new(WindowFocusEvent::create(is_focused)),
                WindowEvent::MouseButton(button, action, modifiers) => Box::new(match action {
                    glfw::Action::Release => {
                        MouseButtonEvent::create(false, false, button, modifiers)
                    }
                    glfw::Action::Press => MouseButtonEvent::create(true, false, button, modifiers),
                    glfw::Action::Repeat => MouseButtonEvent::create(true, true, button, modifiers),
                }),
                WindowEvent::CursorPos(x, y) => Box::new(MouseMoveEvent::create(Position { x, y })),
                WindowEvent::Scroll(dx, dy) => Box::new(MouseScrollEvent::create(dx, dy)),
                WindowEvent::Key(key, code, action, modifiers) => Box::new(match action {
                    glfw::Action::Release => KeyboardEvent::create(false, key, false, modifiers),
                    glfw::Action::Press => KeyboardEvent::create(true, key, false, modifiers),
                    glfw::Action::Repeat => KeyboardEvent::create(true, key, true, modifiers),
                }),
                // WindowEvent::Maximize(isFullSize) => todo!(),
                _ => Box::new(NoneEvent::create()),
            }
        } else {
            Box::new(NoneEvent::create())
        };
        return hexgemEvent;
    }
}
