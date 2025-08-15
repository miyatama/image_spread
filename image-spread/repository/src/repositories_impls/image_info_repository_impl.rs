use crate::repositories::ImageInfoRepository;
use util::AppResult;
use util::ImageInfo;

pub struct ImageInfoRepositoryImpl<'r, T: ImageInfoApiClient> {
    todo_api_client: &'r T,
}

impl<'r, T: ImageInfoApiClient> ImageInfoRepositoryImpl<'r, T> {
    pub fn new(todo_api_client: &'r T) -> Self {
        Self {
            todo_api_client: todo_api_client,
        }
    }
}

impl<'r, T: ImageInfoApiClient> ImageInfoRepository for ImageInfoRepositoryImpl<'r, T> {
    fn parse(&self, path: String, grid_width: u32) -> AppResult<ImageInfo> {
        self.todo_api_client.create(ImageInfo { id: 0, text: text })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use util::ImageInfo;

    #[test]
    fn test_parse() {
        assert_eq!(true, false);
    }
}
