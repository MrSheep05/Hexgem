use hexgem_engine::{info, Application, HexgemEvent};
pub struct Sandbox {
    pub application: Application,
}

impl hexgem_engine::App for Sandbox {
    fn create_application() -> Self {
        let mut application = Application::new();

        application
            .event_emitter
            .on(hexgem_engine::EventType::MouseButtonPressed, &|event| {
                match event.downcast_ref::<hexgem_engine::HexgemEvents::MouseButtonEvent>() {
                    Some(mouse_event) => info!("{:?}", mouse_event.button),
                    None => panic!("Some error does not match"),
                }
            });

        {
            application
                .event_emitter
                .on(hexgem_engine::EventType::MouseButtonReleased, &|_| {
                    info!("BOOM");
                });
        }
        return Sandbox {
            application: application,
        };
    }
}

impl hexgem_engine::EventHandler for Sandbox {
    fn handle_event(&self, event: &HexgemEvent) {
        match event {
            HexgemEvent::MouseButtonReleased { .. } => {
                info!("BOOM");
            }
            _ => (),
        }
    }
}

impl hexgem_engine::HexgemApp for Sandbox {}
