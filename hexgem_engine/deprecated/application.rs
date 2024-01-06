use log::*;

use crate::Hexgem::hexgem_events::EventLayer;

use super::hexgem_events::{EventEmitter, EventHandler, HexgemEvent};

pub trait App: Sized {
    fn create_application() -> Self;
}

pub trait HexgemApp: App + EventHandler {
    fn run(&self, env: &Application) {
        env.run(self);
    }
}
pub struct Application {
    pub event_emitter: EventEmitter,
}

impl Application {
    pub fn new() -> Self {
        info!("Application created");
        let event_emitter = EventEmitter::new();
        return Application { event_emitter };
    }

    pub fn run(&self, app: &impl HexgemApp) {
        info!("Executed app run");
        EventLayer::init(app, &self.event_emitter); //THAT LINE IS A LOOP
    }
}
