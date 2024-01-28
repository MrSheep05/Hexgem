use log::info;

use crate::HexgemEvent::Event;

use super::window::Window;

pub trait Layer {
    fn get_name(&self) -> &'static str;
    fn on_attach(&mut self) {
        info!("{} layer has been attached", self.get_name());
    }
    fn on_detach(&mut self) {
        info!("{} layer has been detached", self.get_name());
    }
    fn on_update(&mut self, window: &mut Box<dyn Window>) {
        // info!("Called update on {} layer", self.get_name());
    }
    fn on_event(&mut self, event: &mut Box<dyn Event>, window: &mut dyn Window);
}
