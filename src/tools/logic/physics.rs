use std::collections::HashMap;

use crate::{minecraft::blocks::Block, tools::maths::vector::position::Position};

pub trait RedstoneSource {
    
}

pub trait RedstoneActivable {
    fn on_activate(&self);
    fn on_desactivate(&self);
}

pub trait Physics {
    fn apply_physics(&self, blocks: &mut HashMap<Position, Box<dyn Block>>);
}