#[macro_use]
extern crate lazy_static;
#[macro_use]
mod result;
pub use result::*;

mod entity;
mod player;
mod server;
mod util;
mod world;
mod lib;
mod data;

use tokio::runtime;
use log::LevelFilter;
use log::*;
use env_logger::Builder;
use std::sync::Arc;

fn start_tokio() -> Result<runtime::Runtime> {
    Ok(runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()?)
}

fn main() {
    Builder::from_default_env()
        .filter_level(LevelFilter::Info)
        .init();
    let server = Arc::new(server::Server::new());
    start_tokio().unwrap().block_on(server::start(server))
}
