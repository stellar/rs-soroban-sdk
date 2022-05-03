use crate::OrAbort;
use core::any::Any;
use std::rc::Rc;
use stellar_contract_host::{HostConvertable, Object, Val};

// TODO: Make stellar_contract_host::Host object safe so it can be used here?
pub struct Host(pub Rc<stellar_contract_host::HostContext>);

impl Host {
    fn new() {
        Host
    }
}

impl stellar_contract_host::Host for Host {
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn log_value(&mut self, v: Val) -> Val {
        self.0.log_value(v)
    }

    fn get_last_operation_result(&mut self) -> Object {
        self.0.get_last_operation_result()
    }

    fn obj_from_u64(&mut self, u: u64) -> Object {
        self.0.obj_from_u64(u)
    }

    fn obj_to_u64(&mut self, u: Object) -> u64 {
        self.0.obj_to_u64(u)
    }

    fn obj_from_i64(&mut self, i: i64) -> Object {
        self.0.obj_from_i64(i)
    }

    fn obj_to_i64(&mut self, i: Object) -> i64 {
        self.0.obj_to_i64(i)
    }

    fn map_new(&mut self) -> Object {
        self.0.map_new()
    }

    fn map_put(&mut self, m: Object, k: Val, v: Val) -> Object {
        self.0.map_put(m, k, v)
    }

    fn map_get(&mut self, m: Object, k: Val) -> Val {
        self.0.map_get(m, k)
    }

    fn map_del(&mut self, m: Object, k: Val) -> Object {
        self.0.map_del(m, k)
    }

    fn map_len(&mut self, m: Object) -> Val {
        self.0.map_len(m)
    }

    fn map_keys(&mut self, m: Object) -> Object {
        self.0.map_keys(m)
    }

    fn map_has(&mut self, m: Object, k: Val) -> Val {
        self.0.map_has(m, k)
    }

    fn vec_new(&mut self) -> Object {
        self.0.vec_new()
    }

    fn vec_put(&mut self, v: Object, i: Val, x: Val) -> Object {
        self.0.vec_put(v, i, x)
    }

    fn vec_get(&mut self, v: Object, i: Val) -> Val {
        self.0.vec_get(v, i)
    }

    fn vec_del(&mut self, v: Object, i: Val) -> Object {
        self.0.vec_del(v, i)
    }

    fn vec_len(&mut self, v: Object) -> Val {
        self.0.vec_len(v)
    }

    fn vec_push(&mut self, v: Object, x: Val) -> Object {
        self.0.vec_push(v, x)
    }

    fn vec_pop(&mut self, v: Object) -> Object {
        self.0.vec_pop(v)
    }

    fn vec_take(&mut self, v: Object, n: Val) -> Object {
        self.0.vec_take(v, n)
    }

    fn vec_drop(&mut self, v: Object, n: Val) -> Object {
        self.0.vec_drop(v, n)
    }

    fn vec_front(&mut self, v: Object) -> Val {
        self.0.vec_front(v)
    }

    fn vec_back(&mut self, v: Object) -> Val {
        self.0.vec_back(v)
    }

    fn vec_insert(&mut self, v: Object, i: Val, n: Val) -> Object {
        self.0.vec_insert(v, i, n)
    }

    fn vec_append(&mut self, v1: Object, v2: Object) -> Object {
        self.0.vec_append(v1, v2)
    }

    fn pay(&mut self, src: Object, dst: Object, asset: Object, amount: Val) -> Val {
        self.0.pay(src, dst, asset, amount)
    }

    fn account_balance(&mut self, acc: Object) -> Val {
        self.0.account_balance(acc)
    }

    fn account_trust_line(&mut self, acc: Object, asset: Object) -> Object {
        self.0.account_trust_line(acc, asset)
    }

    fn trust_line_balance(&mut self, tl: Object) -> Val {
        self.0.trust_line_balance(tl)
    }

    fn get_contract_data(&mut self, k: Val) -> Val {
        self.0.get_contract_data(k)
    }

    fn put_contract_data(&mut self, k: Val, v: Val) -> Val {
        self.0.put_contract_data(k, v)
    }

    fn has_contract_data(&mut self, k: Val) -> Val {
        self.0.has_contract_data(k)
    }

    fn val_from<HC: HostConvertable>(&mut self, v: HC) -> Val {
        v.val_from(self)
    }

    fn try_val_into<HC: HostConvertable>(&mut self, v: Val) -> Option<HC> {
        HC::try_val_into(v, self)
    }

    fn val_into<HC: HostConvertable>(&mut self, v: Val) -> HC {
        self.try_val_into::<HC>(v).or_abort()
    }
}
