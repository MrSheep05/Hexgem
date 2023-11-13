use std::any::Any;

use hexgem_engine::{
    info, Application, HexgemEvent,
    HexgemEvents::{MouseButtonEvent, MouseScrollEvent},
};
pub struct Sandbox {
    pub application: Application,
}

impl hexgem_engine::App for Sandbox {
    fn create_application() -> Self {
        let mut application = Application::new();
        application.event_emitter.listen_on(
            hexgem_engine::EventType::MouseButtonPressed,
            move |event: &MouseButtonEvent| info!("{:?}", event),
        );
        //This will panic!
        // application.event_emitter.listen_on(
        //     hexgem_engine::EventType::MouseButtonPressed,
        //     move |event: &MouseScrollEvent| info!("{:?}", event),
        // );
        return Sandbox {
            application: application,
        };
    }
}

impl hexgem_engine::EventHandler for Sandbox {
    fn handle_event(&self, event: &HexgemEvent) {
        match event {
            HexgemEvent::MouseButtonReleased { .. } => {
                info!("Another use of event");
            }
            _ => (),
        }
    }
}

impl hexgem_engine::HexgemApp for Sandbox {}
