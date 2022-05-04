use stellar_contract_env::{Env, Object};

pub trait ObjectType<E: Env> {
    fn is_obj_type(obj: Object) -> bool;
    unsafe fn unchecked_from_obj(obj: Object) -> Self;
}
