use hexgem_engine::{error, info, App, HexgemLogger};
use sandbox::Sandbox;

mod sandbox;
fn main() {
    HexgemLogger::init();
    let sandbox = Sandbox::create_application();
    sandbox.run();
}
