mod entity;
mod error;

pub use entity::image_info::ImageInfo;

pub use error::Error;

pub type AppResult<T> = anyhow::Result<T, Error>;
