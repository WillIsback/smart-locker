use aes_gcm::aead::consts::U12;
use aes_gcm::aead::KeyInit;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use flate2::write::GzEncoder;
use flate2::Compression;

pub const SIGNATURE: &[u8; 8] = b"SMARTLKR"; // Signature fixe pour identifier le format
pub const FORMAT_VERSION: u8 = 1; // Version actuelle du format
pub const NONCE_SIZE: usize = 12; // Taille du nonce (12 octets pour AES-GCM)
pub const KEY_SIZE: usize = 32; // Taille de la clé symétrique (32 octets pour AES-256)

pub struct EncryptionConfig {
    pub signature: &'static [u8; 8],
    pub format_version: u8,
    pub nonce_size: usize,
    pub key_size: usize,
    pub compression: Compression,
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl EncryptionConfig {
    pub fn new() -> Self {
        Self {
            signature: SIGNATURE,
            format_version: FORMAT_VERSION,
            nonce_size: NONCE_SIZE,
            key_size: KEY_SIZE,
            compression: Compression::default(),
        }
    }

    /// Initialise le chiffreur AES-GCM avec une clé donnée
    pub fn init_cipher(&self, key_data: &[u8]) -> Result<Aes256Gcm, String> {
        if key_data.len() != self.key_size {
            return Err(format!(
                "Invalid key size: expected {} bytes, got {} bytes",
                self.key_size,
                key_data.len()
            ));
        }
        let key = Key::<Aes256Gcm>::from_slice(key_data);
        Ok(Aes256Gcm::new(key))
    }

    /// Génère un nonce aléatoire
    pub fn generate_nonce(&self) -> Nonce<U12> {
        let random_bytes = rand::random::<[u8; NONCE_SIZE]>();
        *Nonce::<U12>::from_slice(&random_bytes)
    }

    /// Initialise un compresseur Gzip
    pub fn init_compressor(&self) -> GzEncoder<Vec<u8>> {
        GzEncoder::new(Vec::new(), self.compression)
    }
}
