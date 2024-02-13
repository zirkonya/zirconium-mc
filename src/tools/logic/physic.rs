use std::collections::HashMap;

use crate::{minecraft::blocks::Block, tools::maths::vector::position::Position};

pub trait RestoneSource {
    
}

pub trait RedstoneActivable {
    fn on_activate(&self);
    fn on_desactivate(&self);
}

pub trait Physic {
    fn apply_physic(&self, blocks: &mut HashMap<Position, Box<dyn Block>>);
}