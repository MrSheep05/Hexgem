use std::any::Any;

use hexgem_engine::{
    info, Application, HexgemEvent,
    HexgemEvents::{MouseButtonEvent, MouseScrollEvent, WindowCloseEvent},
    HexgemWindow,
};
pub struct Sandbox {
    pub application: Application,
}

impl hexgem_engine::App for Sandbox {
    fn create_application() -> Self {
        let mut application = Application::new();
        application.event_emitter.listen_on::<MouseButtonEvent, _>(
            hexgem_engine::EventType::MouseButtonPressed,
            move |event, _| info!("{:?}", event),
        );
        application.event_emitter.listen_on::<WindowCloseEvent, _>(
            hexgem_engine::EventType::WindowClose,
            move |_, elwt| {
                info!("Closing window");
                elwt.exit();
            },
        );
        // // This will panic!
        // application.event_emitter.listen_on::<MouseScrollEvent, _>(
        //     hexgem_engine::EventType::MouseButtonPressed,
        //     move |event, _| info!("{:?}", event),
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
