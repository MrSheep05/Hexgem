use crate::{Hexgem::hexgem_events::*, Unwrap};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use super::event_emitter::EventEmitter;

pub trait EventHandler {
    fn handle_event(&self, event: &HexgemEvent) {}
    fn handle_category(&self, event_category: &EventCategory) {}
}

macro_rules! eventDispatch {
    ($b:ident,$a: expr) => {
        $b.handle_event($a);
        $b.handle_category(&$a.get_category());
    };
    ($b:ident) => {
        $b.handle_event(&HexgemEvent::None);
        $b.handle_category(&EventCategory::None);
    };
}
pub struct EventLayer {
    pub window: Window,
}
impl EventLayer {
    pub fn init(event_handler: &impl EventHandler, event_emitter: &EventEmitter) -> Self {
        let event_loop = EventLoop::new().get("Cannot create new event loop");
        event_loop.set_control_flow(ControlFlow::Poll);
        let window = WindowBuilder::new()
            .build(&event_loop)
            .get("Cannot create new window");
        event_loop
            .run(move |event, _| match event {
                Event::WindowEvent {
                    event: window_event,
                    ..
                } => match window_event {
                    WindowEvent::KeyboardInput {
                        event: key_event, ..
                    } => dispatch_key_event(key_event, event_handler, event_emitter),
                    WindowEvent::CursorMoved { position, .. } => {
                        let hexgem_event =
                            HexgemEvent::MouseMoved(mouse_events::MouseMovedEvent { position });
                        event_emitter.emit(&hexgem_event);
                        eventDispatch!(event_handler, &hexgem_event);
                    }
                    WindowEvent::MouseWheel { delta, phase, .. } => {
                        let hexgem_event =
                            HexgemEvent::MouseScrolled(mouse_events::MouseScrollEvent {
                                scroll_delta: delta,
                                phase,
                            });
                        event_emitter.emit(&hexgem_event);
                        eventDispatch!(event_handler, &hexgem_event);
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        dispatch_mouse_input_event(event_emitter, event_handler, state, button)
                    }
                    WindowEvent::CloseRequested => {
                        let hexgem_event = HexgemEvent::WindowClose;
                        event_emitter.emit(&hexgem_event);
                        eventDispatch!(event_handler, &hexgem_event);
                    }
                    WindowEvent::Resized(size) => {
                        let hexgem_event =
                            HexgemEvent::WindowResize(input_events::WindowResizeEvent { size });
                        event_emitter.emit(&hexgem_event);
                        eventDispatch!(event_handler, &hexgem_event);
                    }
                    // WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer } //TODO
                    WindowEvent::Focused(is_focused) => {
                        dispatch_focus_event(event_emitter, event_handler, is_focused)
                    }
                    WindowEvent::Moved(position) => {
                        let hexgem_event =
                            HexgemEvent::WindowMoved(input_events::WindowMoveEvent { position });
                        event_emitter.emit(&hexgem_event);
                        eventDispatch!(event_handler, &hexgem_event);
                    }
                    _ => {
                        eventDispatch!(event_handler);
                    }
                },
                _ => {
                    eventDispatch!(event_handler);
                }
            })
            .get("Cannot perform run of event_loop");
        return Self { window };
    }
}

fn dispatch_key_event(
    event: winit::event::KeyEvent,
    event_handler: &impl EventHandler,
    event_emitter: &EventEmitter,
) {
    let key_event = keyboard_events::KeyEvent {
        key: event.physical_key,
        repeat: event.repeat,
        location: event.location,
    };
    match event.state {
        ElementState::Pressed => {
            let hexgem_event = HexgemEvent::KeyPressed(key_event.clone());
            event_emitter.emit(&hexgem_event);
            eventDispatch!(event_handler, &hexgem_event);
        }
        ElementState::Released => {
            let hexgem_event = HexgemEvent::KeyReleased(key_event.clone());
            event_emitter.emit(&hexgem_event);

            eventDispatch!(event_handler, &hexgem_event);
        }
    }
}

fn dispatch_mouse_input_event(
    event_emitter: &EventEmitter,
    event_handler: &impl EventHandler,
    state: ElementState,
    button: MouseButton,
) {
    match state {
        ElementState::Pressed => {
            let hexgem_event =
                HexgemEvent::MouseButtonPressed(mouse_events::MouseButtonEvent { button });
            event_emitter.emit(&hexgem_event);

            eventDispatch!(event_handler, &hexgem_event);
        }
        ElementState::Released => {
            let hexgem_event =
                HexgemEvent::MouseButtonReleased(mouse_events::MouseButtonEvent { button });
            event_emitter.emit(&hexgem_event);

            eventDispatch!(event_handler, &hexgem_event);
        }
    }
}

fn dispatch_focus_event(
    event_emitter: &EventEmitter,
    event_handler: &impl EventHandler,
    is_focused: bool,
) {
    if is_focused {
        let hexgem_event = HexgemEvent::WindowFocus;
        event_emitter.emit(&hexgem_event);
        eventDispatch!(event_handler, &hexgem_event);
    } else {
        let hexgem_event = HexgemEvent::WindowLostFocus;
        event_emitter.emit(&hexgem_event);
        eventDispatch!(event_handler, &hexgem_event);
    }
}
