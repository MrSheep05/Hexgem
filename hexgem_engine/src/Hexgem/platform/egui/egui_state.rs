use egui::{Modifiers, Pos2, RawInput};
use log::warn;

use super::egui_clipboard::{init_clipboard, ClipboardContext, ClipboardProvider};

pub struct EguiStateInput {
    pub clipboard: Option<ClipboardContext>,
    pub pointer_pos: Pos2,
    pub input: RawInput,
    pub modifiers: Modifiers,
    pub pixels_per_point: f32,
}

impl EguiStateInput {
    pub fn new(input: RawInput, pixels_per_point: f32) -> Self {
        Self {
            input,
            pixels_per_point,
            clipboard: init_clipboard(),
            pointer_pos: Pos2 { x: 0., y: 0. },
            modifiers: Modifiers::default(),
        }
    }
    pub fn process_output(&mut self, egui_output: egui::FullOutput) {
        let text = egui_output.platform_output.copied_text;
        if !text.is_empty() {
            if let Some(clipboard) = self.clipboard.as_mut() {
                match clipboard.set_contents(text) {
                    Ok(_) => (),
                    Err(e) => warn!("Could not set clipboard content from egui {e}"),
                }
            }
        }
    }
}
