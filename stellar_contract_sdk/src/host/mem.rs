use super::MockHost;

pub struct MemHost {}

impl MockHost for MemHost {
    fn log_value(&self, v: crate::Val) -> crate::Val {
        todo!()
    }

    fn get_last_operation_result(&self) -> crate::Object {
        todo!()
    }
}

impl MemHost {
    pub fn new() -> Self {
        Self {}
    }
}
