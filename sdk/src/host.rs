use stellar_contract_host::Val;

use crate::host_fns;

pub struct Host(());

impl stellar_contract_host::Host for Host {
    fn as_mut_any(&mut self) -> &mut dyn core::any::Any {
        todo!()
    }

    fn log_value(&mut self, v: Val) -> Val {
        unsafe { host_fns::context::log_value(v) }
    }

    fn get_last_operation_result(&mut self) -> stellar_contract_host::Object {
        todo!()
    }

    fn obj_from_u64(&mut self, u: u64) -> stellar_contract_host::Object {
        todo!()
    }

    fn obj_to_u64(&mut self, u: stellar_contract_host::Object) -> u64 {
        todo!()
    }

    fn obj_from_i64(&mut self, i: i64) -> stellar_contract_host::Object {
        todo!()
    }

    fn obj_to_i64(&mut self, i: stellar_contract_host::Object) -> i64 {
        todo!()
    }

    fn map_new(&mut self) -> stellar_contract_host::Object {
        todo!()
    }

    fn map_put(
        &mut self,
        m: stellar_contract_host::Object,
        k: Val,
        v: Val,
    ) -> stellar_contract_host::Object {
        todo!()
    }

    fn map_get(
        &mut self,
        m: stellar_contract_host::Object,
        k: Val,
    ) -> Val {
        todo!()
    }

    fn map_del(
        &mut self,
        m: stellar_contract_host::Object,
        k: Val,
    ) -> stellar_contract_host::Object {
        todo!()
    }

    fn map_len(&mut self, m: stellar_contract_host::Object) -> Val {
        todo!()
    }

    fn map_keys(&mut self, m: stellar_contract_host::Object) -> stellar_contract_host::Object {
        todo!()
    }

    fn map_has(
        &mut self,
        m: stellar_contract_host::Object,
        k: Val,
    ) -> Val {
        todo!()
    }

    fn vec_new(&mut self) -> stellar_contract_host::Object {
        todo!()
    }

    fn vec_put(
        &mut self,
        v: stellar_contract_host::Object,
        i: Val,
        x: Val,
    ) -> stellar_contract_host::Object {
        todo!()
    }

    fn vec_get(
        &mut self,
        v: stellar_contract_host::Object,
        i: Val,
    ) -> Val {
        todo!()
    }

    fn vec_del(
        &mut self,
        v: stellar_contract_host::Object,
        i: Val,
    ) -> stellar_contract_host::Object {
        todo!()
    }

    fn vec_len(&mut self, v: stellar_contract_host::Object) -> Val {
        todo!()
    }

    fn vec_push(
        &mut self,
        v: stellar_contract_host::Object,
        x: Val,
    ) -> stellar_contract_host::Object {
        todo!()
    }

    fn vec_pop(&mut self, v: stellar_contract_host::Object) -> stellar_contract_host::Object {
        todo!()
    }

    fn vec_take(
        &mut self,
        v: stellar_contract_host::Object,
        n: Val,
    ) -> stellar_contract_host::Object {
        todo!()
    }

    fn vec_drop(
        &mut self,
        v: stellar_contract_host::Object,
        n: Val,
    ) -> stellar_contract_host::Object {
        todo!()
    }

    fn vec_front(&mut self, v: stellar_contract_host::Object) -> Val {
        todo!()
    }

    fn vec_back(&mut self, v: stellar_contract_host::Object) -> Val {
        todo!()
    }

    fn vec_insert(
        &mut self,
        v: stellar_contract_host::Object,
        i: Val,
        n: Val,
    ) -> stellar_contract_host::Object {
        todo!()
    }

    fn vec_append(
        &mut self,
        v1: stellar_contract_host::Object,
        v2: stellar_contract_host::Object,
    ) -> stellar_contract_host::Object {
        todo!()
    }

    fn pay(
        &mut self,
        src: stellar_contract_host::Object,
        dst: stellar_contract_host::Object,
        asset: stellar_contract_host::Object,
        amount: Val,
    ) -> Val {
        todo!()
    }

    fn account_balance(
        &mut self,
        acc: stellar_contract_host::Object,
    ) -> Val {
        todo!()
    }

    fn account_trust_line(
        &mut self,
        acc: stellar_contract_host::Object,
        asset: stellar_contract_host::Object,
    ) -> stellar_contract_host::Object {
        todo!()
    }

    fn trust_line_balance(
        &mut self,
        tl: stellar_contract_host::Object,
    ) -> Val {
        todo!()
    }

    fn get_contract_data(&mut self, k: Val) -> Val {
        todo!()
    }

    fn put_contract_data(
        &mut self,
        k: Val,
        v: Val,
    ) -> Val {
        todo!()
    }

    fn has_contract_data(&mut self, k: Val) -> Val {
        todo!()
    }
}
