use crate::{client::client::Client, error::network::NetworkError, packet::packet::Packet};
use std::{
    collections::{HashMap, HashSet},
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex, TryLockError,
    },
};

pub struct ClientManager {
    max_pid: u16,
    max_cid: u16,
    clients_partition: HashMap<u16, Arc<Mutex<HashMap<u16, Client>>>>,
    free_id: HashSet<(u16, u16)>,
    packet_sender: Sender<(u32, Packet)>,
}

impl ClientManager {
    pub fn new(
        max_client_per_partition: u16,
        max_partition: u16,
    ) -> (Self, Receiver<(u32, Packet)>) {
        let (packet_sender, packet_receiver) = mpsc::channel();
        (
            Self {
                max_pid: max_partition,
                max_cid: max_client_per_partition,
                clients_partition: HashMap::new(),
                free_id: HashSet::new(),
                packet_sender,
            },
            packet_receiver,
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
        manager: Arc<Mutex<Self>>,
        pid: u16,
        clients: Arc<Mutex<HashMap<u16, Client>>>,
        packet_sender: Sender<(u32, Packet)>,
    ) {
        let mut to_remove = Vec::new();
        loop {
            match clients.try_lock() {
                Ok(mut clients) => {
                    if clients.is_empty() {
                        break;
                    }
                    for (&cid, client) in clients.iter_mut() {
                        let id = Self::merge_id(pid, cid);
                        match client.read_packet() {
                            Ok(packet) => {
                                packet_sender
                                    .send((id, packet))
                                    .expect("Error while sending event");
                            }
                            Err(err) => {
                                if let NetworkError::IOError(err) = &err {
                                    if let std::io::ErrorKind::WouldBlock = err.kind() {
                                        continue;
                                    }
                                }
                                to_remove.push(id);
                                eprintln!("[{pid:04x}{cid:04x}] {err:?}");
                            }
                        }
                    }
                }
                Err(err) => {
                    if let TryLockError::WouldBlock = err {
                        continue;
                    } else {
                        break;
                    }
                }
            }
            if !to_remove.is_empty() {
                let mut manager = manager.lock().unwrap();
                for id in &to_remove {
                    manager.remove_client(*id);
                }
            }

            std::thread::yield_now();
        }
    }

    /// WARNING: Can't accept client for asking status if server is full !!!!
    pub fn add_client(manager: Arc<Mutex<Self>>, client: Client) -> Option<u32> {
        let manager_clone = manager.clone();
        if let Ok(mut manager) = manager.lock() {
            let (pid, cid) = manager.next_id()?;
            let id = Self::merge_id(pid, cid);
            match manager.clients_partition.get_mut(&pid) {
                Some(partition) => {
                    partition.lock().unwrap().insert(cid, client);
                }
                None => {
                    let mut partition = HashMap::new();
                    partition.insert(cid, client.try_clone().unwrap());
                    let partition = Arc::new(Mutex::new(partition));
                    manager.clients_partition.insert(pid, partition.clone());
                    let partition = partition.clone();
                    let packet_sender = manager.packet_sender.clone();
                    std::thread::spawn(move || {
                        Self::handle_partition(manager_clone, pid, partition, packet_sender)
                    });
                }
            }
            Some(id)
        } else {
            None
        }
    }

    /// remove a partition
    fn remove_partition(&mut self, pid: u16) -> Option<Arc<Mutex<HashMap<u16, Client>>>> {
        self.clients_partition.remove(&pid)
    }

    /// remove a client from partition
    fn remove_client_inner(&mut self, pid: u16, cid: u16) -> Client {
        self.clients_partition
            .get_mut(&pid)
            .unwrap()
            .lock()
            .unwrap()
            .remove(&cid)
            .unwrap()
    }

    /// - Remove client from partition
    /// - Shutdown client
    /// - Remove partition if it's empty
    /// - add id of removed client into free_id
    pub fn remove_client(&mut self, client_id: u32) {
        let (pid, cid) = Self::split_id(client_id);
        let mut client = self.remove_client_inner(pid, cid);
        client.shutdown().expect("Error while shutdown client"); // TODO : handle error
        if self.clients_partition[&pid].lock().unwrap().is_empty() {
            self.remove_partition(pid);
        }
        self.free_id.insert((pid, cid));
    }
}
