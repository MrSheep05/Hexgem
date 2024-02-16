use std::time::Instant;

use crate::{Hexgem::core::Size, Layer};
use egui_backend::egui::{vec2, Pos2, Rect};
use egui_gl_glfw as egui_backend;
use glfw::{Window, WindowEvent};

pub struct EguiWindow {
    painter: egui_backend::Painter,
    context: EguiContext,
}

impl EguiWindow {
    pub fn create(window: &mut Window) -> Self {
        let mut painter = egui_backend::Painter::new(window);
        let scale = window.get_content_scale().0;
        let (width, height) = window.get_framebuffer_size();
        let context = EguiContext::new(Size { width, height }, scale);
        Self { painter, context }
    }

    pub fn render(&mut self) {
        self.context.render();
    }

    pub fn handle_events(&mut self, event: WindowEvent) {
        egui_backend::handle_event(event, &mut self.context.input_state);
    }
}
pub struct EguiContext {
    scale: f32,
    start_time: Instant,
    context: egui::Context,
    input_state: egui_backend::EguiInputState,
}

impl EguiContext {
    pub fn new(size: Size<i32>, scale: f32) -> Self {
        let context = egui::Context::default();
        let mut input_state = egui_backend::EguiInputState::new(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    Pos2::new(0f32, 0f32),
                    vec2(size.width as f32 / 2., size.height as f32 / 2.) / scale,
                )),
                ..Default::default()
            },
            scale,
        );
        Self {
            scale,
            context,
            input_state,

            start_time: Instant::now(),
        }
    }

    pub fn render(&mut self) {
        self.input_state.input.time = Some(self.start_time.elapsed().as_secs_f64());
        self.context.begin_frame(self.input_state.input.take());
        self.input_state.pixels_per_point = self.scale;

        egui::Window::new("Egui with GLFW").show(&self.context, |ui| {
            egui::TopBottomPanel::top("Top").show(&self.context, |ui| {
                ui.menu_button("File", |ui| {
                    {
                        let _ = ui.button("test 1");
                    }
                    ui.separator();
                    {
                        let _ = ui.button("test 2");
                    }
                });
            });


            ui.label("A simple sine wave plotted onto a GL texture then blitted to an egui managed Image.");
            ui.label(" ");
            ui.text_edit_multiline(&mut "Provide Text");
            ui.label(" ");            
            ui.add(egui::Slider::new(&mut 50., 0.0..=50.0).text("Amplitude"));
            ui.label(" ");

        });
    }
}
