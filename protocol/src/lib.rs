#![allow(non_snake_case)]

#[macro_use]
pub mod macros;
#[macro_use]
extern crate basin2_lib;

mod chat;
pub mod connection;
pub mod network;
mod packet;
pub mod packets;
mod server;
#[macro_use]
extern crate enum_primitive;
use pkg_version::*;

pub const PROTOCOL_VERSION: u32 = pkg_version_major!();

pub use connection::*;
pub use packet::*;
pub use server::start_server;
