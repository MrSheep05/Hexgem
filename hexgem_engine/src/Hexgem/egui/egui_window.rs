use crate::{
    Hexgem::{core::Size, platform::EguiPlatform::HexgemEventHandler},
    HexgemEvent::Event,
};
use egui_backend::egui::{vec2, Pos2, Rect};
use egui_gl_glfw as egui_backend;
use glfw::Window;
use std::time::Instant;

pub struct EguiWindow {
    pub context: EguiContext,
    pub painter: egui_backend::Painter,
}

impl EguiWindow {
    pub fn create(window: &mut Window) -> Self {
        let scale = window.get_content_scale().0;
        let (width, height) = window.get_framebuffer_size();
        let context = EguiContext::new(Size { width, height }, scale);
        let painter = egui_backend::Painter::new(window);
        Self { painter, context }
    }

    pub fn render(&mut self) {
        self.context.render(&mut self.painter);
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
        let input_state = egui_backend::EguiInputState::new(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    Pos2::new(0f32, 0f32),
                    vec2(size.width as f32, size.height as f32) / scale,
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

    pub fn handle_event(&mut self, event: &mut Box<(dyn Event + 'static)>) {
        self.input_state.handle_event(event);
    }

    pub fn render(&mut self, painter: &mut egui_backend::Painter) {
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
        let egui::FullOutput {
            platform_output,
            textures_delta,
            shapes,
            pixels_per_point,
            viewport_output: _,
        } = self.context.end_frame();
        if !platform_output.copied_text.is_empty() {
            egui_backend::copy_to_clipboard(&mut self.input_state, platform_output.copied_text);
        }

        let clipped_shapes = self.context.tessellate(shapes, pixels_per_point);
        painter.paint_and_update_textures(1.0, &clipped_shapes, &textures_delta);
    }
}
