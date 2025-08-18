use crate::infras::FileSystem;
use util::AppResult;
use util::Error::SaveFileError;

pub struct FileSystemImpl;

const EBIDENCE_DIR: &str = "ebidence";

impl FileSystemImpl {
    pub fn new() -> Self {
        Self {}
    }

    fn create_ebidence_dir() {
        std::fs::create_dir_all(EBIDENCE_DIR).unwrap();
    }
}

impl FileSystem for FileSystemImpl {
    fn open_image_file(&self, path: String) -> AppResult<image::DynamicImage> {
        Ok(image::open(path).expect("failed to load image"))
    }

    fn save_ebidence_file(&self, filename: String, image: image::DynamicImage) -> AppResult<()> {
        FileSystemImpl::create_ebidence_dir();
        let path = format!("./{}/{}", EBIDENCE_DIR, filename);
        match image.save(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(SaveFileError(format!("{:?}", e))),
        }
    }
}
