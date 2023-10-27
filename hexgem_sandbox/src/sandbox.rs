use hexgem_engine::{Application, HexgemEvent};
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
    fn handleEvent(&self, event: HexgemEvent) {
        match event {
            _ => (),
        }
    }
}
impl hexgem_engine::HexgemApp for Sandbox {}
