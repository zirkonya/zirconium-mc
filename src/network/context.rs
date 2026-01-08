use std::{collections::HashMap, net::SocketAddr};

use clap::ValueEnum;

#[derive(ValueEnum, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum State {
    #[default]
    Handshaking,
    Login,
    Status,
    Config,
    Play,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum Bound {
    Server,
    Client,
}

#[derive(Default, Debug, Clone)]
pub struct ClientContext {
    pub state: State,
    pub compression_set: bool,
    pub compression_threshold: Option<i32>,
}

#[derive(Default, Clone)]
pub struct ServerContext {
    pub clients: HashMap<SocketAddr, ClientContext>,
}
