#![doc = include_str!("../README.md")]
use std::{
    net::TcpListener,
    sync::{Arc, Mutex},
};

use zr_core::handler::Handler;
use zr_network::client::{
    client::{Client, COMPRESSION_ACTIVE},
    manager::ClientManager,
};
use zr_protocol::handler::protocol_handler::ProtocolHandler;

const SERVER_ADDRESS: &str = "127.0.0.1:25565";

pub fn main() {
    let listener = TcpListener::bind(SERVER_ADDRESS).expect("Cannot listen");
    let (client_manager, receiver) = ClientManager::new(20, 1);
    let client_manager = Arc::new(Mutex::new(client_manager));
    let protocol_handler = Arc::new(Mutex::new(ProtocolHandler::new(receiver)));
    Handler::handle(protocol_handler.clone());
    println!("Listen on {SERVER_ADDRESS}");
    loop {
        for stream in listener.incoming() {
            if let Ok(stream) = stream {
                let mut protocol_handler = protocol_handler.lock().unwrap();
                let client = Client::new_with_opt(stream, COMPRESSION_ACTIVE).unwrap();
                if let Some(id) =
                    ClientManager::add_client(client_manager.clone(), client.try_clone().unwrap())
                {
                    println!("client accepted [{id:08x}]");
                    protocol_handler.add_client(id, client);
                } else {
                    eprint!("server is full");
                }
            }
        }
    }
}
