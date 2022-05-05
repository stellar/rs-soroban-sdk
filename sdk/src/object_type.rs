use super::Object;

pub trait ObjectType {
    fn is_obj_type(obj: Object) -> bool;
    unsafe fn unchecked_from_obj(obj: Object) -> Self;
}
