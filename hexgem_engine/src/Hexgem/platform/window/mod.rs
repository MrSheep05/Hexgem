mod glfw_window;
mod sdl_window;
mod window_types;
pub mod HexgemWindow {

    pub use super::glfw_window::GlfwWindow;
    pub use super::sdl_window::SdlWindow;
}
