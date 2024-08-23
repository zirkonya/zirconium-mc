use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use crate::packet::packet::Packet;

// TODO : Handler (who)
//      : When
//      : Cancel?
#[derive(Debug)]
pub enum Event {
    ClientJoin { client_id: u32 },
    ClientQuit { client_id: u32, reason: String },
    ClientSendPacket { client_id: u32, packet: Packet },
}

pub type EventPool = Arc<Mutex<VecDeque<Event>>>;
