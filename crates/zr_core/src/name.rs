pub trait Nameable {
    fn set_custom_name(&mut self, name: &str);
    fn custom_name(&self) -> &String;
}
