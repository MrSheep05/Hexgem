use hexgem_engine::{info, Application, HexgemEvent};
pub struct Sandbox {
    pub application: Application,
}

impl hexgem_engine::App for Sandbox {
    fn create_application() -> Self {
        let mut application = Application::new();

        application
            .event_emitter
            .on(hexgem_engine::EventType::MouseButtonPressed, &|_| {
                info!("BOMB");
            });

        {
            application.event_emitter.on(
                hexgem_engine::EventType::MouseButtonReleased,
                &move |_| {
                    info!("BOOM");
                },
            );
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
