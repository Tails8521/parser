use std::collections::HashMap;

use bitstream_reader::{BitRead, LittleEndian, ReadError};

use crate::demo::gamevent::GameEventValue;
use crate::demo::header::Header;
use crate::demo::packet::Packet;
pub use crate::demo::parser::state::ParserState;
use crate::Stream;

mod state;

/// Errors that can occur during parsing
#[derive(Debug)]
pub enum ParseError {
    /// Error while reading bits from stream
    ReadError(ReadError),
    /// Packet identifier is invalid
    InvalidPacketType(u8),
    /// Message identifier is invalid
    InvalidMessageType(u8),
    /// SendProp type is invalid
    InvalidSendPropType(u8),
    /// Invalid structure found while creating array SendProp
    InvalidSendPropArray(String),
    /// Expected amount of data left after parsing an object
    DataRemaining(usize),
    /// String table that was send for update doesn't exist
    StringTableNotFound(u8),
    /// A unknown game event type was read
    UnknownGameEvent(String),
    /// A read game event doesn't contain the expected values
    InvalidGameEvent {
        name: String,
        value: GameEventValue,
    },
}

impl From<ReadError> for ParseError {
    fn from(err: ReadError) -> ParseError {
        ParseError::ReadError(err)
    }
}

pub type Result<T> = std::result::Result<T, ParseError>;

pub trait Parse: Sized {
    fn parse(stream: &mut Stream, state: &ParserState) -> Result<Self>;
}

impl<T: BitRead<LittleEndian>> Parse for T {
    fn parse(stream: &mut Stream, state: &ParserState) -> Result<Self> {
        Self::read(stream).map_err(ParseError::from)
    }
}

pub struct DemoParser {
    stream: Stream,
    state: ParserState,
}

impl DemoParser {
    pub fn new(stream: Stream) -> Self {
        DemoParser {
            state: ParserState::new(),
            stream,
        }
    }

    pub fn read<T: Parse>(&mut self) -> Result<T> {
        T::parse(&mut self.stream, &self.state)
    }

    pub fn stream_pos(&self) -> usize {
        self.stream.pos()
    }

    pub fn set_stream_pos(&mut self, pos: usize) -> Result<()> {
        self.stream.set_pos(pos)?;
        Ok(())
    }

    pub fn parse_demo(mut self) -> Result<(Header, Vec<Packet>)> {
        let header = self.read::<Header>()?;
        let mut packets = vec![];
        loop {
            let packet = self.read::<Packet>()?;
            match packet {
                Packet::Stop(_) => break,
                packet => packets.push(packet),
            }
        }
        Ok((header, packets))
    }
}
