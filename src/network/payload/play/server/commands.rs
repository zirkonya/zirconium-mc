use std::sync::Arc;

use zr_protocol::{
    macros::Serialize,
    message::{option::PrefixOption, size::PrefixSize},
};

use crate::utils::varint::VarInt;

#[derive(Serialize, Debug)]
// #[discriminant_type(VarInt<i32>)]
pub struct Node {
    pub flags: u8,                                             //
    pub children: PrefixSize<VarInt<i32>, Arc<[VarInt<i32>]>>, //
    pub redirect_node: PrefixOption<VarInt<i32>>,            //
    pub name: PrefixOption<Arc<[u8]>>,                       // TODO : String
    pub parser_id: PrefixOption<VarInt<i32>>,                //
    pub properties: PrefixOption<()>,                        // TODO : Properties
    pub suggestions_type: PrefixOption<VarInt<i32>>,         //
}

#[derive(Serialize, Debug)]
pub struct Commands {
    pub nodes: PrefixSize<VarInt<i32>, Arc<[Node]>>,
    pub root_index: VarInt<i32>,
}
