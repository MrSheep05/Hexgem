use winit::{
    event::{ElementState, KeyEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::WindowBuilder,
};

use crate::{
    HexgemEvent::{
        Event, KeyboardEvent, MouseButtonEvent, MouseMoveEvent, MouseScrollEvent, NoneEvent,
        WindowCloseEvent, WindowFocusEvent, WindowMoveEvent, WindowResizeEvent,
    },
    Unwrap,
};

type InnerWindow = winit::window::Window;

pub struct Window {
    window: InnerWindow,
    event_loop: EventLoop<()>,
}

impl Window {
    pub fn create(title: &str) -> Self {
        let event_loop = EventLoop::new().get("Cannot create new event loop");
        event_loop.set_control_flow(ControlFlow::Poll);
        let window = WindowBuilder::new()
            .with_title(title)
            .build(&event_loop)
            .get("Cannot create new window");
        Self { window, event_loop }
    }

    pub fn open<F>(self, mut on_event: F)
    where
        F: FnMut(Box<dyn Event>, &EventLoopWindowTarget<()>),
    {
        self.event_loop
            .run(|event, elwt| {
                let event_model: Box<dyn Event> = match event {
                    winit::event::Event::WindowEvent {
                        event: window_event,
                        ..
                    } => match window_event {
                        winit::event::WindowEvent::Resized(size) => {
                            Box::new(WindowResizeEvent::create(size))
                        }
                        winit::event::WindowEvent::Moved(position) => {
                            Box::new(WindowMoveEvent::create(position))
                        }
                        winit::event::WindowEvent::CloseRequested => {
                            Box::new(WindowCloseEvent::create())
                        }
                        winit::event::WindowEvent::Focused(isFocused) => {
                            Box::new(WindowFocusEvent::create(isFocused))
                        }
                        winit::event::WindowEvent::KeyboardInput {
                            event: key_event, ..
                        } => dispatch_key_event(key_event),
                        winit::event::WindowEvent::CursorMoved { position, .. } => {
                            Box::new(MouseMoveEvent::create(position))
                        }
                        winit::event::WindowEvent::MouseWheel { delta, phase, .. } => {
                            Box::new(MouseScrollEvent::create(delta, phase))
                        }
                        winit::event::WindowEvent::MouseInput { state, button, .. } => {
                            dispatch_mouse_input_event(state, button)
                        }
                        _ => Box::new(NoneEvent::create()),
                    },
                    _ => Box::new(NoneEvent::create()),
                };
                on_event(event_model, elwt);
            })
            .expect("Cannot run event_loop");
    }
}

fn dispatch_key_event(key_event: KeyEvent) -> Box<dyn Event> {
    return Box::new(match key_event.state {
        winit::event::ElementState::Pressed => KeyboardEvent::create(
            true,
            key_event.physical_key,
            key_event.location,
            key_event.repeat,
        ),
        winit::event::ElementState::Released => KeyboardEvent::create(
            false,
            key_event.physical_key,
            key_event.location,
            key_event.repeat,
        ),
    });
}

fn dispatch_mouse_input_event(
    state: ElementState,
    button: winit::event::MouseButton,
) -> Box<dyn Event> {
    return Box::new(match state {
        ElementState::Pressed => MouseButtonEvent::create(true, button),
        ElementState::Released => MouseButtonEvent::create(false, button),
    });
}
