use util::AppResult;
use util::ImageInfo;

//TODO Usecaseのテスト組み込み
/*
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
*/
pub trait ParseImageUseCase {
    fn run(&self) -> AppResult<Vec<ImageInfo>>;
}
