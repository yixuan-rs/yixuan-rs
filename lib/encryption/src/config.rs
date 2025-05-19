use serde::{Deserialize, Deserializer};

use crate::{ec::MiHoYoEC, mersenne_twister};

#[derive(Deserialize)]
pub struct RsaVersion {
    #[serde(deserialize_with = "from_base64")]
    pub client_public_key: Vec<u8>,
    #[serde(deserialize_with = "from_base64")]
    pub server_private_key: Vec<u8>,
}

pub struct ScrambledKey {
    pub seed: String,
    pub xorpad: [u8; 4096],
}

pub fn from_base64<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = <String>::deserialize(deserializer)?;
    base64_simd::STANDARD
        .decode_to_vec(s)
        .map_err(serde::de::Error::custom)
}

impl<'de> Deserialize<'de> for ScrambledKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let seed = <String>::deserialize(deserializer)?;
        let seed_buf = base64_simd::STANDARD
            .decode_to_vec(&seed)
            .map_err(serde::de::Error::custom)?;

        let scrambler = MiHoYoEC::try_from(seed_buf.as_ref()).map_err(serde::de::Error::custom)?;

        Ok(Self {
            seed,
            xorpad: mersenne_twister::generate_xorpad(scrambler.scramble(), false),
        })
    }
}
