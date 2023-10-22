use hexgem_engine::App;
use sandbox::Sandbox;

mod sandbox;
fn main() {
    let sandbox = Sandbox::create_application();
    sandbox.run();
}
