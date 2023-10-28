use crate::{Hexgem::hexgem_events::*, Unwrap};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub trait EventHandler {
    fn handle_event(&self, event: HexgemEvent) {}
    fn handle_category(&self, event_category: EventCategory) {}
}

macro_rules! eventDispatch {
    ($b:ident,$a: expr) => {
        $b.handle_event($a);
        $b.handle_category($a.get_category());
    };
    ($b:ident) => {
        $b.handle_event(HexgemEvent::None);
        $b.handle_category(EventCategory::None);
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
                    event: window_event,
                    ..
                } => match window_event {
                    WindowEvent::KeyboardInput {
                        event: key_event, ..
                    } => dispatch_key_event(key_event, event_handler),
                    WindowEvent::CursorMoved { position, .. } => {
                        eventDispatch!(
                            event_handler,
                            HexgemEvent::MouseMoved {
                                mouse_event: mouse_events::MouseMovedEvent { position }
                            }
                        );
                    }
                    WindowEvent::MouseWheel { delta, phase, .. } => {
                        eventDispatch!(
                            event_handler,
                            HexgemEvent::MouseScrolled {
                                mouse_event: mouse_events::MouseScrollEvent {
                                    scroll_delta: delta,
                                    phase
                                }
                            }
                        );
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        dispatch_mouse_input_event(event_handler, state, button)
                    }
                    WindowEvent::CloseRequested => {
                        eventDispatch!(event_handler, HexgemEvent::WindowClose);
                    }
                    WindowEvent::Resized(size) => {
                        eventDispatch!(
                            event_handler,
                            HexgemEvent::WindowResize {
                                resize_event: input_events::WindowResizeEvent { size }
                            }
                        );
                    }
                    // WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer } //TODO
                    WindowEvent::Focused(is_focused) => {
                        dispatch_focus_event(event_handler, is_focused)
                    }
                    WindowEvent::Moved(position) => {
                        eventDispatch!(
                            event_handler,
                            HexgemEvent::WindowMoved {
                                move_event: input_events::WindowMoveEvent { position }
                            }
                        );
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
        return window;
    }
}

fn dispatch_key_event(event: winit::event::KeyEvent, event_handler: &impl EventHandler) {
    let key_event = keyboard_events::KeyEvent {
        key: event.physical_key,
        repeat: event.repeat,
        location: event.location,
    };
    match event.state {
        ElementState::Pressed => {
            eventDispatch!(
                event_handler,
                HexgemEvent::KeyPressed {
                    key_event: key_event.clone()
                }
            );
        }
        ElementState::Released => {
            eventDispatch!(
                event_handler,
                HexgemEvent::KeyReleased {
                    key_event: key_event.clone()
                }
            );
        }
    }
}

fn dispatch_mouse_input_event(
    event_handler: &impl EventHandler,
    state: ElementState,
    button: MouseButton,
) {
    match state {
        ElementState::Pressed => {
            eventDispatch!(
                event_handler,
                HexgemEvent::MouseButtonPressed {
                    mouse_button_event: mouse_events::MouseButtonEvent { button },
                }
            );
        }
        ElementState::Released => {
            eventDispatch!(
                event_handler,
                HexgemEvent::MouseButtonReleased {
                    mouse_button_event: mouse_events::MouseButtonEvent { button }
                }
            );
        }
    }
}

fn dispatch_focus_event(event_handler: &impl EventHandler, is_focused: bool) {
    if is_focused {
        eventDispatch!(event_handler, HexgemEvent::WindowFocus);
    } else {
        eventDispatch!(event_handler, HexgemEvent::WindowLostFocus);
    }
}
