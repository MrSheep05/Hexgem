use std::cell::RefCell;

use log::{error, info, warn};

use crate::HexgemEvent::{Event, EventDispatcher, EventType, MouseButtonEvent, WindowCloseEvent};

use super::{
    layer::Layer,
    layer_stack::LayerStack,
    platform::MacOSWindow,
    window::{Window, WindowProps},
};

pub trait HexgemApp: Sized {
    fn application() -> Application;
    fn run<T>(&self, mut callback: T)
    where
        T: FnMut(&mut Application),
    {
        let mut application = Self::application();
        callback(&mut application);
        application.run(self);
    }
}
pub struct Application {
    window: Option<Box<dyn Window>>,
    layer_stack: LayerStack,
    running: bool,
}

impl Application {
    pub fn create() -> Self {
        let window = Some(MacOSWindow::create(WindowProps::default()));
        let layer_stack = LayerStack::create();
        Self {
            layer_stack,
            running: true,
            window,
        }
    }

    pub fn push_layer<T>(&mut self, layer: T)
    where
        T: Layer + 'static,
    {
        self.layer_stack.push_layer(layer);
    }

    pub fn push_overlay<T>(&mut self, layer: T)
    where
        T: Layer + 'static,
    {
        self.layer_stack.push_overlay(layer);
    }

    fn on_event(&mut self) -> impl FnMut(Box<dyn Event>) + '_ {
        return |mut event: Box<dyn Event>| {
            let mut handle_vector: Vec<bool> = vec![];

            {
                let event_dispatcher = EventDispatcher::from(&mut event);
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
                        self.running = false;
                        None
                    },
                ));
            }
            *event.handled() = handle_vector.contains(&true);
            if event.is_handled() {
                info!("HANDLED ${}", event.is_handled())
            }
            let layers = self.layer_stack.layers();

            for layer in layers.iter().rev() {
                layer.on_event(&mut event);
            }
        };
    }

    fn run(mut self, app: &impl HexgemApp) {
        self.window.take().map(|mut window| {
            while self.running {
                let mut callback = self.on_event();
                window.on_update(&mut callback);
            }
        });
    }
}
