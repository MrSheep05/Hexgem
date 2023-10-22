use log::*;

pub trait App: Sized {
    fn run(&self, env: &Application) {
        env.run(self);
    }
    fn create_application() -> Self;
}

pub struct Application {}

impl Application {
    pub fn new() -> Self {
        info!("Application created");
        return Application {};
    }

    pub fn run<T: App>(&self, app: &T) {
        info!("Executed app run");
        while true {}
    }
}
