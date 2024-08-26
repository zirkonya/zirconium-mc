use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use crate::{client::Client, packet::packet::Packet};

#[derive(Debug)]
pub enum Event {
    ClientJoin { client_id: u32, client: Client },
    ClientQuit { client_id: u32, reason: String },
    ClientSendPacket { client_id: u32, packet: Packet },
}

pub type EventPool = Arc<Mutex<VecDeque<Event>>>;
