pub trait Nameable {
    fn name(&self) -> &String;
    fn set_name(&mut self, name: &str);
}
