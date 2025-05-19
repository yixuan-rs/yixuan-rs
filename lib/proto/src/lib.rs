#[path = "../out/cs_output.rs"]
pub mod client;

#[path = "../out/head.rs"]
pub mod head;

#[path = "../out/action.rs"]
pub mod action;

#[path = "../out/common.rs"]
pub mod common;

#[path = "../out/server_only.rs"]
pub mod server_only;

pub use client::*;
pub use prost::{DecodeError, Message};

pub trait NetCmd {
    const CMD_ID: u16;

    fn get_cmd_id(&self) -> u16 {
        Self::CMD_ID
    }

    fn is_client_to_server_cmd(&self) -> bool {
        Self::CMD_ID < 10000
    }
}

pub trait NetResponse {
    fn set_retcode(&mut self, retcode: i32);
    fn get_retcode(&self) -> i32;
}
