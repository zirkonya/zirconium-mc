use super::{event::Event, listener::Listener};

pub struct EventManager {
    pub(crate) listeners: Vec<Box<dyn Listener>>,
    event_pool: Vec<Event>,
}

impl EventManager {
    pub fn new() -> Self {
        Self {
            listeners: Vec::new(),
            event_pool: Vec::new(),
        }
    }

    pub fn add_listener(&mut self, listener: Box<dyn Listener>) -> &mut Self {
        self.listeners.push(listener);
        self
    }

    // better name ?
    pub fn add_event(&mut self, event: Event) {
        self.event_pool.push(event);
    }

    fn call(&mut self, event: Event) {
        for listener in &mut self.listeners {
            if let Err(err) = listener.as_mut().handle_event(&event) {
                eprintln!("Error while listening : {event:?} {err:?}");
            }
        }
    }
}

// TODO : ThreadSystem
