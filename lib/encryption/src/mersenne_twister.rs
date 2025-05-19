use rand_mt::Mt64;

pub fn generate_xorpad(seed: u64, big_endian: bool) -> [u8; 4096] {
    let mut buf = [0u8; 4096];
    let mut rng = Mt64::new(seed);

    if big_endian {
        (0..512)
            .for_each(|i| buf[i * 8..(i + 1) * 8].copy_from_slice(&rng.next_u64().to_be_bytes()));
    } else {
        (0..512)
            .for_each(|i| buf[i * 8..(i + 1) * 8].copy_from_slice(&rng.next_u64().to_le_bytes()));
    }

    buf
}
