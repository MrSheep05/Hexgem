use log::info;

use crate::HexgemEvent;

pub unsafe trait Layer {
    const NAME: &'static str = "HexgemLayer";
    fn on_attach(&self) {
        info!("{} layer has been attached", Self::NAME);
    }
    fn on_detach(&self) {
        info!("{} layer has been detached", Self::NAME)
    }
    fn on_update(&self) {
        info!("Called update on {} layer", Self::NAME)
    }
    fn on_event(&self, event: &HexgemEvent);
}
