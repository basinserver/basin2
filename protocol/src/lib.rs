#![allow(non_snake_case)]

#[macro_use]
pub mod macros;

mod chat;
pub mod connection;
pub mod nbt;
pub mod network;
mod packet;
pub mod packets;
pub mod result;
mod server;
#[macro_use]
extern crate enum_primitive;
use pkg_version::*;

pub use result::*;
pub use nbt::*;

pub const PROTOCOL_VERSION: u32 = pkg_version_major!();

pub use connection::*;
pub use packet::*;
pub use server::start_server;
