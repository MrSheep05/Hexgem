use crate::Unwrap;

use super::event_types::HexgemEvent;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
pub trait EventHandler {
    fn handleEvent(&self, event: HexgemEvent);
    fn _handleEvent(&self, event_option: Option<HexgemEvent>) {
        let event = event_option.unwrap_or(HexgemEvent::None);
        self.handleEvent(event);
    }
}
macro_rules! handleEvent {
    ($b:ident,$a: expr) => {
        $b._handleEvent(Some($a))
    };
    ($b:ident) => {
        $b._handleEvent(None)
    };
}
pub struct EventLayer {}
impl EventLayer {
    pub fn init(event_handler: &impl EventHandler) -> Window {
        let event_loop = EventLoop::new().get("Cannot create new event loop");
        event_loop.set_control_flow(ControlFlow::Poll);
        let window = WindowBuilder::new()
            .build(&event_loop)
            .get("Cannot create new window");

        event_loop
            .run(move |event, _| match event {
                Event::WindowEvent {
                    window_id,
                    event: window_event,
                } => match window_event {
                    _ => handleEvent!(event_handler),
                },
                _ => handleEvent!(event_handler),
            })
            .get("Cannot perform run of event_loop");
        return window;
    }
}
