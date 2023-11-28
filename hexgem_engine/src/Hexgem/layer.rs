use log::info;

use crate::HexgemEvent;

pub trait Layer {
    fn get_name(&self) -> &'static str;
    fn on_attach(&self) {
        info!("{} layer has been attached", self.get_name());
    }
    fn on_detach(&self) {
        info!("{} layer has been detached", self.get_name());
    }
    fn on_update(&self) {
        info!("Called update on {} layer", self.get_name());
    }
    fn on_event(&self, event: &HexgemEvent) {}
}
