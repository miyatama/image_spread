#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("create setting error: {}", .0)]
    SettingInitializeError(String),
    #[error("invalid image info: {}", .0)]
    InvalidImageInfoError(String),
    #[error("{}", .0)]
    Unknown(#[from] anyhow::Error),
}
