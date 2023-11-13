use std::{any::Any, collections::HashMap};

use log::error;
use winit::event;

use super::HexgemEvent;

type HandlerFn = Box<dyn Fn(&Box<&dyn Any>)>;
pub struct EventEmitter {
    handlers: HashMap<EventType, HashMap<i32, HandlerFn>>,
    keys: HashMap<EventType, Vec<i32>>,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum EventType {
    None,
    WindowClose,
    WindowResize,
    WindowFocus,
    WindowLostFocus,
    WindowMoved,
    AppTick,
    AppUpdate,
    AppRender,
    KeyPressed,
    KeyReleased,
    MouseButtonPressed,
    MouseButtonReleased,
    MouseMoved,
    MouseScrolled,
}

impl EventType {
    pub fn get(event: &HexgemEvent) -> Self {
        return match event {
            HexgemEvent::None => EventType::None,
            HexgemEvent::WindowClose { .. } => EventType::WindowClose,
            HexgemEvent::WindowResize { .. } => EventType::WindowResize,
            HexgemEvent::WindowFocus { .. } => EventType::WindowFocus,
            HexgemEvent::WindowLostFocus { .. } => EventType::WindowLostFocus,
            HexgemEvent::WindowMoved { .. } => EventType::WindowMoved,
            HexgemEvent::AppTick { .. } => EventType::AppTick,
            HexgemEvent::AppUpdate { .. } => EventType::AppUpdate,
            HexgemEvent::AppRender { .. } => EventType::AppRender,
            HexgemEvent::KeyPressed { .. } => EventType::KeyPressed,
            HexgemEvent::KeyReleased { .. } => EventType::KeyReleased,
            HexgemEvent::MouseButtonPressed { .. } => EventType::MouseButtonPressed,
            HexgemEvent::MouseButtonReleased { .. } => EventType::MouseButtonReleased,
            HexgemEvent::MouseMoved { .. } => EventType::MouseMoved,
            HexgemEvent::MouseScrolled { .. } => EventType::MouseScrolled,
        };
    }
}
pub struct EventSubscription {
    event_type: EventType,
    index: i32,
}

impl EventEmitter {
    pub fn new() -> Self {
        return Self {
            handlers: HashMap::new(),
            keys: HashMap::new(),
        };
    }

    fn on(&mut self, event: EventType, handler: Box<dyn Fn(&Box<&dyn Any>)>) -> EventSubscription {
        let new_key = self
            .keys
            .entry(event.clone())
            .or_default()
            .iter()
            .max()
            .unwrap_or(&-1)
            + &1;
        self.handlers
            .entry(event.clone())
            .or_insert_with(|| HashMap::new());
        self.handlers.entry(event.clone()).and_modify(|prev| {
            prev.insert(new_key.clone(), handler);
        });
        match self.handlers.get(&event) {
            Some(some) => {
                if some.get(&new_key).is_some() {
                    self.keys
                        .entry(event.clone())
                        .and_modify(|prev| prev.push(new_key.clone()))
                        .or_insert(vec![new_key.clone()]);
                } else {
                    error!("Could not add handler to emmitter");
                }
            }
            None => error!("None handlers of {:?} has been found", event.clone()),
        }
        return EventSubscription {
            event_type: event,
            index: new_key,
        };
    }

    pub fn remove(&mut self, subscription: EventSubscription) {
        let event_handler = self
            .handlers
            .entry(subscription.event_type)
            .or_insert_with(|| HashMap::new());
        event_handler.remove_entry(&subscription.index);
    }

    pub fn emit(&self, event: &HexgemEvent) {
        let event_type = EventType::get(event);
        let event_content = event.get_event();
        if let Some(event_handlers) = self.handlers.get(&event_type) {
            for (_, handler) in event_handlers {
                {
                    handler(&Box::new(event_content));
                }
            }
        };
    }

    pub fn listen_on<T: 'static, F>(
        &mut self,
        event_type: EventType,
        handler: F,
    ) -> EventSubscription
    where
        F: Fn(&T) -> () + 'static,
    {
        self.on(
            event_type.clone(),
            Box::new(move |event| match event.downcast_ref::<T>() {
                Some(event) => handler(event),
                None => panic!("Cannot downcast Box<&dyn Any> to desired type!"),
            }),
        )
    }
}
