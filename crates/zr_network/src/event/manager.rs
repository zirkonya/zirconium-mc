use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use zr_core::worker::Worker;

use crate::error::worker::WorkerError;

use super::{
    event::{Event, EventPool},
    listener::Listener,
};

pub struct EventManager {
    alive: bool,
    pub(crate) listeners: Vec<Box<dyn Listener>>,
    event_pool: EventPool,
}

impl EventManager {
    pub fn new() -> Self {
        Self {
            alive: true,
            listeners: Vec::new(),
            event_pool: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn add_listener(&mut self, listener: Box<dyn Listener>) -> &mut Self {
        self.listeners.push(listener);
        self
    }

    // better name ?
    pub fn add_event(&mut self, event: Event) {
        match self.event_pool.lock() {
            Ok(mut pool) => pool.push_front(event),
            Err(mut err) => {
                **err.get_mut() = VecDeque::from(vec![event]);
                self.event_pool.clear_poison();
                eprintln!("Error occured in EventPool");
            }
        }
    }

    fn pop_event(&mut self) -> Option<Event> {
        match self.event_pool.lock() {
            Ok(mut pool) => pool.pop_back(),
            Err(mut err) => {
                **err.get_mut() = VecDeque::new();
                self.event_pool.clear_poison();
                eprintln!("Error occured in EventPool");
                None
            }
        }
    }

    fn call(&mut self, event: &Event) {
        for listener in &mut self.listeners {
            if let Err(err) = listener.as_mut().handle_event(event) {
                eprintln!("Error while listening : {event:?} {err:?}");
            }
        }
    }
}

impl Default for EventManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Worker for EventManager {
    type Err = WorkerError;
    fn is_alive(&self) -> bool {
        self.alive
    }

    fn init(&mut self) {
        if self.listeners.is_empty() {
            eprintln!("EventManager has no listeners");
            self.alive = false;
        }
    }

    fn handle(&mut self) {
        if let Some(event) = self.pop_event() {
            self.call(&event);
        }
    }

    fn end(&mut self) {
        println!("End")
    }
}
