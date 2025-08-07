use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum NetEncryptionError {
    #[error("RSA error: {0}")]
    RsaError(String),

    #[error("AES error: {0}")]
    AesError(String),

    #[error("Invalid verify token")]
    InvalidVerifyToken,
}
