use infra::FileSystem;

pub trait InfraHandler {
    type FileSystemAccessor: FileSystem;
    fn file_system(&self) -> &Self::FileSystemAccessor;
}