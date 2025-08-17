use crate::RepositoryHandler;
use infra_handler::InfraHandler;
use repository::ImageInfoRepositoryImpl;

pub struct RepositoryHandlerImpl<'d, D: InfraHandler> {
    image_info_repository: ImageInfoRepositoryImpl<'d, D::FileSystemAccessor>,
}

impl<'d, D: InfraHandler> RepositoryHandlerImpl<'d, D> {
    pub fn new(handler: &'d D) -> Self {
        let image_info_repository = ImageInfoRepositoryImpl::new(handler.file_system());
        Self {
            image_info_repository: image_info_repository,
        }
    }
}

impl<'d, D: InfraHandler> RepositoryHandler for RepositoryHandlerImpl<'d, D> {
    type ImageInfo = ImageInfoRepositoryImpl<'d, D::FileSystemAccessor>;
    fn image_info_repository(&self) -> &Self::ImageInfo {
        &self.image_info_repository
    }
}
