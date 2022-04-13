use crate::Status;

impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (ty, code) = self.decompose_status();
        f.debug_struct("Status")
            .field("type", &ty)
            .field("code", &code)
            .finish()
    }
}
