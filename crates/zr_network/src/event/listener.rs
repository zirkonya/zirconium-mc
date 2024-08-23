use super::event::Event;
use std::error::Error;

/// ```
/// pub struct Example;
///
/// #[listener]
/// impl Listener for Example {
///     #[on("Event::ClientJoin")]
///     fn on_join(&mut self, event: &Event) -> Result<(), Box<dyn Error>> {
///         some_code_here(event);
///     }
///
///     #[on("Event::ClientQuit")]
///     fn on_quit(&mut self, event: &Event) -> Result<() Box<dyn Error>> {
///         some_other_code_here(event);
///     }
/// }
/// ```
pub trait Listener {
    fn handle_event(&mut self, event: &Event) -> Result<(), Box<dyn Error>>;
}
