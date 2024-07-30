pub(crate) mod handler;
pub(crate) mod manager;
// N clients in [sender, receiver]
// sender pull from response pool
// receiver push into event pool
// EventManager [event_pool, listeners]
// Listener macro #[on(Event)]
// ClientManager -> Manage pool of client in differents thread
