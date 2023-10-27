use log::*;

use crate::Hexgem::hexgem_events::EventLayer;

use super::hexgem_events::{EventHandler, HexgemEvent};

pub trait App: Sized {
    fn create_application() -> Self;
}

pub trait HexgemApp: App + EventHandler {
    fn run(&self, env: &Application) {
        env.run(self);
    }
}
pub struct Application {}

impl Application {
    pub fn new() -> Self {
        info!("Application created");
        return Application {};
    }

    pub fn run(&self, app: &impl HexgemApp) {
        info!("Executed app run");
        EventLayer::init(app);
        while true {}
    }
}
