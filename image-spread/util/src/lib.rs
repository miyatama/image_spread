mod entity;
mod error;

pub use entity::image_info::CellBlock;
pub use entity::image_info::ImageGridCell;
pub use entity::image_info::ImageInfo;

pub use entity::basics::Line;
pub use entity::basics::Point;

pub use error::Error;

pub type AppResult<T> = anyhow::Result<T, Error>;
