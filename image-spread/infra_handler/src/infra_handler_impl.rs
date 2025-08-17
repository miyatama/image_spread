use crate::infra_handler::InfraHandler;
use infra::FileSystemImpl;

pub struct InfraHandlerImpl {
    file_system_accessor: FileSystemImpl,
}

impl InfraHandlerImpl {
    pub fn new() -> Self {
        let file_system= FileSystemImpl::new();
        Self {
            file_system_accessor: file_system,
        }
    }
}

impl InfraHandler for InfraHandlerImpl {
    type FileSystemAccessor = FileSystemImpl;
    fn file_system(&self) -> &Self::FileSystemAccessor {
        &self.file_system_accessor
    }
}
