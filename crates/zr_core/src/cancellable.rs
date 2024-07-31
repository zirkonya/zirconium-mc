pub trait Cancellable {
    fn is_cancel(&self) -> bool;
    fn cancel(&mut self, cancel: bool);
}
