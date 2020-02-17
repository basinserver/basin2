pub mod serverbound;
pub use serverbound::*;
pub mod clientbound;
use super::Packet;
pub use clientbound::*;

hierarchy! {
    child<Packet> enum Status {
        StatusServerbound,
        StatusClientbound,
    }
}
