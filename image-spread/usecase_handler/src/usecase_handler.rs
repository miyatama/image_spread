use usecase::ParseImageUseCase;
pub trait UsecaseHandler {
    type ParseImage: ParseImageUseCase;
    fn parse_image(&self) -> &Self::ParseImage;
}
