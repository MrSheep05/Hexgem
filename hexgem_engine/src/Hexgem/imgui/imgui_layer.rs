use imgui::Context as ImGuiContext;

use crate::{Hexgem::window::Window, Layer};

use super::ImGuiWindow;

type LoadFn = dyn FnMut(&'static str) -> *const ::std::os::raw::c_void;
pub struct ImGuiLayer {
    imgui_window: ImGuiWindow,
}

impl ImGuiLayer {
    pub fn new<F>(load_fn: F) -> Self
    where
        F: FnMut(&'static str) -> *const ::std::os::raw::c_void,
    {
        let imgui_window = ImGuiWindow::new(load_fn);
        Self { imgui_window }
    }
}
impl Layer for ImGuiLayer {
    fn get_name(&self) -> &'static str {
        "ImGui Layer"
    }

    fn on_event(
        &mut self,
        event: &mut Box<dyn crate::HexgemEvent::Event>,
        window: &mut dyn Window,
    ) {
        self.imgui_window.handle_event(event, window);
    }

    fn on_attach(&mut self) {}

    fn on_detach(&mut self) {}

    fn on_update(&mut self, window: &mut Box<dyn Window>) {
        //TODO move the imgui or make it option
        self.imgui_window
            .draw(window, |ui| ui.show_demo_window(&mut true));
    }
}
