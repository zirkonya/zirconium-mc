use crate::{client::Client, event::event::Event};
use std::{
    collections::{HashMap, HashSet},
    net::TcpStream,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
};

pub struct ClientManager {
    max_pid: u16,
    max_cid: u16,
    clients_partition: HashMap<u16, Arc<Mutex<HashMap<u16, Client>>>>,
    free_id: HashSet<(u16, u16)>,
    event_sender: Sender<Event>,
}

impl ClientManager {
    pub fn new(max_client_per_partition: u16, max_partition: u16) -> (Self, Receiver<Event>) {
        let (event_sender, event_receiver) = mpsc::channel();
        (
            Self {
                max_pid: max_partition,
                max_cid: max_client_per_partition,
                clients_partition: HashMap::new(),
                free_id: HashSet::new(),
                event_sender,
            },
            event_receiver,
        )
    }

    fn next_id(&self) -> Option<(u16, u16)> {
        for cid in 0..self.max_cid {
            for pid in 0..self.max_pid {
                if !self.clients_partition.contains_key(&pid)
                    || !self.clients_partition[&pid]
                        .lock()
                        .unwrap()
                        .contains_key(&cid)
                {
                    return Some((pid, cid));
                }
            }
        }
        None
    }

    fn merge_id(pid: u16, cid: u16) -> u32 {
        ((pid as u32) << 16) | cid as u32
    }

    fn split_id(id: u32) -> (u16, u16) {
        ((id >> 16) as u16, id as u16)
    }

    fn handle_partition(
        pid: u16,
        clients: Arc<Mutex<HashMap<u16, Client>>>,
        event_sender: Sender<Event>,
    ) {
        loop {
            if let Ok(mut clients) = clients.lock() {
                if clients.is_empty() {
                    break;
                }
                for (&cid, client) in clients.iter_mut() {
                    match client.read_packet() {
                        Ok(packet) => {
                            let event = Event::ClientSendPacket {
                                client_id: Self::merge_id(pid, cid),
                                packet,
                            };
                            event_sender.send(event).expect("Error while sending event");
                        }
                        Err(err) => eprintln!("[{pid:04x}_{cid:04x}] {err:?}"),
                    }
                }
            } else {
                break;
            }
        }
    }

    pub fn add_client(&mut self, client_stream: TcpStream) -> Option<u32> {
        let (pid, cid) = self.next_id()?;
        let id = Self::merge_id(pid, cid);
        let client = Client::new(id, client_stream).expect("cannot create client"); // ??
        match self.clients_partition.get_mut(&pid) {
            Some(partition) => {
                partition.lock().unwrap().insert(cid, client);
            }
            None => {
                let mut partition = HashMap::new();
                partition.insert(cid, client);
                let partition = Arc::new(Mutex::new(partition));
                self.clients_partition.insert(pid, partition.clone());
                let partition = partition.clone();
                let event_sender = self.event_sender.clone();
                std::thread::spawn(move || Self::handle_partition(pid, partition, event_sender));
            }
        }
        Some(id)
    }

    pub fn remove_client(&mut self, client_id: u32) {
        let (pid, cid) = Self::split_id(client_id);
        let mut client = self
            .clients_partition
            .get_mut(&pid)
            .unwrap()
            .lock()
            .unwrap()
            .remove(&cid)
            .unwrap();
        client.shutdown().expect("Error while shutdown client"); // TODO : handle error
        self.free_id.insert((pid, cid));
    }
}
