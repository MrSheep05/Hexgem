use hexgem_engine::{info, Application, HexgemEvent};
pub struct Sandbox {
    pub application: Application,
}

impl hexgem_engine::App for Sandbox {
    fn create_application() -> Self {
        let application = Application::new();
        return Sandbox { application };
    }
}

impl hexgem_engine::EventHandler for Sandbox {
    fn handle_event(&self, event: HexgemEvent) {
        match event {
            HexgemEvent::MouseButtonPressed { .. } => {
                info!("Category of mousePressed {}", event.get_category())
            }
            _ => (),
        }
    }
}
impl hexgem_engine::HexgemApp for Sandbox {}
