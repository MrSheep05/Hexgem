#[derive(Debug, Clone, Copy)]

pub struct AppRenderEvent {}
#[derive(Debug, Clone, Copy)]

pub struct AppUpdateEvent {}
#[derive(Debug, Clone, Copy)]

pub struct AppTickEvent {}
#[derive(Debug)]

pub enum EventCategoryApplication {
    AppTick(AppTickEvent),
    AppUpdate(AppUpdateEvent),
    AppRender(AppRenderEvent),
}
