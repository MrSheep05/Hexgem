mod egui_event_handler;
mod egui_hexgem_event;
pub mod EguiPlatform {

    pub use super::egui_event_handler::HexgemEventHandler;
    pub use super::egui_hexgem_event::HexgemEventToEgui;
}
