use core::fmt::Debug;

use crate::{Object, Status, Symbol, Val};

impl Debug for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_u63() {
            f.debug_struct("Val").field("u63", &self.as_u63()).finish()
        } else if self.is_u32() {
            f.debug_struct("Val").field("u32", &self.as_u32()).finish()
        } else if self.is_i32() {
            f.debug_struct("Val").field("i32", &self.as_i32()).finish()
        } else if self.is_bool() {
            f.debug_struct("Val")
                .field("bool", &self.as_bool())
                .finish()
        } else if self.is_symbol() {
            f.debug_struct("Val")
                .field("symbol", &self.as_symbol())
                .finish()
        } else if self.is_object() {
            f.debug_struct("Val")
                .field("object", &self.as_object())
                .finish()
        } else {
            f.debug_struct("Val")
                .field("body", &self.get_body())
                .field("tag", &self.get_tag())
                .finish()
        }
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Object")
            .field("type", &self.get_type())
            .field("idx", &self.get_idx())
            .finish()
    }
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Symbol")
            .field("str", &self.into_iter().collect::<String>())
            .finish()
    }
}

impl Debug for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (ty, code) = self.decompose_status();
        f.debug_struct("Status")
            .field("type", &ty)
            .field("code", &code)
            .finish()
    }
}
