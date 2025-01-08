pub mod avatar;
mod hash;
pub mod nibbler;

use crate::avatar::generate_avatar;

pub fn avatar(name: &str) -> Vec<u8> {
    generate_avatar(name)
}

pub fn bs58_decode(data: &str) -> u64 {
    let mut to = [0u8; 8];
    to.copy_from_slice(&bs58::decode(data).into_vec().unwrap());
    u64::from_be_bytes(to)
}
