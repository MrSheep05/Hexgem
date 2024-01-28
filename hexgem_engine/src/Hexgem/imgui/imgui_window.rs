use std::{ffi::CStr, os::raw::c_void, time::Instant};

use glfw::{ffi::GLFWwindow, MouseButton, StandardCursor};
use imgui::{ConfigFlags, Context as ImGuiContext, MouseCursor, Ui};
use imgui_opengl_renderer::Renderer;
use log::info;

use crate::{
    Hexgem::window::Window,
    HexgemEvent::{
        Event, EventDispatcher, EventType, MouseButtonEvent, MouseMoveEvent, MouseScrollEvent,
    },
};
struct GlfwClipboardBackend(*mut GLFWwindow);

impl imgui::ClipboardBackend for GlfwClipboardBackend {
    fn get(&mut self) -> Option<String> {
        let char_ptr = unsafe { glfw::ffi::glfwGetClipboardString(self.0) };
        let c_str = unsafe { CStr::from_ptr(char_ptr) };
        Some(String::from(c_str.to_str().unwrap()))
    }

    fn set(&mut self, value: &str) {
        unsafe {
            glfw::ffi::glfwSetClipboardString(self.0, value.as_ptr() as *const i8);
        };
    }
}

pub struct ImGuiWindow {
    last_frame: Instant,
    imgui_context: imgui::Context,
    renderer: Renderer,
}

impl ImGuiWindow {
    pub fn new<F>(mut load_fn: F) -> Self
    where
        F: FnMut(&'static str) -> *const ::std::os::raw::c_void,
    {
        let mut imgui = ImGuiContext::create();
        unsafe {
            let window_ptr = glfw::ffi::glfwGetCurrentContext();
            imgui.set_clipboard_backend(GlfwClipboardBackend(window_ptr));
        };

        let renderer = Renderer::new(&mut imgui, |s| load_fn(s));
        Self {
            imgui_context: imgui,
            last_frame: Instant::now(),
            renderer,
        }
    }

    pub fn handle_event(&mut self, event: &mut Box<dyn Event>, window: &mut dyn Window) {
        let dispatcher = EventDispatcher::from(event);

        dispatcher.dispatch::<MouseButtonEvent, _>(
            crate::HexgemEvent::EventType::MouseButtonPressed,
            |event| {
                let mouse_key = match event.button {
                    MouseButton::Button1
                    | glfw::MouseButton::Button6
                    | glfw::MouseButton::Button7
                    | glfw::MouseButton::Button8 => 0,
                    MouseButton::Button2 => 1,
                    MouseButton::Button3 => 2,
                    MouseButton::Button4 => 3,
                    MouseButton::Button5 => 4,
                };
                self.imgui_context.io_mut().mouse_down[mouse_key] = true;
                info!("Mouse {} button is {}", mouse_key, "down");

                None
            },
        );

        dispatcher.dispatch::<MouseButtonEvent, _>(
            crate::HexgemEvent::EventType::MouseButtonReleased,
            |event| {
                let mouse_key = match event.button {
                    MouseButton::Button1
                    | glfw::MouseButton::Button6
                    | glfw::MouseButton::Button7
                    | glfw::MouseButton::Button8 => 0,
                    MouseButton::Button2 => 1,
                    MouseButton::Button3 => 2,
                    MouseButton::Button4 => 3,
                    MouseButton::Button5 => 4,
                };
                self.imgui_context.io_mut().mouse_down[mouse_key] = false;
                info!("Mouse {} button is {}", mouse_key, "up");

                None
            },
        );

        dispatcher.dispatch::<MouseMoveEvent, _>(EventType::MouseMoved, |event| {
            self.imgui_context.io_mut().mouse_pos =
                [event.position.x as f32, event.position.y as f32];
            info!("Position set to {:?}", event.position);
            None
        });

        dispatcher.dispatch::<MouseScrollEvent, _>(EventType::MouseScrolled, |event| {
            self.imgui_context.io_mut().mouse_wheel = event.dy as f32;
            None
        });
    }

    pub fn frame(&mut self, window: &Box<dyn Window>) -> &mut imgui::Ui {
        let io = self.imgui_context.io_mut();
        let now = Instant::now();
        let time_delta = now - self.last_frame;
        let delta_s =
            (time_delta.as_secs() as u32 + time_delta.subsec_nanos()) as f32 / 1_000_000_000.0;
        io.delta_time = delta_s;
        self.last_frame = now;
        io.display_size = [
            window.get_width() as f32 / 2.,
            window.get_height() as f32 / 2.,
        ];
        self.imgui_context.new_frame()
    }

    pub fn draw<F>(&mut self, window: &mut Box<dyn Window>, mut callback: F)
    where
        F: FnMut(&mut Ui),
    {
        let ui = self.frame(window);
        let io = ui.io();
        let glfw_window = window.get_window();
        if !io
            .config_flags
            .contains(ConfigFlags::NO_MOUSE_CURSOR_CHANGE)
        {
            match ui.mouse_cursor() {
                Some(mouse_cursor) if !io.mouse_draw_cursor => {
                    glfw_window.set_cursor_mode(glfw::CursorMode::Normal);

                    let cursor = match mouse_cursor {
                        MouseCursor::TextInput => StandardCursor::IBeam,
                        MouseCursor::ResizeNS => StandardCursor::VResize,
                        MouseCursor::ResizeEW => StandardCursor::HResize,
                        MouseCursor::Hand => StandardCursor::Hand,
                        _ => StandardCursor::Arrow,
                    };
                    glfw_window.set_cursor(Some(glfw::Cursor::standard(cursor)));
                }
                _ => {
                    glfw_window.set_cursor_mode(glfw::CursorMode::Hidden);
                }
            }
        }
        callback(ui);
        self.renderer.render(&mut self.imgui_context);
    }
}
