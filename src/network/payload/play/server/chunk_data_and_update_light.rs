use crate::utils::varint::VarInt;
use std::sync::Arc;
use zr_protocol::{macros::Serialize, message::size::PrefixSize, serialization::Serialize};

#[derive(Serialize, Debug, Clone)]
pub struct BlockEntity {
    pub packed_xz: u8,
    pub y: i16,
    pub block_entity_type: VarInt<i32>,
    pub data: Arc<[u8]>, // TODO : NBT
}

#[derive(Serialize, Debug, Clone)]
pub struct HeightMap {
    pub ty: VarInt<i32>, // TODO : VarInt enum
    pub data: PrefixSize<VarInt<i32>, Arc<[u64]>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Single {
    pub value: VarInt<i32>,
}

impl Palette for Single {}

#[derive(Serialize, Debug, Clone)]
pub struct Indirect {
    pub palette: PrefixSize<VarInt<i32>, Arc<[VarInt<i32>]>>,
}

impl Palette for Indirect {}

#[derive(Serialize, Debug, Clone)]
pub struct Direct;

impl Palette for Direct {}

pub trait Palette: Serialize {}

#[derive(Serialize, Debug, Clone)]
pub struct PalettedContainer<P: Palette> {
    pub bits_per_entry: u8,
    pub palette: P,
    pub data_array: PrefixSize<VarInt<i32>, Arc<[i64]>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ChunkSection<BlockStatePalette: Palette, BiomesPalette: Palette> {
    pub block_count: i16,
    pub block_state: PalettedContainer<BlockStatePalette>,
    pub biomes: PalettedContainer<BiomesPalette>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ChunkData {
    pub heightmaps: PrefixSize<VarInt<i32>, Arc<[HeightMap]>>,
    pub data: PrefixSize<VarInt<i32>, Arc<[u8]>>,
    pub block_entities: PrefixSize<VarInt<i32>, Arc<[BlockEntity]>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct LightData {
    pub sky_light_mask: PrefixSize<VarInt<i32>, Arc<[u64]>>, // TODO : BitSet
    pub block_light_mask: PrefixSize<VarInt<i32>, Arc<[u64]>>, // TODO : BitSet
    pub empty_sky_light_mask: PrefixSize<VarInt<i32>, Arc<[u64]>>, // TODO : BitSet
    pub empty_block_light_mask: PrefixSize<VarInt<i32>, Arc<[u64]>>, // TODO : BitSet
    pub sky_light_arrays: PrefixSize<VarInt<i32>, Arc<[PrefixSize<VarInt<i32>, Arc<[u8]>>]>>, // bidouillage de sagouin
    pub block_light_arrays: PrefixSize<VarInt<i32>, Arc<[PrefixSize<VarInt<i32>, Arc<[u8]>>]>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ChunkDataAndUpdateLight {
    pub x: i32,
    pub z: i32,
    pub data: ChunkData,
    pub light: LightData,
}
