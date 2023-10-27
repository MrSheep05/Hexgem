use hexgem_engine::{debug, error, info, warn, App, HexgemApp, HexgemLogger};
use sandbox::Sandbox;

mod sandbox;
fn main() {
    HexgemLogger::init().expect("Error occured on init logger");
    let sandbox = Sandbox::create_application();
    sandbox.run(&sandbox.application);
}
