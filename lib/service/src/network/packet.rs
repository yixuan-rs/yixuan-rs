use yixuan_proto::{Message, NetCmd, head::PacketHead};

pub struct NetPacket {
    pub cmd_id: u16,
    pub head: PacketHead,
    pub body: Vec<u8>,
}

#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    #[error("packet is not fully received yet ({0}/{1})")]
    Incomplete(usize, usize),
    #[error("unexpected head magic (0x{0:X})")]
    HeadMagicMismatch(u32),
    #[error("unexpected tail magic (0x{0:X})")]
    TailMagicMismatch(u32),
    #[error("failed to decode PacketHead: {0}")]
    PacketHeadCorrupted(#[from] yixuan_proto::DecodeError),
}

#[derive(thiserror::Error, Debug)]
pub enum GetProtoError {
    #[error("unexpected cmd_id: {0} (expected: {1})")]
    CmdIdMismatch(u16, u16),
    #[error("failed to decode: {0}")]
    Decode(#[from] yixuan_proto::DecodeError),
}

impl NetPacket {
    pub const BASE_LENGTH: usize = 16;
    pub const HEAD_MAGIC: [u8; 4] = 0x1234567u32.to_be_bytes();
    pub const TAIL_MAGIC: [u8; 4] = 0x89ABCDEFu32.to_be_bytes();

    pub fn new<T: NetCmd + Message>(head: PacketHead, cmd: T) -> Self {
        Self {
            cmd_id: cmd.get_cmd_id(),
            head,
            body: cmd.encode_to_vec(),
        }
    }

    pub fn decode(buf: &[u8]) -> Result<(Self, usize), DecodeError> {
        assert_length(Self::BASE_LENGTH, buf)?;

        if buf[0..4] != Self::HEAD_MAGIC {
            return Err(DecodeError::HeadMagicMismatch(u32::from_be_bytes(
                buf[0..4].try_into().unwrap(),
            )));
        }

        let cmd_id = u16::from_be_bytes(buf[4..6].try_into().unwrap());
        let head_length = u16::from_be_bytes(buf[6..8].try_into().unwrap()) as usize;
        let body_length = u32::from_be_bytes(buf[8..12].try_into().unwrap()) as usize;

        assert_length(Self::BASE_LENGTH + head_length + body_length, buf)?;

        if buf[12 + head_length + body_length..16 + head_length + body_length] != Self::TAIL_MAGIC {
            return Err(DecodeError::TailMagicMismatch(u32::from_be_bytes(
                buf[12 + head_length + body_length..16 + head_length + body_length]
                    .try_into()
                    .unwrap(),
            )));
        }

        Ok((
            Self {
                cmd_id,
                head: PacketHead::decode(&buf[12..12 + head_length])?,
                body: buf[12 + head_length..12 + head_length + body_length].to_vec(),
            },
            Self::BASE_LENGTH + head_length + body_length,
        ))
    }

    pub fn get_proto<T: yixuan_proto::Message + Default + NetCmd>(
        &self,
    ) -> Result<T, GetProtoError> {
        if T::CMD_ID != self.cmd_id {
            return Err(GetProtoError::CmdIdMismatch(self.cmd_id, T::CMD_ID));
        }

        T::decode(self.body.as_ref()).map_err(GetProtoError::Decode)
    }
}

impl From<NetPacket> for Vec<u8> {
    fn from(value: NetPacket) -> Self {
        let mut buf = Vec::with_capacity(
            NetPacket::BASE_LENGTH + value.head.encoded_len() + value.body.len(),
        );

        buf.extend_from_slice(&NetPacket::HEAD_MAGIC);
        buf.extend_from_slice(&value.cmd_id.to_be_bytes());
        buf.extend_from_slice(&(value.head.encoded_len() as u16).to_be_bytes());
        buf.extend_from_slice(&(value.body.len() as u32).to_be_bytes());
        buf.extend_from_slice(&(value.head.encode_to_vec()));
        buf.extend_from_slice(&value.body);
        buf.extend_from_slice(&NetPacket::TAIL_MAGIC);

        buf
    }
}

#[inline(always)]
fn assert_length(required: usize, buf: &[u8]) -> Result<(), DecodeError> {
    (required <= buf.len())
        .then_some(())
        .ok_or(DecodeError::Incomplete(required, buf.len()))
}
