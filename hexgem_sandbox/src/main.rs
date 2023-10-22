use hexgem_engine::{App, HexgemLogger};
use sandbox::Sandbox;

mod sandbox;
fn main() {
    HexgemLogger::init();
    let sandbox = Sandbox::create_application();
    sandbox.run();
}
