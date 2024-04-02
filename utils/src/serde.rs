use serde::de::{Deserialize, Deserializer};
use ethnum::{u256,U256};

pub fn u256_from_le_bytes<'de, D>(deserializer: D) -> Result<U256, D::Error>
    where
        D: Deserializer<'de>,
{
    let s: &[u8;32] = &Deserialize::deserialize(deserializer)?;
    // do better hex decoding than this
    Ok(u256::from_le_bytes(*s))
}