use crate::infras::FileSystem;
use util::AppResult;
use util::ImageInfo;

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

#[cfg(test)]
mod tests {
    use super::*;
    use util::ImageInfo;

    #[test]
    fn test_open_image_file() {
        assert_eq!(true, false);
    }
}
