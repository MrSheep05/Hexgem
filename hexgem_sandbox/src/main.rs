use hexgem_engine::{
    HexgemLogger,
    NewHexgem::{Application, HexgemApp},
};

struct Sandbox {}

impl HexgemApp for Sandbox {
    fn application() -> Application {
        Application::create()
    }
}
fn main() {
    HexgemLogger::init().expect("Error occured on init logger");
    let sandbox = Sandbox {};
    sandbox.run();
}
