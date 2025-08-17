use crate::infras::FileSystem;
use util::AppResult;

pub struct FileSystemImpl;

impl FileSystemImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl FileSystem for FileSystemImpl {
    fn open_image_file(&self, path: String) -> AppResult<image::DynamicImage> {
        Ok(image::open(path).expect("failed to load image"))
    }
}
