use repository::ImageInfoRepository;

pub trait RepositoryHandler {
    type ImageInfo: ImageInfoRepository;
    fn image_info_repository(&self) -> &Self::ImageInfo;
}
