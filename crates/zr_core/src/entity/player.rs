use uuid::Uuid;

#[derive(Clone)]
pub struct Player {
    uuid: Uuid,
    name: String,
}

impl Player {
    pub fn new(uuid: Uuid, name: String) -> Self {
        Self { uuid, name }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}
