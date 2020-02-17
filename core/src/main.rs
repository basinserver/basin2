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

fn main() {
    server::start();
}
