use crate::usecases::parse_image_usecase::ParseImageUseCase;
use repository::ImageInfoRepository;
use util::AppResult;
use util::ImageInfo;

pub struct ParseImageUseCaseImpl<'r, R: ImageInfoRepository> {
    todo_repository: &'r R,
}

impl<'r, R: ImageInfoRepository> ParseImageUseCaseImpl<'r, R> {
    pub fn new(todo_repository: &'r R) -> Self {
        Self {
            todo_repository: todo_repository,
        }
    }
}

impl<'r, R: ImageInfoRepository> ParseImageUseCase for ParseImageUseCaseImpl<'r, R> {
    #[tracing::instrument(skip(self))]
    fn run(&self) -> AppResult<Vec<ImageInfo>> {
        self.todo_repository.list()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use repository::MockImageInfoRepository as ImageInfoRepository;

    #[tokio::test]
    async fn get_todo_list_usecase_success() {
        let expect_list = vec![
            ImageInfo {
                id: 1,
                text: "test01".to_string(),
            },
            ImageInfo {
                id: 2,
                text: "test02".to_string(),
            },
        ];
        let mock_result = Ok(expect_list.clone());
        let mut mock_todo_repository = ImageInfoRepository::new();
        mock_todo_repository
            .expect_list()
            .times(1)
            .return_once_st(move || mock_result);
        let usecase = ParseImageUseCaseImpl::new(&mock_todo_repository);
        let result = usecase.run();
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expect_list);
    }
}
