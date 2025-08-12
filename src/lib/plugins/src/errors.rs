use thiserror::Error;

#[derive(Debug, Error)]
pub enum PluginError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("loading error: {0}")]
    Load(String),
}
