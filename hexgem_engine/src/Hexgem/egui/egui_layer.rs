use crate::{Layer, Window};

use super::egui_window::EguiWindow;

pub struct EguiLayer {
    egui_window: EguiWindow,
}

impl EguiLayer {
    pub fn create(window: &Box<dyn Window>) -> Self {
        let egui_window = EguiWindow::create(window);
        Self { egui_window }
    }
}

impl Layer for EguiLayer {
    fn get_name(&self) -> &'static str {
        "egui Layer"
    }

    fn on_event(
        &mut self,
        event: &mut Box<dyn crate::HexgemEvent::Event>,
        window: &mut dyn crate::Window,
    ) {
        self.egui_window.context.handle_event(event);
    }

    fn on_update(&mut self, window: &mut Box<dyn crate::Window>) {
        self.egui_window.render();
    }
}
