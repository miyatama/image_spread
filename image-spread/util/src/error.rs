#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("create setting error: {}", .0)]
    SettingInitializeError(String),
    #[error("failed reqwest request: {}", .0)]
    // TodoApiError(#[from] reqwest::Error),
    TodoApiError(String),
    #[error("{}", .0)]
    Unknown(#[from] anyhow::Error),
}
