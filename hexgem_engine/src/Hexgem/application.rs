pub trait App: Sized {
    type Env: AppEnv;
    fn run(&self, env: &Self::Env) {
        env.run(self);
    }
    fn create_application() -> Self;
}

pub trait AppEnv {
    fn run<T: App>(&self, app: &T) {}
}
pub struct Application {}

impl AppEnv for Application {
    fn run<T: App>(&self, app: &T) {
        while true {}
    }
}

impl Application {
    pub fn new() -> Self {
        return Application {};
    }
}
