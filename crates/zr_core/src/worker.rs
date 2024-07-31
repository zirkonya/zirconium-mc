use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
    thread::JoinHandle,
};
macro_rules! work {
    (init $worker: expr) => {
        match $worker.lock() {
            Ok(mut worker) => worker.init(),
            Err(err) => {
                eprintln!("{err:?}");
                panic!("worker crash during init, follow error above")
            }
        }
    };
    (handle $worker: expr) => {
        match $worker.lock() {
            Ok(mut worker) => worker.handle(),
            Err(err) => {
                eprintln!("{err:?}");
                panic!("worker crash during handle, follow error above")
            }
        }
    };
    (end $worker: expr) => {
        match $worker.lock() {
            Ok(mut worker) => worker.end(),
            Err(err) => {
                eprintln!("{err:?}");
                panic!("worker crash during end, follow error above")
            }
        }
    };
    (is_alive $worker: expr) => {
        match $worker.lock() {
            Ok(mut worker) => worker.is_alive(),
            Err(err) => {
                eprintln!("{err:?}");
                false
            }
        }
    };
}

pub trait Worker {
    type Err;

    fn is_alive(&mut self) -> bool;

    fn init(&mut self);
    fn handle(&mut self);
    fn end(&mut self);

    fn run(worker: Arc<Mutex<Self>>) -> JoinHandle<()>
    where
        Self::Err: Debug + Send,
        Self: Sync + Send + 'static,
    {
        std::thread::spawn(move || {
            work!(init worker);
            while work!(is_alive worker) {
                work!(handle worker);
            }
            work!(end worker);
        })
    }
}
