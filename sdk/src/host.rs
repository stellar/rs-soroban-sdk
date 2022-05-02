use crate::{host_fns, OrAbort};
use core::any::Any;
use stellar_contract_host::{Object, Val, HostConvertable};

#[derive(Default)]
pub struct Host;

impl stellar_contract_host::Host for Host {
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn log_value(&mut self, v: Val) -> Val {
        unsafe { host_fns::context::log_value(v) }
    }

    fn get_last_operation_result(&mut self) -> Object {
        unsafe { host_fns::context::get_last_operation_result() }
    }

    fn obj_from_u64(&mut self, u: u64) -> Object {
        unsafe { host_fns::u64::from_u64(u) }
    }

    fn obj_to_u64(&mut self, u: Object) -> u64 {
        unsafe { host_fns::u64::to_u64(u) }
    }

    fn obj_from_i64(&mut self, i: i64) -> Object {
        unsafe { host_fns::i64::from_i64(u) }
    }

    fn obj_to_i64(&mut self, i: Object) -> i64 {
        unsafe { host_fns::i64::to_i64(i) }
    }

    fn map_new(&mut self) -> Object {
        unsafe { host_fns::map::new() }
    }

    fn map_put(&mut self, m: Object, k: Val, v: Val) -> Object {
        unsafe { host_fns::map::put(m, k, v) }
    }

    fn map_get(&mut self, m: Object, k: Val) -> Val {
        unsafe { host_fns::map::get(m, k) }
    }

    fn map_del(&mut self, m: Object, k: Val) -> Object {
        unsafe { host_fns::map::del(m, k) }
    }

    fn map_len(&mut self, m: Object) -> Val {
        unsafe { host_fns::map::len(m) }
    }

    fn map_keys(&mut self, m: Object) -> Object {
        unsafe { host_fns::map::keys(m) }
    }

    fn map_has(&mut self, m: Object, k: Val) -> Val {
        unsafe { host_fns::map::has(m, k) }
    }

    fn vec_new(&mut self) -> Object {
        unsafe { host_fns::vec::new() }
    }

    fn vec_put(&mut self, v: Object, i: Val, x: Val) -> Object {
        unsafe { host_fns::vec::put(v, i, x) }
    }

    fn vec_get(&mut self, v: Object, i: Val) -> Val {
        unsafe { host_fns::vec::get(v, i) }
    }

    fn vec_del(&mut self, v: Object, i: Val) -> Object {
        unsafe { host_fns::vec::del(v, i) }
    }

    fn vec_len(&mut self, v: Object) -> Val {
        unsafe { host_fns::vec::len(v) }
    }

    fn vec_push(&mut self, v: Object, x: Val) -> Object {
        unsafe { host_fns::vec::push(v, x) }
    }

    fn vec_pop(&mut self, v: Object) -> Object {
        unsafe { host_fns::vec::pop(v) }
    }

    fn vec_take(&mut self, v: Object, n: Val) -> Object {
        unsafe { host_fns::vec::take(v, n) }
    }

    fn vec_drop(&mut self, v: Object, n: Val) -> Object {
        unsafe { host_fns::vec::drop(v, n) }
    }

    fn vec_front(&mut self, v: Object) -> Val {
        unsafe { host_fns::vec::front(v) }
    }

    fn vec_back(&mut self, v: Object) -> Val {
        unsafe { host_fns::vec::back(v) }
    }

    fn vec_insert(&mut self, v: Object, i: Val, n: Val) -> Object {
        unsafe { host_fns::vec::insert(v, i, n) }
    }

    fn vec_append(&mut self, v1: Object, v2: Object) -> Object {
        unsafe { host_fns::vec::append(v1, v2) }
    }

    fn pay(&mut self, src: Object, dst: Object, asset: Object, amount: Val) -> Val {
        unsafe { host_fns::ledger::pay(src, dst, asset, amount) }
    }

    fn account_balance(&mut self, acc: Object) -> Val {
        unsafe { host_fns::ledger::account_balance(acc) }
    }

    fn account_trust_line(&mut self, acc: Object, asset: Object) -> Object {
        unsafe { host_fns::ledger::account_trust_line(acc, asset) }
    }

    fn trust_line_balance(&mut self, tl: Object) -> Val {
        unsafe { host_fns::ledger::trust_line_balance(tl) }
    }

    fn get_contract_data(&mut self, k: Val) -> Val {
        unsafe { host_fns::ledger::get_contract_data(k) }
    }

    fn put_contract_data(&mut self, k: Val, v: Val) -> Val {
        unsafe { host_fns::ledger::put_contract_data(k, v) }
    }

    fn has_contract_data(&mut self, k: Val) -> Val {
        unsafe { host_fns::ledger::has_contract_data(k) }
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
