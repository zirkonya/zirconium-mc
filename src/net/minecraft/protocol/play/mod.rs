use crate::{format::{nbt::NBTTag, text::{chat::Chat, identifier::Identifier}, uuid::Uuid}, gen_bin, gen_struct, minecraft::{command::Node, inventory::slot::Slot, utils::{Color, Division}}, tools::{maths::{varint::varint::VarInt, vector::{position::Position, vector2::Vector2f, vector3::{Vector3, Vector3d, Vector3f}}}, utils::bin::{binarray::Array, Binary, BinaryError}}};


pub type Id = VarInt<i32>;
pub type Signature = Option<Array<u8>>;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Animation {
    SwingMainArm = 0,
    LeaveBed = 1,
    SwingOffHand = 2,
    CriticalEffect = 3,
    MagicCriticalEffect = 4
}

impl Binary for Animation {
    fn to_bin(&self) -> Vec<u8> {
        vec![*self as u8]
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, BinaryError> where Self: Sized {
        match bin[0] {
            0 => Ok(Animation::SwingMainArm),
            1 => Ok(Animation::LeaveBed),
            2 => Ok(Animation::SwingOffHand),
            3 => Ok(Animation::CriticalEffect),
            4 => Ok(Animation::MagicCriticalEffect),
            _ => Err(BinaryError::wrong_format())
        }
    }

    #[inline(always)]
    fn byte_length(&self) -> usize {
        1
    }
}

gen_bin!(BitSet { length: VarInt<i32>, data: Array<i64> });

/* 0x00 */ gen_bin!(BundleDelimiterPacket);
/* 0x01 */ gen_bin!(SpawnEntityPacket { entity_id: Id, entity_uuid: Uuid, entity_type: VarInt<i32>, coordinate: Vector3d, pitch: u8, yaw: u8, head_yaw: u8, data: VarInt<i32>, velocity: Vector3<i16> });
/* 0x02 */ gen_bin!(SpawnExperienceOrbPacket { entity_id: Id, coordinate: Vector3d, count: i16  });
/* 0x03 */ gen_bin!(EntityAnimationPacket { entity_id: Id, animation: Animation });
/* 0x04 */ gen_bin!(AwardStatisticsPacket { count: VarInt<i32>, statistic: Array<(VarInt<i32>, VarInt<i32>, VarInt<i32>)> });   // Probably modified with u8
/* 0x05 */ gen_bin!(AcknowledgeBlockChangePacket { sequence_id: Id });
/* 0x06 */ gen_bin!(SetBlockDestroyStagePacket { entity_id: Id, location: Position, destroy_stage: i8 });
/* 0x07 */ gen_bin!(BlockEntityDataPacket { location: Position, block_entity_type: VarInt<i32>, nbt_data: NBTTag });
/* 0x08 */ gen_bin!(BlockActionPacket { location: Position, action_id: u8, action_parameter: u8, block_type: Id });
/* 0x09 */ gen_bin!(BlockUpdatePacket { location: Position, block_id: Id });

/* 0x0A */
gen_struct!(
    BossBar {
        uuid:       Uuid,
        action:     VarInt<i32>,
        title:      Option<Chat>,
        health:     Option<f32>,
        color:      Option<Color>,
        division:   Option<Division>,
        flags:      Option<u8>
    }
);

trait TraitName {
     fn add(uuid: Uuid, title: Chat, health: f32, color: Color, division: Division, flags: u8) -> Self;

     fn remove(uuid: Uuid) -> Self;

     fn update_health(uuid: Uuid, health: f32) -> Self;

     fn update_title(uuid: Uuid, title: Chat) -> Self;

     fn update_style(uuid: Uuid, color: Color, division: Division) -> Self;

     fn update_flags(uuid: Uuid, flags: u8) -> Self;
}

impl TraitName for BossBar {
     fn add(uuid: Uuid, title: Chat, health: f32, color: Color, division: Division, flags: u8) -> Self {
        Self::new(uuid, VarInt::new(0), Some(title), Some(health), Some(color), Some(division), Some(flags))
    }

     fn remove(uuid: Uuid) -> Self {
        Self::new(uuid, VarInt::new(1), None, None, None, None, None)
    }

     fn update_health(uuid: Uuid, health: f32) -> Self {
        Self::new(uuid, VarInt::new(2), None, Some(health),None, None, None)
    }

     fn update_title(uuid: Uuid, title: Chat) -> Self {
        Self::new(uuid, VarInt::new(3), Some(title), None, None, None, None)
    }

     fn update_style(uuid: Uuid, color: Color, division: Division) -> Self {
        Self::new(uuid, VarInt::new(4), None, None, Some(color), Some(division), None)
    }

     fn update_flags(uuid: Uuid, flags: u8) -> Self {
        Self::new(uuid, VarInt::new(5), None, None, None, None,  Some(flags))
    }
}

impl Binary for BossBar {
    fn to_bin(&self) -> Vec<u8> {
        let mut vec = self.uuid.to_bin();
        vec.append(&mut self.action.to_bin());
        match self.action.value() {
            0 => {
                vec.append(&mut self.title().unwrap_or_default().to_bin());
                vec.append(&mut self.health().unwrap_or_default().to_bin());
                vec.append(&mut self.color().unwrap_or_default().to_bin());
                vec.append(&mut self.division().unwrap_or_default().to_bin());
                vec.append(&mut self.flags().unwrap_or_default().to_bin());
            },
            1 => {},
            2 => {
                vec.append(&mut self.health().unwrap_or_default().to_bin());
            },
            3 => {
                vec.append(&mut self.title().unwrap_or_default().to_bin());
            },
            4 => {
                vec.append(&mut self.color().unwrap_or_default().to_bin());
                vec.append(&mut self.division().unwrap_or_default().to_bin());
            },
            5 => {
                vec.append(&mut self.flags().unwrap_or_default().to_bin());
            },
            _ => {}
        }
        vec
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, BinaryError> where Self: Sized {
        let uuid = Uuid::from_bin(bin.clone())?;
        let mut cursor = uuid.byte_length();
        let action = VarInt::from_bin(bin[cursor..].to_vec())?;
        cursor += action.byte_length();
        match action.value() {
            0 => {
                let title = Chat::from_bin(bin[cursor..].to_vec())?;
                cursor += title.byte_length();
                let health = f32::from_bin(bin[cursor..].to_vec())?;
                cursor += health.byte_length();
                let color = Color::from_bin(bin[cursor..].to_vec())?;
                cursor += color.byte_length();
                let division = Division::from_bin(bin[cursor..].to_vec())?;
                cursor += division.byte_length();
                let flags = u8::from_bin(bin[cursor..].to_vec())?;
                Ok(Self::add(uuid, title, health, color, division, flags))
            },
            1 => {
                Ok(Self::remove(uuid))
            },
            2 => {
                let health = f32::from_bin(bin[cursor..].to_vec())?;
                Ok(Self::update_health(uuid, health))
            },
            3 => {
                let title = Chat::from_bin(bin[cursor..].to_vec())?;
                Ok(Self::update_title(uuid, title))
            },
            4 => {
                let color = Color::from_bin(bin[cursor..].to_vec())?;
                cursor += color.byte_length();
                let division = Division::from_bin(bin[cursor..].to_vec())?;
                Ok(Self::update_style(uuid, color, division))
            },
            5 => {
                let flags = u8::from_bin(bin[cursor..].to_vec())?;
                Ok(Self::update_flags(uuid, flags))
            },
            _ => { Err(BinaryError::wrong_format()) }
        }
        
    }

    fn byte_length(&self) -> usize {
        self.uuid.byte_length() +
        self.action.byte_length() +
        match self.title() { Some(t) => t.byte_length(), None => 0_usize } +
        match self.health() { Some(t) => t.byte_length(), None => 0_usize } +
        match self.color() { Some(t) => t.byte_length(), None => 0_usize } +
        match self.division() { Some(t) => t.byte_length(), None => 0_usize } +
        match self.flags() { Some(t) => t.byte_length(), None => 0_usize }
    }
}

/* 0x0B */ gen_bin!(ChangeDificultyPacket { difficulty: u8, locked: bool });
/* 0x0C */ gen_bin!(ChunkBatchFinishedPacket { batch_size: VarInt<i32> });
/* 0x0D */ gen_bin!(BatchStartPacket);
/* 0x0E */ gen_bin!(ChunkBiomePacket { number_of_chunk: VarInt<i32>, data: Array<(i32, i32, VarInt<i32>, Array<u8>)> });
/* 0x0F */ gen_bin!(ClearTitlesPacket { reset: bool });
/* 0x10 */ gen_bin!(CommandSuggestionResponsePacket { id: Id, start: VarInt<i32>, length: VarInt<i32>, count: VarInt<i32>, matches: Array<(String, bool, Option<Chat>)> });
/* 0x11 */ gen_bin!(CommandPacket { count: VarInt<i32>, nodes: Array<Node>, root_index: VarInt<i32> });
/* 0x12 */ gen_bin!(CloseContainerPacket { window_id: u8 });
/* 0x13 */ gen_bin!(SetContainerContentPacket { window_id: u8, state_id: Id, count: VarInt<i32>, slot_data: Array<Slot>, carried_item: Slot });
/* 0x14 */ gen_bin!(SetContainerPropertyPacket { window_id: u8, property: i16, value: i16 });
/* 0x15 */ gen_bin!(SetContainerSlotPacket { window_id: u8, state_id: VarInt<i32>, slot: i16, slot_data: Slot });
/* 0x16 */ gen_bin!(SetCooldownPacket { item_id: VarInt<i32>, cooldown_ticks: VarInt<i32> });
/* 0x17 */ gen_bin!(ChatSuggestionsPacket { action: VarInt<i32>, count: VarInt<i32>, entries: Array<String> });
/* 0x18 */ gen_bin!(ClientBoundPluginMessagePacket { channel: Identifier, data: Array<u8> });
/* 0x19 */ gen_bin!(DamageEventPacket { entity_id: Id, source_type_id: Id, source_cause_id: Id, source_direct_id: Id, has_source_position: bool, source_position: Option<Vector3d> });
/* 0x1A */ gen_bin!(DeleteMessagePacket { message_id: Id, signature: Signature });
/* 0x1B */ gen_bin!(DisconnectPacket { reason: Chat });
/* 0x1C */ gen_bin!(DisguisedChatMessagePacket { message: Chat, chat_type: VarInt<i32>, sender_name: Chat, has_target_name: bool, target_name: Chat });
/* 0x1D */ gen_bin!(EntityEventPacket { entity_id: i32, entity_status: u8 /* TODO: https://wiki.vg/Entity_statuses */ });
/* 0x1E */ gen_bin!(ExplosionPacket { 
                coordinate: Vector3d,
                strength: f32,
                record_count: VarInt<i32>,
                records: Array<Vector3<i8>>,
                player_motion: Vector2f,
                block_interaction: VarInt<i32>,
                small_explosion_particle_id: Id,
                small_explosion_particle_data: (), // TODO : Varies
                large_explosion_particle_id: Id,
                large_explosion_particle_data: (), // TODO : Varies
                sound_name: Identifier,
                has_fixed_range: Option<bool>,
                range: Option<f32>
});
/* 0x1F */ gen_bin!(UnloadChunkPacket { z: i32, x: i32 });
/* 0x20 */ gen_bin!(GameEventPacket { event: u8, value: f32 });
/* 0x21 */ gen_bin!(OpenHorseScreenPacket { window_id: Id, slot_count: VarInt<i32>, entity_id: i32 });
/* 0x22 */ gen_bin!(HurtAnimationPacket { entity_id: VarInt<i32>, yaw: f32 });
/* 0x23 */ gen_bin!(InitializeWorldBorderPacket { x: f64, z: f64, old_diameter: f64, new_diameter: f64, speed: VarInt<i64>, portal_teleport_boundary: VarInt<i32>, warning_block: VarInt<i32>, warning_time: VarInt<i32> });

/*  distance = max(min(resizeSpeed * 1000 * warningTime, abs(targetDiameter - currentDiameter)), warningDistance);
    if (playerDistance < distance) {
        warning = 1.0 - playerDistance / distance;
    } else {
        warning = 0.0;
    }
*/
/* 0x24 */ gen_bin!(ClientboundKeepAlivePacket { keep_alive_id: i64 });
/* 0x25 */ gen_bin!(ChunkDataAndUpdateLightPacket {
                x: i32,
                z: i32,
                height_map: NBTTag,
                size: VarInt<i32>,
                data: Array<u8>,
                number_of_block_entity: VarInt<i32>,
                block_entity: Array<(u8, i16, VarInt<i32>, NBTTag)>,
                sky_light_mask: BitSet,
                block_light_mask: BitSet,
                empty_sky_light_mask: BitSet,
                empty_block_light_mask: BitSet,
                sky_light_array_count: VarInt<i32>,
                sky_light_arrays: Array<(VarInt<i32>, Array<u8>)>,
                block_light_array_count: VarInt<i32>,
                block_light_arrays: Array<(VarInt<i32>, Array<u8>)>
});
/* 0x26 */ gen_bin!(WorldEventPacket { event: i32, location: Position, data: i32, disable_relative_volume: bool });
/* 0x27 */ gen_bin!(ParticlePacket { particle_id: Id, long_distance: bool, position: Vector3d, offset: Vector3f, max_speed: f32, particle_count: i32, data: () /* varies */ });
/* 0x28 */ gen_bin!(UpdateLightPacket { 
                x: VarInt<i32>,
                z: VarInt<i32>,
                sky_light_mask: BitSet,
                block_light_mask: BitSet,
                empty_sky_light_mask: BitSet,
                empty_block_light_mask: BitSet,
                sky_light_array_count: VarInt<i32>,
                sky_light_arrays: Array<(VarInt<i32>, Array<u8>)>,
                block_light_array_count: VarInt<i32>,
                block_light_arrays: Array<(VarInt<i32>, Array<u8>)>
});
/* 0x29 */ gen_bin!(LoginPacket { 
                entity_id: i32,
                is_hardcore: bool,
                dimension_count: VarInt<i32>,
                dimension_names: Array<Identifier>,
                max_player: VarInt<i32>,
                view_distance: VarInt<i32>,
                simulation_distance: VarInt<i32>,
                reduced_debug_info: bool,
                enable_respawn_screen: bool,
                do_limited_crafting: bool,
                dimension_type: Identifier,
                dimension_name: Identifier,
                hashed_seed: i64,
                gamemode: u8,
                previous_game_mode: i8,
                is_debug: bool,
                is_flat: bool,
                has_death_location: bool,
                death_dimension_name: Option<Identifier>,
                death_location: Option<Position>,
                portal_cooldown: VarInt<i32>
});
gen_bin!(MapIcon {
                icon_type: Id,
                x: i8,
                z: i8,
                direction: i8,
                has_display_name: bool,
                display_name: Option<Chat>
});
/* 0x2A */ gen_bin!(MapDataPacket {
                map_id: Id,
                scale: i8,
                locked: bool,
                has_icon: bool,
                icon_count: Option<VarInt<i32>>,
                icon: Option<Array<MapIcon>>,
                columns: u8,
                rows: Option<u8>,
                x: Option<u8>,
                z: Option<u8>,
                length: VarInt<i32>,
                data: Option<Array<u8>>
});
