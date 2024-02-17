use glfw::{Context, Glfw, GlfwReceiver, PWindow, WindowEvent};
use log::{error, info};
use std::mem;

use crate::{
    Hexgem::window::Window,
    HexgemEvent::{Event, EventType, NoneEvent},
};

pub struct MacOSWindow {
    vsync_on: bool,
    glfw: Glfw,
    window: PWindow,
    events: Option<GlfwReceiver<(f64, WindowEvent)>>,
}

impl Window for MacOSWindow {
    fn create(
        props: crate::Hexgem::window::WindowProps,
    ) -> Box<(dyn crate::Hexgem::window::Window + 'static)> {
        let mut glfw =
            glfw::init(|err, description| error!("Error occured on glfw init - {}", description))
                .expect("Could not init glfw!");
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        let (mut window, events) = glfw
            .create_window(
                props.width,
                props.height,
                props.title,
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");
        window.make_current();
        gl::load_with(|s| glfw.get_proc_address_raw(s));
        window.set_all_polling(true);
        let mut os_window = Self {
            vsync_on: false,
            glfw,
            window,
            events: Some(events),
        };
        os_window.set_vsync(true);
        Box::new(os_window)
    }

    fn is_vsync(&self) -> bool {
        self.vsync_on
    }

    fn get_width(&self) -> i32 {
        self.window.get_size().0
    }

    fn get_height(&self) -> i32 {
        self.window.get_size().1
    }

    fn on_update(&mut self, mut callback: &mut dyn FnMut(Box<dyn Event>, Box<&mut dyn Window>)) {
        self.window.swap_buffers();
        self.glfw.poll_events();
        let mut count = 0;
        unsafe {
            gl::ClearColor(0.13, 0.15, 0.18, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        self.events.take().map(|events| {
            for (_, event) in glfw::flush_messages(&events) {
                count += 1;
                let hexgem_event = self.get_event(Some(event));
                callback(hexgem_event, self.get_mut());
            }
            if count == 0 {
                callback(Box::new(NoneEvent::create()), self.get_mut());
            }
            self.events = Some(events);
        });
    }

    fn set_vsync(&mut self, enabled: bool) {
        if enabled {
            self.glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
        } else {
            self.glfw.set_swap_interval(glfw::SwapInterval::None);
        }
        self.vsync_on = enabled;
    }

    fn get_mut(&mut self) -> Box<&mut dyn Window> {
        Box::new(self)
    }

    fn get_glfw(&self) -> &Glfw {
        &self.glfw
    }

    fn get_window(&mut self) -> &mut glfw::PWindow {
        &mut self.window
    }
}
