use std::sync::Arc;

use zr_protocol::{macros::Serialize, message::size::PrefixSize};

use crate::utils::{identifier::Identifier, varint::VarInt};

#[derive(Serialize, Debug)]
pub struct UpdateRecipes {
    pub property_sets:
        PrefixSize<VarInt<i32>, Arc<[(Identifier, PrefixSize<VarInt<i32>, Arc<[VarInt<i32>]>>)]>>,
    pub stonecutter_recipes: PrefixSize<VarInt<i32>, Arc<[()]>>, // TODO : ID set
                                                                 // TODO : Slot Display
}
