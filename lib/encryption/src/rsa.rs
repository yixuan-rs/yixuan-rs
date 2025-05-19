use rsa::{
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey, pkcs1v15::SigningKey, sha2::Sha256,
    signature::RandomizedSigner,
};

const RSA_CHUNK_SIZE: usize = 117;

pub fn encrypt(public_key: &[u8], data: &[u8]) -> String {
    let public_key: RsaPublicKey = rsa::pkcs8::DecodePublicKey::from_public_key_der(public_key)
        .expect("Failed to read public key from der");
    let mut rng = rand::thread_rng();

    let mut result: Vec<u8> = Vec::new();
    for chunk in data.chunks(RSA_CHUNK_SIZE) {
        let encrypted_chunk = public_key
            .encrypt(&mut rng, Pkcs1v15Encrypt, chunk)
            .expect("Encryption failed");

        result.extend(encrypted_chunk);
    }

    base64_simd::STANDARD.encode_to_string(result)
}

pub fn sign(private_key: &[u8], data: &[u8]) -> String {
    let private_key = rsa::pkcs8::DecodePrivateKey::from_pkcs8_der(private_key)
        .expect("Failed to read pkcs8 private key");
    let signing_key = SigningKey::<Sha256>::new(private_key);

    base64_simd::STANDARD.encode_to_string(Box::<[u8]>::from(
        signing_key.sign_with_rng(&mut rand::thread_rng(), data),
    ))
}

pub fn decrypt(private_key: &[u8], cipher: &str) -> Option<Vec<u8>> {
    let private_key: RsaPrivateKey = rsa::pkcs8::DecodePrivateKey::from_pkcs8_der(private_key)
        .expect("Failed to read pkcs8 private key");

    let cipher = base64_simd::STANDARD.decode_to_vec(cipher).ok()?;
    private_key.decrypt(Pkcs1v15Encrypt, &cipher).ok()
}
