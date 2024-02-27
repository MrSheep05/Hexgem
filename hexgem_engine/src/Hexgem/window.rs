use std::any::Any;

use crate::HexgemEvent::Event;

use super::core::ToAny;

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

pub trait Window: ToAny {
    fn create(props: WindowProps) -> Box<dyn Window>
    where
        Self: Sized;
    fn is_vsync(&self) -> bool;
    fn get_width(&self) -> i32;
    fn get_height(&self) -> i32;
    fn get_mut(&mut self) -> Box<&mut dyn Window>;
    // #[cfg(not(target_os = "macos"))]
    // fn get_glfw(&self) -> &Glfw;
    // #[cfg(not(target_os = "macos"))]
    // fn get_window(&mut self) -> &mut glfw::PWindow;
    fn on_update(&mut self, callback: &mut dyn FnMut(Box<dyn Event>, Box<&mut dyn Window>));
    fn set_vsync(&mut self, enabled: bool);
    // fn event_callback(&self) -> &dyn FnMut(Box<dyn Event>);
    // fn set_event_callback(&mut self, callback: Box<dyn FnMut(Box<dyn Event>)>);
}

pub fn get_window_struct<T: Window>(window: Box<&mut dyn Window>) -> &T {
    let any = window.as_any();
    match any.downcast_ref::<T>() {
        Some(win) => win,
        None => panic!("Cannot downcast window to that type!"),
    }
}
