use tables::AES_XORPAD_TABLE;

mod aes_custom;
mod tables;

pub struct MiHoYoEC {
    key: [u8; Self::AES_KEY_SIZE],
    scrambler_seed: [u8; Self::SCRAMBLER_SEED_SIZE],
}

impl MiHoYoEC {
    pub const AES_KEY_SIZE: usize = 16;
    pub const SCRAMBLER_SEED_SIZE: usize = 2048;
    pub const MAGIC: u32 = 0x45633262; // "Ec2b"

    pub fn scramble(&self) -> u64 {
        const FINAL_XOR: u64 = 0xCEAC3B5A867837AC;

        let scrambled = self
            .scrambler_seed
            .chunks_exact(8)
            .map(|chunk| u64::from_le_bytes(chunk.try_into().unwrap()))
            .fold(0xFFFFFFFFFFFFFFFF, |val, i| val ^ i);

        let key_qword_0 = u64::from_le_bytes(self.key[0..8].try_into().unwrap());
        let key_qword_1 = u64::from_le_bytes(self.key[8..16].try_into().unwrap());

        scrambled ^ key_qword_0 ^ key_qword_1 ^ FINAL_XOR
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FromBytesError {
    #[error("unexpected magic number at the start: {0:X}")]
    MagicMismatch(u32),
    #[error("out of bounds ({0}/{1})")]
    OutOfBounds(usize, usize),
    #[error("unexpected AES key size: {0}")]
    AesKeySizeMismatch(usize),
    #[error("unexpected scrambler seed payload size: {0}")]
    ScramblerSeedPayloadSizeMismatch(usize),
}

impl TryFrom<&[u8]> for MiHoYoEC {
    type Error = FromBytesError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        const MINIMAL_SIZE: usize = 12; // magic, key size, data size (u32s)

        assert_bounds(MINIMAL_SIZE, value.len())?;

        let magic = u32::from_be_bytes(value[0..4].try_into().unwrap());
        if magic != Self::MAGIC {
            return Err(FromBytesError::MagicMismatch(magic));
        }

        let key_size = u32::from_le_bytes(value[4..8].try_into().unwrap()) as usize;
        assert_bounds(MINIMAL_SIZE + key_size, value.len())?;

        let payload_size =
            u32::from_le_bytes(value[8 + key_size..8 + key_size + 4].try_into().unwrap()) as usize;
        assert_bounds(MINIMAL_SIZE + key_size + payload_size, value.len())?;

        let mut key = scramble_aes_key(
            value[8..8 + key_size]
                .try_into()
                .map_err(|_| FromBytesError::AesKeySizeMismatch(key_size))?,
        );

        key.iter_mut()
            .zip(tables::KEY_XORPAD_TABLE.iter())
            .for_each(|(a, b)| *a ^= b);

        Ok(Self {
            key,
            scrambler_seed: value[12 + key_size..12 + key_size + payload_size]
                .try_into()
                .map_err(|_| FromBytesError::ScramblerSeedPayloadSizeMismatch(payload_size))?,
        })
    }
}

fn scramble_aes_key(key: [u8; MiHoYoEC::AES_KEY_SIZE]) -> [u8; MiHoYoEC::AES_KEY_SIZE] {
    let mut round_keys = [0u8; 176];
    for round in 0..11 {
        for i in 0..16 {
            for j in 0..16 {
                let idx = (round << 8) + (i * 16) + j;
                round_keys[round * 16 + i] ^= AES_XORPAD_TABLE[1][idx] ^ AES_XORPAD_TABLE[0][idx];
            }
        }
    }

    let mut chip = [0u8; 16];
    aes_custom::oqs_mhy128_enc_c(&key, &round_keys, &mut chip);
    chip
}

#[inline(always)]
fn assert_bounds(required: usize, actual: usize) -> Result<(), FromBytesError> {
    (required <= actual)
        .then_some(())
        .ok_or(FromBytesError::OutOfBounds(required, actual))
}
