use hexgem_engine::{
    info, Application, HexgemApp,
    HexgemEvent::{Event, EventDispatcher, EventType},
    HexgemLogger, ImGuiLayer, Layer, Window,
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
        if let Some(gl) = glfw {
            let imgui_layer = ImGuiLayer::new(|s| gl.get_proc_address_raw(s));
            app.push_overlay(imgui_layer);
        }
        // app.window.take().map(|mut w| {
        //     let imgui_layer = ImGuiLayer::new(|s| w.get_window().get_proc_address(s));
        //     app.push_overlay(imgui_layer);
        //     app.window = Some(w);
        // });
        app.push_layer(layer);
        // app.push_overlay(layer) Push ImGuiLayer TODO
    });
}
