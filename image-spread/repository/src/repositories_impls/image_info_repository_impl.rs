use crate::repositories::ImageInfoRepository;
use util::AppResult;
use util::ImageInfo;
use infra::FileSystem;

pub struct ImageInfoRepositoryImpl<'r, T: FileSystem> {
    file_system: &'r T,
}

impl<'r, T: FileSystem> ImageInfoRepositoryImpl<'r, T> {
    pub fn new(file_system: &'r T) -> Self {
        Self {
            file_system: file_system,
        }
    }
}

impl<'r, T: FileSystem> ImageInfoRepository for ImageInfoRepositoryImpl<'r, T> {
    fn parse(&self, path: String, grid_width: u32) -> AppResult<ImageInfo> {
        Ok(ImageInfo{
            path: path,
            grid_width: grid_width,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use util::ImageInfo;

    #[test]
    fn test_parse() {
        assert_eq!(true, false);
    }
}
