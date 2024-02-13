use std::collections::HashMap;

use crate::{format::uuid::Uuid, minecraft::{blocks::Block, entity::Entity}, tools::{logic::{physic::Physic, generator::Generator}, maths::vector::position::Position}};

pub struct Chunk {
    x: i32,
    z: i32,
    blocks: HashMap<Position, Box<dyn Block>>,
    entities: HashMap<Uuid, Box<dyn Entity>>
}

impl Chunk {
    pub fn new<G>(generator: &G, x: i32, z: i32) -> Self
        where G: Generator<Output = HashMap<Position, Box<dyn Block>>>
    {
        Self { x, z, blocks: generator.generate(), entities: HashMap::new() }
    }

    pub fn z(&self) -> i32 {
        self.z
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn blocks(&self) -> &HashMap<Position, Box<dyn Block>> {
        &self.blocks
    }

    pub fn get_block(&self, position: &Position) -> Option<&Box<dyn Block>> {
        self.blocks.get(position)
    }

    pub fn get_block_mut(&mut self, position: &Position) -> Option<&mut Box<dyn Block>> {
        self.blocks.get_mut(position)
    }

    pub fn set_block(&mut self, position: Position, block: Box<dyn Block>) {
        self.blocks.insert(position, block);
    }

    pub fn remove_block(&mut self, position: &Position) {
        self.blocks.remove(position);
    }

    pub fn entities(&self) -> &HashMap<Uuid, Box<dyn Entity>> {
        &self.entities
    }

    pub fn add_entity(&mut self, entity: Box<dyn Entity>) {
        self.entities.insert(entity.uuid(), entity);
    }

    pub fn remove_entity(&mut self, uuid: &Uuid) {
        self.entities.remove(uuid);
    }

    pub fn get_entity(&self, uuid: &Uuid) -> Option<&Box<dyn Entity>> {
        self.entities.get(uuid)
    }

    pub fn get_entity_mut(&mut self, uuid: &Uuid) -> Option<&mut Box<dyn Entity>> {
        self.entities.get_mut(uuid)
    }

    pub fn update_blocks<P: Physic>(&mut self, physic: &P) {
        physic.apply_physic(&mut self.blocks);
    }
}

impl Drop for Chunk {
    fn drop(&mut self) {
        // TODO : Save content on disk
        todo!()
    }
}