use super::MockHost;

pub struct FsHost {}

impl MockHost for FsHost {
    fn log_value(&self, v: crate::Val) -> crate::Val {
        todo!()
    }

    fn get_last_operation_result(&self) -> crate::Object {
        todo!()
    }
}

impl FsHost {
    pub fn new() -> Self {
        Self {}
    }
}
