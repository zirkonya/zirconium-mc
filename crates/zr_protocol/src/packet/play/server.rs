use uuid::Uuid;
use zr_binary::varint::VarInt;
use zr_binary_macros::Binary;
use zr_network_macros::Packet;

#[derive(Binary, Packet)]
#[id = 0x00]
pub struct BundleDelimiter;

pub type Vector3d = f64;
pub type Vector3<T> = T;

#[derive(Binary, Packet)]
#[id = 0x01]
pub struct SpawnEntity {
    pub(crate) entity_id: VarInt<i32>,
    pub(crate) entity_uuid: Uuid,
    pub(crate) entity_type: VarInt<i32>,
    pub(crate) coordinate: Vector3d,
    pub(crate) pitch: u8,
    pub(crate) yaw: u8,
    pub(crate) head_yaw: u8,
    pub(crate) data: VarInt<i32>,
    pub(crate) velocity: Vector3<i16>,
}

#[derive(Binary, Packet)]
#[id = 0x02]
pub struct SpawnExperienceOrb {
    pub(crate) entity_id: VarInt<i32>,
    pub(crate) coordinate: Vector3d,
    pub(crate) count: i16,
}

#[derive(Binary, Packet)]
#[id = 0x03]
pub struct EntityAnimation {
    pub(crate) entity_id: VarInt<i32>,
    pub(crate) animation: u8,
}

#[derive(Debug, Binary, Clone)]
pub struct Statistic {
    pub(crate) category_id: VarInt<i32>,
    pub(crate) statistic_id: VarInt<i32>,
    pub(crate) value: VarInt<i32>,
}

#[derive(Binary, Packet)]
#[id = 0x04]
pub struct AwardStatistics {
    pub(crate) statistic: Vec<Statistic>,
}

#[derive(Binary, Packet)]
#[id = 0x05]
pub struct AcknowledgeBlockChange {
    pub(crate) sequence_id: VarInt<i32>,
}

#[derive(Binary, Packet)]
#[id = 0x06]
pub struct SetBlockDestroyStage {
    pub(crate) entity_id: VarInt<i32>,
    pub(crate) location: Position,
    pub(crate) destroy_stage: i8,
}

#[derive(Binary, Packet)]
#[id = 0x07]
pub struct BlockEntityData {
    pub(crate) location: Position,
    pub(crate) entity_block_type: VarInt<i32>,
    pub(crate) nbt_tag: (),
}

#[derive(Binary, Packet)]
#[id = 0x08]
pub struct BlockAction {
    pub(crate) location: Position,
    pub(crate) action_id: u8,
    pub(crate) action_parameter: u8,
    pub(crate) block_type: VarInt<i32>,
}

#[derive(Binary, Packet)]
#[id = 0x09]
pub struct BlockUpdate {
    pub(crate) location: Position,
    pub(crate) block_id: VarInt<i32>,
}

#[derive(Binary, Packet)]
#[id = 0x0A]
pub struct BossBar {
    // TODO : found a way to represent that | probably enum
}

#[derive(Binary, Packet)]
#[id = 0x0B]
pub struct ChangeDifficulty {
    pub(crate) difficulty: u8,
    pub(crate) difficulty_locked: bool,
}

#[derive(Binary, Packet)]
#[id = 0x0C]
pub struct ChunkBatchFinished;

#[derive(Binary, Packet)]
#[id = 0x0D]
pub struct ChunkBatchStart;

#[derive(Debug, Binary, Clone)]
pub struct ChunkBiomeData {
    pub(crate) chunk_z: i32,
    pub(crate) chunk_x: i32,
    #[prefixed_length = "VarInt<i32>"]
    pub(crate) data: Vec<u8>,
}

#[derive(Binary, Packet)]
#[id = 0x0E]
pub struct ChunkBiomes {
    #[prefixed_length = "VarInt<i32>"]
    pub(crate) chunk_biome_data: Vec<ChunkBiomeData>,
}

#[derive(Binary, Packet)]
#[id = 0x0F]
pub struct ClearTitles {
    pub(crate) reset: bool,
}

// TODO : rename
#[derive(Debug, Binary, Clone)]
pub struct CommandSuggestion {
    pub(crate) eligible: String,
    pub(crate) jsp: Option<String>,
}

#[derive(Binary, Packet)]
#[id = 0x10]
pub struct CommandSuggestionsResponse {
    pub(crate) id: VarInt<i32>,
    pub(crate) start: VarInt<i32>,
    pub(crate) length: VarInt<i32>,
    #[prefixed_length = "VarInt<i32>"]
    pub(crate) matches: Vec<CommandSuggestion>,
}

#[derive(Binary, Packet)]
#[id = 0x11]
pub struct Commands {
    #[prefixed_length = "VarInt<i32>"]
    pub(crate) node: Vec<String>, // TODO : Node (commands)
    // #[prefixed_length = "VarInt<i32>"]
    pub(crate) root_index: VarInt<i32>,
}

#[derive(Binary, Packet)]
#[id = 0x12]
pub struct CloseContainer {
    pub(crate) window_id: u8,
}

pub type Slot = String; // TODO : Slot

#[derive(Binary, Packet)]
#[id = 0x13]
pub struct ContainerContent {
    pub(crate) window_id: u8,
    pub(crate) state_id: VarInt<i32>,
    #[prefixed_length = "VarInt<i32>"]
    pub(crate) slot_data: Vec<Slot>,
    pub(crate) carried_item: Slot,
}

#[derive(Binary, Packet)]
#[id = 0x14]
pub struct SetContainerProperty {
    pub(crate) window_id: u8,
    pub(crate) property: i16,
    pub(crate) value: i16,
}

#[derive(Binary, Packet)]
#[id = 0x15]
pub struct SetContainerSlot {
    pub(crate) windows_id: u8,
    pub(crate) state_id: VarInt<i32>,
    pub(crate) slot: i16,
    pub(crate) slot_data: Slot,
}

#[derive(Binary, Packet)]
#[id = 0x16]
pub struct SetCooldown {
    pub(crate) item_id: VarInt<i32>,
    pub(crate) cooldown_ticks: VarInt<i32>,
}

#[derive(Binary, Packet)]
#[id = 0x17]
pub struct ChatSuggestions {
    pub(crate) action: VarInt<i32>, // TODO : VarInt<enum>
    #[prefixed_length = "VarInt<i32>"]
    pub(crate) entries: Vec<String>,
}

pub type Identifier = String; // TODO : Identifier

#[derive(Binary, Packet)]
#[id = 0x18]
pub struct ClientboundPluginMessage {
    pub(crate) channel: Identifier,
    pub(crate) data: Vec<u8>,
}

#[derive(Binary, Packet)]
#[id = 0x19]
pub struct DamageEvent {
    pub(crate) entity_id: VarInt<i32>,
    pub(crate) source_type_id: VarInt<i32>,
    pub(crate) source_cause_id: VarInt<i32>,
    pub(crate) source_direct_id: VarInt<i32>,
    pub(crate) position: Option<Vector3d>,
}

#[derive(Binary, Packet)]
#[id = 0x1A]
pub struct DeleteMessage {
    pub(crate) message_id: VarInt<i32>,
    pub(crate) signature: Option<Vec<u8>>,
}

#[derive(Binary, Packet)]
#[id = 0x1B]
pub struct Disconnect {
    pub(crate) reason: String,
}

#[derive(Binary, Packet)]
#[id = 0x1C]
pub struct DisguisedChatMessage {
    pub(crate) message: String,
    pub(crate) chat_type: VarInt<i32>,
    pub(crate) sender_name: String,
    pub(crate) has_target_name: bool,
    #[some_if = "has_target_name"]
    pub(crate) target_name: Option<String>,
}

#[derive(Binary, Packet)]
#[id = 0x1D]
pub struct EntityEvent {
    pub(crate) entity_id: i32,
    pub(crate) entity_statue: u8, // TODO : enum
}

pub type Particle = String; // TODO : particle

#[derive(Binary, Packet)]
#[id = 0x1E]
pub struct Explosion {
    pub(crate) coordinate: Vector3d,
    pub(crate) strength: f32,
    #[prefixed_length = "VarInt<i32>"]
    pub(crate) record: Vec<Vector3<i8>>,
    pub(crate) player_motion: Vector2f,
    pub(crate) block_interaction: VarInt<i32>,
    pub(crate) small_explosion_particle_id: VarInt<i32>,
    pub(crate) small_explosion_particle_data: Particle, // TODO : paticles
    pub(crate) large_explosion_particle_id: VarInt<i32>,
    pub(crate) large_explosion_particle_data: Particle,
    pub(crate) sound_name: Identifier,
    pub(crate) has_fixed_range: Option<bool>,
    pub(crate) range: Option<f32>,
}

#[derive(Binary, Packet)]
#[id = 0x1F]
pub struct UnloadChunk {
    pub(crate) chunk_x: i32,
    pub(crate) chunk_z: i32,
}

#[derive(Binary, Packet)]
#[id = 0x20]
pub struct GameEvent {
    pub(crate) event: u8,
    pub(crate) value: f32,
}

#[derive(Binary, Packet)]
#[id = 0x21]
pub struct OpenHorseScreen {
    pub(crate) window_id: u8,
    pub(crate) slot_count: VarInt<i32>,
    pub(crate) entity_id: i32,
}

