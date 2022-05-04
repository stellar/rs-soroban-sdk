#[derive(Clone)]
pub struct Ctx(
    #[cfg(target_family = "wasm")] stellar_contract_env::Guest,
    #[cfg(not(target_family = "wasm"))] stellar_contract_env::WeakHost,
);

impl stellar_contract_env::Env for Ctx {
    fn as_mut_any(&mut self) -> &mut dyn core::any::Any {
        self
    }

    fn check_same_env(&self, other: &Self) {
        self.0.check_same_env(&self.0)
    }

    fn obj_cmp(&self, a: stellar_contract_env::Val, b: stellar_contract_env::Val) -> i64 {
        self.0.obj_cmp(a, b)
    }

    fn log_value(&mut self, v: stellar_contract_env::Val) -> stellar_contract_env::Val {
        self.0.log_value(v)
    }

    fn get_last_operation_result(&mut self) -> stellar_contract_env::Val {
        self.0.get_last_operation_result()
    }

    fn obj_from_u64(&mut self, u: u64) -> stellar_contract_env::Val {
        self.0.obj_from_u64(u)
    }

    fn obj_to_u64(&mut self, u: stellar_contract_env::Val) -> u64 {
        self.0.obj_to_u64(u)
    }

    fn obj_from_i64(&mut self, i: i64) -> stellar_contract_env::Val {
        self.0.obj_from_i64(i)
    }

    fn obj_to_i64(&mut self, i: stellar_contract_env::Val) -> i64 {
        self.0.obj_to_i64(i)
    }

    fn map_new(&mut self) -> stellar_contract_env::Val {
        self.0.map_new()
    }

    fn map_put(
        &mut self,
        m: stellar_contract_env::Val,
        k: stellar_contract_env::Val,
        v: stellar_contract_env::Val,
    ) -> stellar_contract_env::Val {
        self.0.map_put(m, k, v)
    }

    fn map_get(
        &mut self,
        m: stellar_contract_env::Val,
        k: stellar_contract_env::Val,
    ) -> stellar_contract_env::Val {
        self.0.map_get(m, k)
    }

    fn map_del(
        &mut self,
        m: stellar_contract_env::Val,
        k: stellar_contract_env::Val,
    ) -> stellar_contract_env::Val {
        self.0.map_del(m, k)
    }

    fn map_len(&mut self, m: stellar_contract_env::Val) -> stellar_contract_env::Val {
        self.0.map_len(m)
    }

    fn map_keys(&mut self, m: stellar_contract_env::Val) -> stellar_contract_env::Val {
        self.0.map_keys(m)
    }

    fn map_has(
        &mut self,
        m: stellar_contract_env::Val,
        k: stellar_contract_env::Val,
    ) -> stellar_contract_env::Val {
        self.0.map_has(m, k)
    }

    fn vec_new(&mut self) -> stellar_contract_env::Val {
        self.0.vec_new()
    }

    fn vec_put(
        &mut self,
        v: stellar_contract_env::Val,
        i: stellar_contract_env::Val,
        x: stellar_contract_env::Val,
    ) -> stellar_contract_env::Val {
        self.0.vec_put(v, i, x)
    }

    fn vec_get(
        &mut self,
        v: stellar_contract_env::Val,
        i: stellar_contract_env::Val,
    ) -> stellar_contract_env::Val {
        self.0.vec_get(v, i)
    }

    fn vec_del(
        &mut self,
        v: stellar_contract_env::Val,
        i: stellar_contract_env::Val,
    ) -> stellar_contract_env::Val {
        self.0.vec_del(v, i)
    }

    fn vec_len(&mut self, v: stellar_contract_env::Val) -> stellar_contract_env::Val {
        self.0.vec_len(v)
    }

    fn vec_push(
        &mut self,
        v: stellar_contract_env::Val,
        x: stellar_contract_env::Val,
    ) -> stellar_contract_env::Val {
        self.0.vec_push(v, x)
    }

    fn vec_pop(&mut self, v: stellar_contract_env::Val) -> stellar_contract_env::Val {
        self.0.vec_pop(v)
    }

    fn vec_take(
        &mut self,
        v: stellar_contract_env::Val,
        n: stellar_contract_env::Val,
    ) -> stellar_contract_env::Val {
        self.0.vec_take(v, n)
    }

    fn vec_drop(
        &mut self,
        v: stellar_contract_env::Val,
        n: stellar_contract_env::Val,
    ) -> stellar_contract_env::Val {
        self.0.vec_drop(v, n)
    }

    fn vec_front(&mut self, v: stellar_contract_env::Val) -> stellar_contract_env::Val {
        self.0.vec_front(v)
    }

    fn vec_back(&mut self, v: stellar_contract_env::Val) -> stellar_contract_env::Val {
        self.0.vec_back(v)
    }

    fn vec_insert(
        &mut self,
        v: stellar_contract_env::Val,
        i: stellar_contract_env::Val,
        n: stellar_contract_env::Val,
    ) -> stellar_contract_env::Val {
        self.0.vec_insert(v, i, n)
    }

    fn vec_append(
        &mut self,
        v1: stellar_contract_env::Val,
        v2: stellar_contract_env::Val,
    ) -> stellar_contract_env::Val {
        self.0.vec_append(v1, v2)
    }

    fn pay(
        &mut self,
        src: stellar_contract_env::Val,
        dst: stellar_contract_env::Val,
        asset: stellar_contract_env::Val,
        amount: stellar_contract_env::Val,
    ) -> stellar_contract_env::Val {
        self.0.pay(src, dst, asset, amount)
    }

    fn account_balance(&mut self, acc: stellar_contract_env::Val) -> stellar_contract_env::Val {
        self.0.account_balance(acc)
    }

    fn account_trust_line(
        &mut self,
        acc: stellar_contract_env::Val,
        asset: stellar_contract_env::Val,
    ) -> stellar_contract_env::Val {
        self.0.account_trust_line(acc, asset)
    }

    fn trust_line_balance(&mut self, tl: stellar_contract_env::Val) -> stellar_contract_env::Val {
        self.0.trust_line_balance(tl)
    }

    fn get_contract_data(&mut self, k: stellar_contract_env::Val) -> stellar_contract_env::Val {
        self.0.get_contract_data(k)
    }

    fn put_contract_data(
        &mut self,
        k: stellar_contract_env::Val,
        v: stellar_contract_env::Val,
    ) -> stellar_contract_env::Val {
        self.0.put_contract_data(k, v)
    }

    fn has_contract_data(&mut self, k: stellar_contract_env::Val) -> stellar_contract_env::Val {
        self.0.has_contract_data(k)
    }
}
