use std::sync::{Arc, Mutex};

pub trait Handler {
    type Return;
    fn handle(handler: Arc<Mutex<Self>>) -> std::thread::JoinHandle<Self::Return>
    where
        Self: Sized;
}
