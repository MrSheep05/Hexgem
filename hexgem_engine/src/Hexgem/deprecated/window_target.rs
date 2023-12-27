use winit::event_loop::EventLoopWindowTarget;

pub struct WindowTarget<'a> {
    elwt: &'a EventLoopWindowTarget<()>,
    pub handled: bool,
}

impl<'a> WindowTarget<'a> {
    pub fn create(elwt: &'a EventLoopWindowTarget<()>) -> Self {
        return Self {
            elwt,
            handled: false,
        };
    }
    pub fn close(&self) {
        self.elwt.exit();
    }
    pub fn set_event_handled(&mut self) {
        self.handled = true;
    }
}
