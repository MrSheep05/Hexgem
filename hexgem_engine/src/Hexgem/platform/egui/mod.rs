mod egui_clipboard;
mod egui_event_handler;
mod egui_hexgem_event;
mod egui_painter;
mod egui_state;
pub mod EguiPlatform {
    pub use super::egui_event_handler::HexgemEventHandler;
    pub use super::egui_hexgem_event::HexgemEventToEgui;
    pub use super::egui_painter::Painter;
    pub use super::egui_state::EguiStateInput;
}
