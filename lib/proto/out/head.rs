// This file is @generated by prost-build.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PacketHead {
    #[prost(uint32, tag = "1")]
    pub packet_id: u32,
    #[prost(uint32, tag = "2")]
    pub player_uid: u32,
    #[prost(uint64, tag = "3")]
    pub session_id: u64,
    #[prost(uint64, tag = "4")]
    pub gate_session_id: u64,
    #[prost(uint32, tag = "11")]
    pub ack_packet_id: u32,
}
