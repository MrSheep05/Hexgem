use crate::Layer;

use super::egui_window::EguiWindow;
use glfw::Window;

pub struct EguiLayer {
    egui_window: EguiWindow,
}

impl EguiLayer {
    pub fn create(window: &mut Window) -> Self {
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
    }

    fn on_update(&mut self, window: &mut Box<dyn crate::Window>) {
        self.egui_window.render();
    }
}
