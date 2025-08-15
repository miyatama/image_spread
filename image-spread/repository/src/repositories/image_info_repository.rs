use util::AppResult;
use util::ImageInfo;

#[mockall::automock]
pub trait ImageInfoRepository {
    fn parse(&self, path: String, grid_width: u32) -> AppResult<ImageInfo>;
}
