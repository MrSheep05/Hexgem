use log::{info, warn};

use crate::HexgemEvent::{
    Event, EventDispatcher, EventType, Key, KeyboardEvent, Mod, WindowCloseEvent,
};

use super::{
    layer::Layer,
    layer_stack::LayerStack,
    platform::HexgemWindow::*,
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
    pub window: Option<Box<dyn Window>>,
    layer_stack: LayerStack,
    running: bool,
}

impl Application {
    pub fn create() -> Self {
        let window = {
            let props = WindowProps::default();
            #[cfg(not(target_os = "macos"))]
            {
                Some(GlfwWindow::create(props))
            }
            #[cfg(target_os = "macos")]
            {
                Some(SdlWindow::create(props))
            }
        };
        let layer_stack = LayerStack::create();
        Self {
            layer_stack,
            running: true,
            window,
        }
    }

    pub fn push_layer<T>(&mut self, mut layer: T)
    where
        T: Layer + 'static,
    {
        layer.on_attach();
        self.layer_stack.push_layer(layer);
    }

    pub fn push_overlay<T>(&mut self, mut layer: T)
    where
        T: Layer + 'static,
    {
        layer.on_attach();
        self.layer_stack.push_overlay(layer);
    }

    fn on_event(&mut self) -> impl FnMut(Box<dyn Event>, Box<&mut dyn Window>) + '_ {
        return |mut event: Box<dyn Event>, window: Box<&mut dyn Window>| {
            let mut handle_vector: Vec<bool> = vec![];

            {
                let event_dispatcher = EventDispatcher::from(&mut event);

                // event_dispatcher.dispatch::<MouseMoveEvent, _>(EventType::MouseMoved, |e| {
                //     unsafe {
                //         let width = window.get_width() as f64;
                //         let height = window.get_height() as f64;
                //         let red = (e.position.x / width) as f32;
                //         let blue = (e.position.y / height) as f32;
                //         let green = (e.position.y.powi(2) + e.position.x.powi(2)).sqrt()
                //             / (width.powi(2) + height.powi(2)).sqrt();
                // gl::ClearColor(red, green as f32, blue, 1.0);
                // gl::Clear(gl::COLOR_BUFFER_BIT);
                //     }
                //     None
                // });
                event_dispatcher.dispatch::<KeyboardEvent, _>(EventType::KeyPressed, |e| {
                    info!("{:?}", e.modifiers.has(Mod::Gui));
                    if e.modifiers.has(Mod::Gui) && e.key == Key::Q {
                        warn!("CLOSE using cmd+q");
                        self.running = false;
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
            let mut layers = self.layer_stack.layers();

            for layer in layers.iter_mut().rev() {
                if event.is_handled() {
                    info!("HANDLED ${}", event.is_handled());

                    break;
                }
                layer.on_event(&mut event, *window);
            }
        };
    }

    fn run(&mut self, _app: &impl HexgemApp) {
        info!("Running app");
        self.window.take().map(|mut window| {
            while self.running {
                for layer in self.layer_stack.layers() {
                    layer.on_update(&mut window);
                }
                let mut callback = self.on_event();
                window.on_update(&mut callback);
            }
            self.window = Some(window);
        });
    }

    pub fn get_window(&self) -> &Option<Box<dyn Window>> {
        &self.window
    }

    pub fn get_mut_window<F>(&mut self, mut callback: F)
    where
        F: FnMut(Option<Box<&mut dyn Window>>),
    {
        self.window.take().map(|mut window| {
            callback(Some(window.get_mut()));
            self.window = Some(window);
        });
    }
}
