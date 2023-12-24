use log::{error, info, warn};
use winit::{event, event_loop::EventLoopWindowTarget};

use crate::EventsNew::{Event, EventDispatcher, EventType, MouseButtonEvent, WindowCloseEvent};

use super::{layer::Layer, layer_stack::LayarStack, window_new::Window};

pub trait HexgemApp: Sized {
    fn application() -> Application;
    fn run(&self) {
        Self::application().run(self);
    }
}
pub struct Application {
    window: Option<Window>,
    // layer_stack: LayarStack,
    running: bool,
}

impl Application {
    pub fn create() -> Self {
        let window = Window::create("TEST");
        Self {
            window: Some(window),
            running: true,
        }
    }

    // pub fn push_layer<T>(&mut self, layer: T)
    // where
    //     T: Layer + 'static,
    // {
    //     self.layer_stack.push_layer(layer);
    // }

    // pub fn push_overlay<T>(&mut self, layer: T)
    // where
    //     T: Layer + 'static,
    // {
    //     self.layer_stack.push_overlay(layer);
    // }

    fn on_event(&mut self, mut event: Box<dyn Event>, elwt: &EventLoopWindowTarget<()>) {
        let mut handle_vector: Vec<bool> = vec![];

        {
            let event_dispatcher = EventDispatcher::from(&mut event, elwt);
            handle_vector.push(event_dispatcher.dispatch::<MouseButtonEvent, _>(
                EventType::MouseButtonPressed,
                |e| {
                    info!("Click");
                    None
                },
            ));
            handle_vector.push(event_dispatcher.dispatch::<WindowCloseEvent, _>(
                EventType::WindowClose,
                |e| {
                    warn!("CLOSE");
                    event_dispatcher.close();
                    None
                },
            ));
        }
        *event.handled() = handle_vector.contains(&true);
        if event.is_handled() {
            info!("HANDLED ${}", event.is_handled())
        }
        // let layers = self.layer_stack.layers();

        // for layer in layers.iter().rev() {
        //     // layer.on_event(event);
        // }
    }
    fn run(&mut self, app: &impl HexgemApp) {
        info!("Running app");
        self.window.take().map(|w| {
            w.open(|event, elwt| {
                self.on_event(event, elwt);
            })
        });
    }
}
