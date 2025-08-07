pub mod errors;

use aes::Aes128;
use aes::cipher::{AsyncStreamCipher, KeyIvInit};
use cfb8::{Decryptor as Cfb8Decryptor, Encryptor as Cfb8Encryptor};
use errors::NetEncryptionError;
use rsa::pkcs1v15::Pkcs1v15Encrypt;
use rsa::rand_core::{OsRng, RngCore};
use rsa::{RsaPrivateKey, RsaPublicKey};

pub fn generate_rsa_keypair() -> Result<(RsaPrivateKey, RsaPublicKey), NetEncryptionError> {
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 1024)
        .map_err(|e| NetEncryptionError::RsaError(e.to_string()))?;
    let public_key = RsaPublicKey::from(&private_key);
    Ok((private_key, public_key))
}

pub fn generate_verify_token() -> [u8; 4] {
    let mut token = [0u8; 4];
    OsRng.fill_bytes(&mut token);
    token
}

pub fn decrypt_shared_secret(
    private_key: &RsaPrivateKey,
    input: &[u8],
) -> Result<Vec<u8>, NetEncryptionError> {
    private_key
        .decrypt(Pkcs1v15Encrypt, input)
        .map_err(|e| NetEncryptionError::RsaError(e.to_string()))
}

pub struct Aes128Cfb8Encryptor {
    key: [u8; 16],
    iv: [u8; 16],
}

impl Aes128Cfb8Encryptor {
    pub fn new(key: [u8; 16], iv: [u8; 16]) -> Self {
        Self { key, iv }
    }

    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, NetEncryptionError> {
        let mut buf = data.to_vec();
        let cipher = Cfb8Encryptor::<Aes128>::new_from_slices(&self.key, &self.iv)
            .map_err(|e| NetEncryptionError::AesError(e.to_string()))?;
        cipher.encrypt(&mut buf);
        Ok(buf)
    }
}

pub struct Aes128Cfb8Decryptor {
    key: [u8; 16],
    iv: [u8; 16],
}

impl Aes128Cfb8Decryptor {
    pub fn new(key: [u8; 16], iv: [u8; 16]) -> Self {
        Self { key, iv }
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, NetEncryptionError> {
        let mut buf = data.to_vec();
        let cipher = Cfb8Decryptor::<Aes128>::new_from_slices(&self.key, &self.iv)
            .map_err(|e| NetEncryptionError::AesError(e.to_string()))?;
        cipher.decrypt(&mut buf);
        Ok(buf)
    }
}
