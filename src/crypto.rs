use anyhow::{Result, Context};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit, OsRng, rand_core::RngCore};

const NONCE_SIZE: usize = 12;

pub fn generate_master_key() -> Vec<u8> {
    let mut key = vec![0u8; 32];
    OsRng.fill_bytes(&mut key);
    key
}

pub fn save_master_key(path: &str, key: &[u8]) -> Result<()> {
    std::fs::write(path, key)
        .with_context(|| format!("Failed to save master key to '{}'", path))
}

pub fn load_master_key(path: &str) -> Result<Vec<u8>> {
    std::fs::read(path)
        .with_context(|| format!("Failed to read master key from '{}'", path))
}

pub fn encrypt(plaintext: &str, master_key: &[u8]) -> Result<Vec<u8>> {
    let  k = aes_gcm::Key::<Aes256Gcm>::from_slice(master_key);
    let cipher = Aes256Gcm::new(k);

    let mut nonce = [0u8; NONCE_SIZE];
    OsRng.fill_bytes(&mut nonce);

    let ciphertext = cipher.encrypt(Nonce::from_slice(&nonce), plaintext.as_bytes())
        .map_err(|e| anyhow::anyhow!("Encryption failed: {:?}", e))?;

    let mut combined = nonce.to_vec();
    combined.extend_from_slice(&ciphertext);

    Ok(combined)
}

pub fn decrypt(ciphertext: &[u8], master_key: &[u8]) -> Result<String> {
    if ciphertext.len() < NONCE_SIZE {
        anyhow::bail!("Ciphertext too short");
    }
    let  k = aes_gcm::Key::<Aes256Gcm>::from_slice(master_key);
    let (nonce, data) = ciphertext.split_at(NONCE_SIZE);
    let cipher = Aes256Gcm::new(k);

    let plaintext = cipher.decrypt(Nonce::from_slice(nonce), data)
        .map_err(|e| anyhow::anyhow!("Encryption failed: {:?}", e))?;

    Ok(String::from_utf8(plaintext)
        .map_err(anyhow::Error::new)
        .context("Invalid UTF-8 in decrypted text")?)
}
