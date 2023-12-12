use crate::EventsNew::{Event, EventDispatcher, EventType, WindowCloseEvent};

use super::{layer::Layer, layer_stack::LayarStack, window_new::Window};

pub trait HexgemApp: Sized {
    fn application() -> Application;
    fn run(&self) {
        Self::application().run(self);
    }
}
pub struct Application {
    window: Window,
    layer_stack: LayarStack,
}

impl Application {
    pub fn create() -> Self {
        todo!();
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

    fn on_event(&self, event: &'static impl Event) {
        let eventDispatcher = EventDispatcher::from(Box::new(event));
        eventDispatcher.dispatch::<WindowCloseEvent>(EventType::WindowClose, |e| {})
    }
    fn run(&self, app: &impl HexgemApp) {}
}
