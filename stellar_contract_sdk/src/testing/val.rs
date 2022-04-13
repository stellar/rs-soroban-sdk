use crate::Val;
use core::fmt::Debug;

impl Debug for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Val")
            .field("body", &self.get_body())
            .field("tag", &self.get_tag())
            .finish()
    }
}
