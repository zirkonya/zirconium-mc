use crate::{format::nbt::NBTTag, gen_bin, gen_struct, tools::maths::varint::varint::VarInt};

gen_bin!(Slot { present: bool, item_id: Option<VarInt<i32>>, item_count: Option<i8>, nbt: Option<NBTTag> });