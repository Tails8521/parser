use bitstream_reader::BitRead;
use serde::Serialize;

use crate::demo::packet::datatable::ServerClass;
use crate::demo::sendprop::SendProp;
use crate::{Parse, ParserState, Result, Stream};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct EntityId(u32);

impl EntityId {
    pub fn new(num: u32) -> Self {
        EntityId(num)
    }
}

#[derive(BitRead, Clone, Copy, Debug)]
#[discriminant_bits = 3]
pub enum PVS {
    PRESERVE = 0,
    ENTER = 1,
    LEAVE = 2,
    DELETE = 4,
}

#[derive(Debug)]
pub struct PacketEntity {
    server_class: ServerClass,
    entity_index: EntityId,
    props: Vec<SendProp>,
    in_pvs: bool,
    pvs: PVS,
    serial_number: u32,
    delay: Option<u32>,
}

#[derive(Debug)]
pub struct PacketEntitiesMessage {
    entities: Vec<PacketEntity>,
    removed_entities: Vec<EntityId>,
    max_entries: u16,
    delta: Option<u32>,
    base_line: u8,
    updated_base_line: bool,
}

impl Parse for PacketEntitiesMessage {
    fn parse(stream: &mut Stream, _state: &ParserState) -> Result<Self> {
        let max_entries = stream.read_sized(11)?;
        let delta = stream.read()?;
        let base_line = stream.read_sized(1)?;
        let _updated_entries: u16 = stream.read_sized(11)?;
        let length: u32 = stream.read_sized(20)?;
        let updated_base_line = stream.read()?;
        let _data = stream.read_bits(length as usize)?;

        // TODO

        Ok(PacketEntitiesMessage {
            entities: Vec::new(),
            removed_entities: Vec::new(),
            max_entries,
            delta,
            base_line,
            updated_base_line,
        })
    }
}
