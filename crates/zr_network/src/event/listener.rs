use super::event::Event;
use std::error::Error;

/// ```
/// pub struct Example;
///
/// impl Listener for Example {
///     #[on("Event::ClientConnected")]
///     fn on_connection(&mut self, event: &Event) -> Result<(), Box<dyn Error>> {
///         some_code_here(event);
///     }
///
///     #[on("Event::ClientDisconnected")]
///     fn on_disconnection(&mut self, event: &Event) -> Result<() Box<dyn Error>> {
///         some_other_code_here(event);
///     }
/// }
/// ```
pub trait Listener {
    fn handle_event(&mut self, event: &Event) -> Result<(), Box<dyn Error>>;
}
