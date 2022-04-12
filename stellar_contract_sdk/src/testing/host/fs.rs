use super::MockHost;
use crate::{Object, Val};

pub struct FsHost {}

impl MockHost for FsHost {
    fn log_value(&mut self, v: Val) -> Val {
        todo!()
    }

    fn get_last_operation_result(&mut self) -> Object {
        todo!()
    }

    fn map_new(&mut self) -> Object {
        todo!()
    }

    fn map_put(&mut self, m: Object, k: Val, v: Val) -> Object {
        todo!()
    }

    fn map_get(&mut self, m: Object, k: Val) -> Val {
        todo!()
    }

    fn map_del(&mut self, m: Object, k: Val) -> Object {
        todo!()
    }

    fn map_len(&mut self, m: Object) -> Val {
        todo!()
    }

    fn map_keys(&mut self, m: Object) -> Object {
        todo!()
    }

    fn map_has(&mut self, m: Object, k: Val) -> Val {
        todo!()
    }

    fn get_contract_data(&mut self, k: Val) -> Val {
        todo!()
    }

    fn put_contract_data(&mut self, k: Val, v: Val) -> Val {
        todo!()
    }

    fn has_contract_data(&mut self, k: Val) -> Val {
        todo!()
    }

    fn vec_new(&mut self) -> Object {
        todo!()
    }

    fn vec_put(&mut self, v: Object, i: Val, x: Val) -> Object {
        todo!()
    }

    fn vec_get(&mut self, v: Object, i: Val) -> Val {
        todo!()
    }

    fn vec_del(&mut self, v: Object, i: Val) -> Object {
        todo!()
    }

    fn vec_len(&mut self, v: Object) -> Val {
        todo!()
    }

    fn vec_push(&mut self, v: Object, x: Val) -> Object {
        todo!()
    }

    fn vec_pop(&mut self, v: Object) -> Object {
        todo!()
    }

    fn vec_take(&mut self, v: Object, n: Val) -> Object {
        todo!()
    }

    fn vec_drop(&mut self, v: Object, n: Val) -> Object {
        todo!()
    }

    fn vec_front(&mut self, v: Object) -> Val {
        todo!()
    }

    fn vec_back(&mut self, v: Object) -> Val {
        todo!()
    }

    fn vec_insert(&mut self, v: Object, i: Val, n: Val) -> Object {
        todo!()
    }

    fn vec_append(&mut self, v1: Object, v2: Object) -> Object {
        todo!()
    }
}

impl FsHost {
    pub fn new() -> Self {
        Self {}
    }
}
