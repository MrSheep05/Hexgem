use glfw::{Context, Glfw, GlfwReceiver, PWindow, WindowEvent};
use log::{error, info};

use crate::{
    Hexgem::{
        core::{Position, Size},
        window::Window,
    },
    HexgemEvent::{
        Event, Key, KeyboardEvent, Modifiers, MouseButton, MouseButtonEvent, MouseMoveEvent,
        MouseScrollEvent, NoneEvent, WindowCloseEvent, WindowFocusEvent, WindowMoveEvent,
        WindowResizeEvent,
    },
};

pub struct GlfwWindow {
    vsync_on: bool,
    glfw: Glfw,
    window: PWindow,
    events: Option<GlfwReceiver<(f64, WindowEvent)>>,
}

impl GlfwWindow {
    fn get_event(event: Option<WindowEvent>) -> Box<dyn Event> {
        let hexgem_event: Box<dyn Event> = if let Some(event_some) = event {
            match event_some {
                WindowEvent::Pos(x, y) => Box::new(WindowMoveEvent::create(Position { x, y })),
                WindowEvent::Size(width, height) => {
                    Box::new(WindowResizeEvent::create(Size { width, height }))
                }
                WindowEvent::Close => Box::new(WindowCloseEvent::create()),

                // WindowEvent::Refresh => todo!(),
                WindowEvent::Focus(is_focused) => Box::new(WindowFocusEvent::create(is_focused)),
                WindowEvent::MouseButton(button, action, modifiers) => Box::new(match action {
                    glfw::Action::Release => MouseButtonEvent::create(
                        false,
                        false,
                        MouseButton::from(button),
                        Modifiers::from(modifiers),
                    ),
                    glfw::Action::Press => MouseButtonEvent::create(
                        true,
                        false,
                        MouseButton::from(button),
                        Modifiers::from(modifiers),
                    ),
                    glfw::Action::Repeat => MouseButtonEvent::create(
                        true,
                        true,
                        MouseButton::from(button),
                        Modifiers::from(modifiers),
                    ),
                }),
                WindowEvent::CursorPos(x, y) => Box::new(MouseMoveEvent::create(Position { x, y })),
                WindowEvent::Scroll(dx, dy) => Box::new(MouseScrollEvent::create(dx, dy)),
                WindowEvent::Key(key, code, action, modifiers) => Box::new(match action {
                    glfw::Action::Release => KeyboardEvent::create(
                        false,
                        Key::from(key),
                        false,
                        Modifiers::from(modifiers),
                    ),
                    glfw::Action::Press => KeyboardEvent::create(
                        true,
                        Key::from(key),
                        false,
                        Modifiers::from(modifiers),
                    ),
                    glfw::Action::Repeat => KeyboardEvent::create(
                        true,
                        Key::from(key),
                        true,
                        Modifiers::from(modifiers),
                    ),
                }),
                // WindowEvent::Maximize(isFullSize) => todo!(),
                _ => Box::new(NoneEvent::create()),
            }
        } else {
            Box::new(NoneEvent::create())
        };
        return hexgem_event;
    }
}

impl Window for GlfwWindow {
    fn create(
        props: crate::Hexgem::window::WindowProps,
    ) -> Box<(dyn crate::Hexgem::window::Window + 'static)> {
        let mut glfw =
            glfw::init(|err, description| error!("Error occured on glfw init - {}", description))
                .expect("Could not init glfw!");

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
        info!("Created GLFW window");
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
                let hexgem_event = Self::get_event(Some(event));
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
    #[cfg(not(target_os = "macos"))]

    fn get_glfw(&self) -> &Glfw {
        &self.glfw
    }

    #[cfg(not(target_os = "macos"))]
    fn get_window(&mut self) -> &mut glfw::PWindow {
        &mut self.window
    }
}
