use hexgem_engine::Application;
pub struct Sandbox {
    application: Application,
}

impl hexgem_engine::App for Sandbox {
    fn create_application() -> Self {
        let application = Application::new();
        return Sandbox { application };
    }
}

impl Sandbox {
    pub fn run(&self) {
        self.application.run(self);
    }
}
