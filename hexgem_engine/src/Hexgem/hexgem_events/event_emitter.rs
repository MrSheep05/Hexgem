use std::collections::HashMap;

use log::error;

use crate::HexgemEvent;

type HandlerFn = Box<&'static dyn Fn(&HexgemEvent)>;
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
            HexgemEvent::WindowClose => EventType::WindowClose,
            HexgemEvent::WindowResize { .. } => EventType::WindowResize,
            HexgemEvent::WindowFocus => EventType::WindowFocus,
            HexgemEvent::WindowLostFocus => EventType::WindowLostFocus,
            HexgemEvent::WindowMoved { .. } => EventType::WindowMoved,
            HexgemEvent::AppTick => EventType::AppTick,
            HexgemEvent::AppUpdate => EventType::AppUpdate,
            HexgemEvent::AppRender => EventType::AppRender,
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

    pub fn on(
        &mut self,
        event: EventType,
        handler: &'static dyn Fn(&HexgemEvent),
    ) -> EventSubscription {
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
            .and_modify(|prev| {
                prev.insert(new_key.clone(), Box::new(handler));
            })
            .or_insert(HashMap::from([(new_key.clone(), Box::new(handler))]));
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
        return {
            EventSubscription {
                event_type: event,
                index: new_key,
            }
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
        if let Some(event_handlers) = self.handlers.get(&event_type) {
            for (_, handler) in event_handlers {
                handler(event);
            }
        };
    }
}
