use crate::UsecaseHandler;
use repository_handler::RepositoryHandler;
use usecase::{
    ParseImageUseCaseImpl,
};

pub struct UsecaseHandlerImpl<'r, R: RepositoryHandler> {
    parse_image_useecase: ParseImageUseCaseImpl<'r, R::Todo>,
}

impl<'r, R: RepositoryHandler> UsecaseHandlerImpl<'r, R> {
    pub async fn new(handler: &'r R) -> Self {
        let parse_image_useecase = ParseImageUseCaseImpl::new(handler.todo_repository());
        Self {
            parse_image_useecase: parse_image_useecase,
        }
    }
}

impl<'r, R: RepositoryHandler> UsecaseHandler for UsecaseHandlerImpl<'r, R> {
    type ParseImage = ParseImageUseCaseImpl<'r, R::Todo>;

    fn parse_image(&self) -> &Self::ParseImage {
        &self.parse_image_useecase
    }
}
