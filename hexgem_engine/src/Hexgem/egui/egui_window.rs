use egui::{vec2, Image, Pos2, Rect};
use log::info;

use crate::{
    Hexgem::{
        core::Size,
        platform::EguiPlatform::{EguiStateInput, HexgemEventHandler, Painter},
    },
    HexgemEvent::Event,
    Window,
};
use std::time::Instant;

pub struct EguiWindow {
    pub context: EguiContext,
    pub painter: Painter,
}

impl EguiWindow {
    pub fn create(window: &Box<dyn Window>) -> Self {
        let painter = Painter::new(window);
        let (width, height) = (window.get_width(), window.get_height());
        let context = EguiContext::new(Size { width, height }, painter.get_scale());
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
    input_state: EguiStateInput,
}

impl EguiContext {
    pub fn new(size: Size<i32>, scale: f32) -> Self {
        let context = egui::Context::default();
        let input_state = EguiStateInput::new(
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

    pub fn render(&mut self, painter: &mut Painter) {
        self.input_state.input.time = Some(self.start_time.elapsed().as_secs_f64());
        self.context.begin_frame(self.input_state.input.take());
        self.input_state.pixels_per_point = self.scale;
        egui::Window::new("Egui with GLFW").show(&self.context, |ui| {
            ui.separator();
            ui.label("A simple sine wave plotted onto a GL texture then blitted to an egui managed Image.");
           
            if ui.button("Quit").clicked() {
                info!("Exit");
            }
        });
        let output = self.context.end_frame();
        self.input_state.process_output(&output);

        painter.paint_and_update(output, &self.context);
    }
}
