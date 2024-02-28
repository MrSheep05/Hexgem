use log::{error, info};
use sdl2::{
    pixels::Color,
    video::{GLContext, GLProfile},
};

use crate::{
    Hexgem::core::{Position, Size},
    HexgemEvent::{
        Event, Key, KeyboardEvent, Modifiers, MouseButton, MouseButtonEvent, MouseMoveEvent,
        MouseScrollEvent, NoneEvent, WindowCloseEvent, WindowFocusEvent, WindowMoveEvent,
        WindowResizeEvent,
    },
    Window,
};

pub struct SdlWindow {
    pub video_subsystem: sdl2::VideoSubsystem,
    context: sdl2::Sdl,
    pub gl_context: GLContext,
    // window: sdl2::video::Window,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: Option<sdl2::EventPump>,
    vsync: bool,
}

impl SdlWindow {
    fn get_event(event: Option<sdl2::event::Event>) -> Box<dyn Event> {
        let none = Box::new(NoneEvent::create());
        match event {
            Some(event) => match event {
                sdl2::event::Event::Window { win_event, .. } => match win_event {
                    sdl2::event::WindowEvent::Moved(x, y) => {
                        Box::new(WindowMoveEvent::create(Position { x, y }))
                    }
                    sdl2::event::WindowEvent::SizeChanged(width, height)
                    | sdl2::event::WindowEvent::Resized(width, height) => {
                        Box::new(WindowResizeEvent::create(Size { width, height }))
                    }
                    sdl2::event::WindowEvent::FocusGained => {
                        Box::new(WindowFocusEvent::create(true))
                    }
                    sdl2::event::WindowEvent::FocusLost => {
                        Box::new(WindowFocusEvent::create(false))
                    }
                    sdl2::event::WindowEvent::Close => Box::new(WindowCloseEvent::create()),
                    _ => none,
                },
                sdl2::event::Event::KeyDown {
                    keycode,
                    keymod,
                    repeat,
                    ..
                } => Box::new(KeyboardEvent::create(
                    true,
                    if let Some(key) = keycode {
                        Key::from(key)
                    } else {
                        Key::Unknown
                    },
                    repeat,
                    Modifiers::from(keymod),
                )),
                sdl2::event::Event::KeyUp {
                    keycode,
                    keymod,
                    repeat,
                    ..
                } => Box::new(KeyboardEvent::create(
                    false,
                    if let Some(key) = keycode {
                        Key::from(key)
                    } else {
                        Key::Unknown
                    },
                    repeat,
                    Modifiers::from(keymod),
                )),
                sdl2::event::Event::MouseMotion { x, y, .. } => {
                    Box::new(MouseMoveEvent::create(Position {
                        x: x as f64,
                        y: y as f64,
                    }))
                }
                sdl2::event::Event::MouseButtonDown {
                    mouse_btn, clicks, ..
                } => Box::new(MouseButtonEvent::create(
                    true,
                    clicks > 1,
                    MouseButton::from(mouse_btn),
                    Modifiers::create(vec![]),
                )),
                sdl2::event::Event::MouseButtonUp {
                    mouse_btn, clicks, ..
                } => Box::new(MouseButtonEvent::create(
                    false,
                    clicks > 1,
                    MouseButton::from(mouse_btn),
                    Modifiers::create(vec![]),
                )),
                sdl2::event::Event::MouseWheel { x, y, .. } => {
                    Box::new(MouseScrollEvent::create(x as f64, y as f64))
                }
                _ => none,
            },
            None => none,
        }
    }
}
crate::toAnyImpl!(SdlWindow);

impl Window for SdlWindow {
    fn create(props: crate::Hexgem::window::WindowProps) -> Box<dyn Window>
    where
        Self: Sized,
    {
        let context = sdl2::init().expect("Error occured on sdl2 init!");

        let video_subsystem = context.video().expect("Cannot create video subsystem");
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_double_buffer(true);
        gl_attr.set_multisample_samples(4);
        gl_attr.set_framebuffer_srgb_compatible(true);
        gl_attr.set_context_version(3, 2);
        let window = video_subsystem
            .window(props.title, props.width, props.height)
            .opengl()
            .position_centered()
            .resizable()
            .build()
            .expect("Cannot create sdl window");
        gl::load_with(|c| window.subsystem().gl_get_proc_address(c) as *const _);
        let gl_context = window
            .gl_create_context()
            .expect("Cannot create gl context");

        let mut canvas = window.into_canvas().build().unwrap();
        let event_pump = Some(
            context
                .event_pump()
                .expect("Could not create event_pump for sdl"),
        );
        canvas.present();
        let mut sdl_window = Self {
            context,
            video_subsystem,
            gl_context,
            // window,
            canvas,
            event_pump,
            vsync: true,
        };
        sdl_window.set_vsync(true);

        info!("Created sdl window");
        Box::new(sdl_window)
    }

    fn is_vsync(&self) -> bool {
        self.vsync
    }

    fn get_width(&self) -> i32 {
        match self.canvas.output_size() {
            Ok((w, _)) => w as i32,
            Err(_) => panic!("Could not get size of output"),
        }
        // self.window.size().0 as i32
    }

    fn get_height(&self) -> i32 {
        match self.canvas.output_size() {
            Ok((_, h)) => h as i32,
            Err(_) => panic!("Could not get size of output"),
        }
        // self.window.size().1 as i32
    }

    fn get_mut(&mut self) -> Box<&mut dyn Window> {
        Box::new(self)
    }

    fn on_update(&mut self, callback: &mut dyn FnMut(Box<dyn Event>, Box<&mut dyn Window>)) {
        self.canvas.set_draw_color(Color::RGB(35, 39, 45));
        self.canvas.clear();

        self.event_pump.take().map(|mut event_pump| {
            let iter = event_pump.poll_iter();
            let mut count = 0;
            for event in iter {
                count += 1;
                callback(Self::get_event(Some(event)), self.get_mut());
            }
            if count == 0 {
                callback(Self::get_event(None), self.get_mut());
            }
            self.event_pump = Some(event_pump);
        });
        self.canvas.present();
    }

    fn set_vsync(&mut self, enabled: bool) {
        match self.video_subsystem.gl_set_swap_interval(enabled as i32) {
            Ok(_) => (),
            Err(e) => error!("Could not change vsync {}", e),
        }
    }
}
