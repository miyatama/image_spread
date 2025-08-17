use crate::UsecaseHandler;
use repository_handler::RepositoryHandler;
use usecase::{
    ParseImageUseCaseImpl,
};

pub struct UsecaseHandlerImpl<'r, R: RepositoryHandler> {
    parse_image_usecase: ParseImageUseCaseImpl<'r, R::ImageInfo>,
}

impl<'r, R: RepositoryHandler> UsecaseHandlerImpl<'r, R> {
    pub async fn new(handler: &'r R) -> Self {
        let parse_image_usecase = ParseImageUseCaseImpl::new(handler.image_info_repository());
        Self {
            parse_image_usecase: parse_image_usecase,
        }
    }
}

impl<'r, R: RepositoryHandler> UsecaseHandler for UsecaseHandlerImpl<'r, R> {
    type ParseImage = ParseImageUseCaseImpl<'r, R::ImageInfo>;

    fn parse_image(&self) -> &Self::ParseImage {
        &self.parse_image_usecase
    }
}
