use hexgem_engine::{
    info, Application, HexgemApp,
    HexgemEvent::{Event, EventDispatcher, EventType},
    HexgemLogger, Layer,
};

struct Sandbox {}
impl HexgemApp for Sandbox {
    fn application() -> Application {
        Application::create()
    }
}

struct ExampleLayer {
    pub name: &'static str,
}

impl Layer for ExampleLayer {
    fn get_name(&self) -> &'static str {
        self.name
    }

    fn on_event(&self, event: &mut Box<dyn Event>) {
        if event.get_event_type() != EventType::None {}
    }
}
fn main() {
    HexgemLogger::init().expect("Error occured on init logger");
    let sandbox = Sandbox {};
    sandbox.run(|app| {
        let layer = ExampleLayer { name: "TEST" };

        app.push_layer(layer);
    });
}
