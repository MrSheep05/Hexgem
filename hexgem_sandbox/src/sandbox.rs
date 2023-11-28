use hexgem_engine::{
    error, info, Application, HexgemEvent,
    HexgemEvents::{MouseButtonEvent, MouseScrollEvent, WindowCloseEvent},
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
            move |_, wt| {
                info!("Closing window");
                wt.close();
            },
        );
        application.event_emitter.listen_on::<MouseScrollEvent, _>(
            hexgem_engine::EventType::MouseScrolled,
            |event, wi| {
                error!("ERROR SCROLL");
                wi.close();
            },
        );
        error!("ERROR");

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
            HexgemEvent::WindowClose { .. } => {
                info!("Another use of event");
            }
            _ => (),
        }
    }
}

impl hexgem_engine::HexgemApp for Sandbox {}
