use crate::InfraHandler;
use infra_handler::InfraHandler;
use infra::FileSystemImpl;

pub struct InfraHandlerImpl<'d, D: InfraHandler> {
    file_system: FileSystemImpl,
}

impl<'d, D: InfraHandler> InfraHandlerImpl<'d, D> {
    pub fn new(handler: &'d D) -> Self {
        let file_system= FileSystemImpl::new();
        Self {
            file_system_accessor: file_system,
        }
    }
}

impl<'d, D: InfraHandler> InfraHandler for InfraHandlerImpl<'d, D> {
    type FileSystemAccessor = FileSystemImpl;
    fn file_system(&self) -> &Self::FileSystemAccessor {
        &self.file_system_accessor
    }
}
