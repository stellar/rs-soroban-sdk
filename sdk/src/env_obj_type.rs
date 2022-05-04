use crate::EnvObj;
use stellar_contract_env::Env;

pub trait EnvObjType<E: Env> {
    fn is_obj_type(obj: EnvObj<E>) -> bool;
    unsafe fn unchecked_from_obj(obj: EnvObj<E>) -> Self;
}
