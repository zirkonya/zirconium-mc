use std::sync::Arc;

use zr_protocol::{
    macros::Serialize,
    message::{option::PrefixOption, size::PrefixSize},
};

use crate::utils::{identifier::Identifier, varint::VarInt};

#[derive(zr_protocol::macros::Serialize, Debug)]
pub struct SetEntityMetadata {
    pub entity_id: VarInt<i32>, // TODO : Entity API
    pub metadata: Metadata,     // TODO : Prefixed enum
}

#[derive(Serialize, Debug)]
#[discriminant_type(VarInt<i32>)]
pub enum Metadata {
    Byte(u8),
    VarInt(VarInt<i32>),
    VarLong(VarInt<i64>),
    Float(f32),
    String(PrefixSize<VarInt<i32>, Arc<[u8]>>),
    TextComponent(PrefixSize<VarInt<i32>, Arc<[u8]>>), // TODO : Text Component
    OptionalTextComponent(PrefixOption<PrefixSize<VarInt<i32>, Arc<[u8]>>>), // TODO : Text Component
    Slot(()),                                                                  // TODO : Slot
    Bool(bool),
    Rotations(f32, f32, f32),
    Position(u64),                                       // TODO : Position
    OptionalPosition(PrefixOption<u64>),               // TODO : Position
    Direction(VarInt<i32>),                              // TODO : VarInt Enum
    OptionalLivingEntityReference(PrefixOption<u128>), // TODO : UUID
    BlockState(VarInt<i32>),                             // TODO : BlockState
    OptinalBlockState(PrefixOption<VarInt<i32>>),      // TODO : BlockState
    Paticle(()),                                         // TODO : Particle
    Particles(()),                                       // TODO : Particles
    VillagerData(VarInt<i32>, VarInt<i32>, VarInt<i32>),
    OptionalVarInt(PrefixOption<VarInt<i32>>), // WHY ?
    Pose(VarInt<i32>),                           // TODO : VarInt enum
    CatVariant(VarInt<i32>),
    CowVariant(VarInt<i32>),
    WolfVariant(VarInt<i32>),
    WolfSoundVariant(VarInt<i32>),
    FrogVariant(VarInt<i32>),
    PigVariant(VarInt<i32>),
    ChickenVariant(VarInt<i32>),
    ZombieNautilusVariant(VarInt<i32>),
    OptionalGlobalPosition(PrefixOption<Identifier>, PrefixOption<u64>), // TODO : Position
    PaintingVariant(VarInt<i32>),
    SnifferState(VarInt<i32>),               // TODO : VarInt enum
    ArmadilloState(VarInt<i32>),             // TODO : VarInt enum
    CopperGolemState(VarInt<i32>),           // TODO : VarInt enum
    WeatheringCopperGolemState(VarInt<i32>), // TODO : VarInt enum
    Vector3(f32, f32, f32),
    Quaternion(f32, f32, f32, f32),
    ResolvableProfile(()),    // TODO : ResolvableProfil
    HumanoidArm(VarInt<i32>), // TODO : VarInt enum
}
