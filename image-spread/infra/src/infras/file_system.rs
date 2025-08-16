/**
 * 本システムで利用するファイルアクセスを提供する
 */
use util::AppResult;

#[cfg(feature = "mock")]
use mockall::automock;

#[cfg_attr(feature = "mock", automock)]
pub trait FileSystem {
    fn open_image_file(&self, path: String) -> AppResult<image::DynamicImage>;
}
