use crate::format::uuid::Uuid;

pub trait Entity {
    fn uuid(&self) -> Uuid;
}