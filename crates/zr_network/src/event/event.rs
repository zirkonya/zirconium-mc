// TODO : Handler (who)
//      : When
//      : Cancel?
#[derive(Debug)]
pub enum Event {
    ClientConnected,
    ClientLogin,
    ClientJoin,
    ClientDisconnected,
    ClientLogout,
    ClientLeave,
}
