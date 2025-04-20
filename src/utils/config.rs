use aes_gcm::aead::consts::U12;
use aes_gcm::aead::KeyInit;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use flate2::write::GzEncoder;
use flate2::Compression;

pub const SIGNATURE: &[u8; 8] = b"SMARTLKR"; // Signature fixe pour identifier le format
pub const FORMAT_VERSION: u8 = 1; // Version actuelle du format
pub const NONCE_SIZE: usize = 12; // Taille du nonce (12 octets pour AES-GCM)
pub const KEY_SIZE: usize = 32; // Taille de la clé symétrique (32 octets pour AES-256)

/// Configuration structure for encryption settings.
/// This structure defines the parameters used for AES-GCM encryption,
/// including the signature, format version, nonce size, key size, and compression settings.
pub struct EncryptionConfig {
    /// Fixed signature to identify the format.
    pub signature: &'static [u8; 8],
    /// Current version of the format.
    pub format_version: u8,
    /// Size of the nonce (12 bytes for AES-GCM).
    pub nonce_size: usize,
    /// Size of the symmetric key (32 bytes for AES-256).
    pub key_size: usize,
    /// Compression level for Gzip.
    pub compression: Compression,
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl EncryptionConfig {
    /// Creates a new instance of the encryption configuration with default values.
    pub fn new() -> Self {
        Self {
            signature: SIGNATURE,
            format_version: FORMAT_VERSION,
            nonce_size: NONCE_SIZE,
            key_size: KEY_SIZE,
            compression: Compression::default(),
        }
    }

    /// Initializes the AES-GCM cipher with the provided key.
    ///
    /// # Arguments
    /// * `key_data` - A byte slice containing the encryption key.
    ///
    /// # Returns
    /// * `Ok(Aes256Gcm)` - The initialized AES-GCM cipher.
    /// * `Err(String)` - An error message if the key size is invalid.
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

    /// Generates a random nonce for AES-GCM encryption.
    ///
    /// # Returns
    /// * `Nonce<U12>` - A randomly generated nonce of 12 bytes.
    pub fn generate_nonce(&self) -> Nonce<U12> {
        let random_bytes = rand::random::<[u8; NONCE_SIZE]>();
        *Nonce::<U12>::from_slice(&random_bytes)
    }

    /// Initializes a Gzip compressor with the specified compression level.
    ///
    /// # Returns
    /// * `GzEncoder<Vec<u8>>` - A Gzip encoder for compressing data.
    pub fn init_compressor(&self) -> GzEncoder<Vec<u8>> {
        GzEncoder::new(Vec::new(), self.compression)
    }
}
