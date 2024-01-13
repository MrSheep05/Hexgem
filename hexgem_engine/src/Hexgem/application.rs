use log::{error, info, warn};

use crate::HexgemEvent::{
    Event, EventDispatcher, EventType, MouseButtonEvent, MouseMoveEvent, WindowCloseEvent,
};

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

    fn on_event(&mut self) -> impl FnMut(Box<dyn Event>, Box<&mut dyn Window>) + '_ {
        return |mut event: Box<dyn Event>, window: Box<&mut dyn Window>| {
            let mut handle_vector: Vec<bool> = vec![];

            {
                let event_dispatcher = EventDispatcher::from(&mut event);

                event_dispatcher.dispatch::<MouseMoveEvent, _>(EventType::MouseMoved, |e| {
                    unsafe {
                        let width = window.get_width() as f64;
                        let height = window.get_height() as f64;
                        let red = (e.position.x / width) as f32;
                        let blue = (e.position.y / height) as f32;
                        let green = (e.position.y.powi(2) + e.position.x.powi(2)).sqrt()
                            / (width.powi(2) + height.powi(2)).sqrt();
                        gl::ClearColor(red, green as f32, blue, 1.0);
                        gl::Clear(gl::COLOR_BUFFER_BIT);
                    }
                    None
                });

                handle_vector.push(event_dispatcher.dispatch::<WindowCloseEvent, _>(
                    EventType::WindowClose,
                    |_| {
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

    fn run(&mut self, _app: &impl HexgemApp) {
        info!("Running app");
        self.window.take().map(|mut window| {
            while self.running {
                let mut callback = self.on_event();
                window.on_update(&mut callback);
            }
            self.window = Some(window);
        });
    }
}
