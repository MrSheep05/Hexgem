use hexgem_engine::{
    info, Application, EguiLayer, HexgemApp,
    HexgemEvent::{Event, EventType},
    HexgemLogger, Layer, Window,
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

    fn on_event(&mut self, event: &mut Box<dyn Event>, window: &mut dyn Window) {
        if event.get_event_type() != EventType::None {}
    }
}
fn main() {
    HexgemLogger::init().expect("Error occured on init logger");
    let sandbox = Sandbox {};
    sandbox.run(|app| {
        let layer = ExampleLayer { name: "TEST" };
        let glfw = app.get_window().as_ref().map(|w| w.get_glfw());

        app.window.take().map(|mut w| {
            let imgui_layer = EguiLayer::create(w.get_window());
            app.push_overlay(imgui_layer);
            app.window = Some(w);
        });
        app.push_layer(layer);
    });
}
