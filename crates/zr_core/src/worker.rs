use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
    thread::JoinHandle,
};

// macro_rules! try_work {
//     (init $worker: expr) => {
//         match $worker.try_lock() {
//             Ok(mut worker) => worker.init(),
//             Err(err) if err == Err(err) => {
//                 eprintln!("{err:?}");
//                 panic!("worker crash during init, follow error above")
//             }
//         }
//     };
//     (handle $worker: expr) => {
//         match $worker.lock() {
//             Ok(mut worker) => worker.handle(),
//             Err(err) => {
//                 eprintln!("{err:?}");
//                 panic!("worker crash during handle, follow error above")
//             }
//         }
//     };
//     (end $worker: expr) => {
//         match $worker.lock() {
//             Ok(mut worker) => worker.end(),
//             Err(err) => {
//                 eprintln!("{err:?}");
//                 panic!("worker crash during end, follow error above")
//             }
//         }
//     };
//     (is_alive $worker: expr) => {
//         match $worker.lock() {
//             Ok(worker) => worker.is_alive(),
//             Err(err) => {
//                 eprintln!("{err:?}");
//                 false
//             }
//         }
//     };
// }

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
            Ok(worker) => worker.is_alive(),
            Err(err) => {
                eprintln!("{err:?}");
                false
            }
        }
    };
}

pub trait Worker {
    // TODO : use critical error (error when worker crash)
    //        if this error occured, all app stop too
    type Err;

    fn is_alive(&self) -> bool;

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
