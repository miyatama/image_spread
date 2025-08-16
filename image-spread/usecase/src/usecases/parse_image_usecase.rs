use util::AppResult;
use util::ImageInfo;
use crate::entities::parse_image_usecase_param::ParseImageUseCaseParam;

//TODO Usecaseのテスト組み込み
/*
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
*/
pub trait ParseImageUseCase {
    fn run(&self, param: ParseImageUseCaseParam) -> AppResult<Vec<ImageInfo>>;
}
