mod decoder;
mod encoder;
mod item;

pub use decoder::Decoder;
pub use encoder::Encoder;
pub use item::RlpItem;

pub mod prelude {
    pub use crate::{Decoder, Encoder, RlpItem};
}

pub fn encode(item: &RlpItem) -> Vec<u8> {
    Encoder::new().encode(item)
}

pub fn decode(data: &[u8]) -> Result<(RlpItem, usize), String> {
    Decoder::new().decode(data)
}
