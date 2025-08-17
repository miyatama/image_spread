use crate::entities::parse_image_usecase_param::ParseImageUseCaseParam;
use crate::usecases::parse_image_usecase::ParseImageUseCase;
use repository::ImageInfoRepository;
use util::AppResult;
use util::ImageInfo;

pub struct ParseImageUseCaseImpl<'r, R: ImageInfoRepository> {
    image_info_repository: &'r R,
}

impl<'r, R: ImageInfoRepository> ParseImageUseCaseImpl<'r, R> {
    pub fn new(image_info_repository: &'r R) -> Self {
        Self {
            image_info_repository: image_info_repository,
        }
    }
}

impl<'r, R: ImageInfoRepository> ParseImageUseCase for ParseImageUseCaseImpl<'r, R> {
    #[tracing::instrument(skip(self))]
    fn run(&self, param: ParseImageUseCaseParam) -> AppResult<ImageInfo> {
        self.image_info_repository
            .parse(param.path, param.grid_width)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use repository::MockImageInfoRepository as ImageInfoRepository;

    #[tokio::test]
    async fn parse_image_usecase_success() {
        assert_eq!(true, false);
    }
}
