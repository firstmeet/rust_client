use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Packet {
    pub data: Vec<u8>,
    pub no_delay: bool,
}