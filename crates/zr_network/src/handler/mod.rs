pub mod error;
pub mod options;
pub mod client;

use std::{future::Future, sync::{Arc, Mutex}};

use error::Result;
use tokio::task::JoinHandle;

macro_rules! catch_error {
    ($mutex: ident.$function: ident ()) => {
        if let Err(e) = $mutex.$function().await {
            eprintln!("{e:?}");
            return;
        }
    };
}

pub trait Handler {
    fn is_alive(&mut self) -> bool;
    
    fn init(&mut self) -> impl Future<Output = Result<()>>;
    fn inner(&mut self) -> impl Future<Output = Result<()>>;
    fn stop(&mut self) -> impl Future<Output = Result<()>>;

    fn run(self_mutex: Arc<Mutex<Self>>) -> JoinHandle<()>
    where
        Self: Send + Sync + 'static,
    {
        let self_arc = self_mutex.clone();

        tokio::task::spawn_local(async move {
            let mut self_mutex = self_arc.lock().unwrap();
            catch_error!(self_mutex.init());

            while self_mutex.is_alive() {
                catch_error!(self_mutex.inner());
            }

            catch_error!(self_mutex.stop());
        })
    }
}