pub trait Generator {
    type Output;
    fn generate(&self) -> Self::Output;
}