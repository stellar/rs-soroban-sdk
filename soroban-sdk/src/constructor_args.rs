use crate::{Env, IntoVal, Val, Vec};

pub trait ConstructorArgs: IntoVal<Env, Vec<Val>> {}

impl<T> ConstructorArgs for Vec<T> {}

macro_rules! impl_constructor_args_for_tuple {
    ( $($typ:ident $idx:tt)* ) => {
        impl<$($typ),*> ConstructorArgs for ($($typ,)*)
        where
            $($typ: IntoVal<Env, Val>),*
        {
        }
    };
}

// 0 topics
impl ConstructorArgs for () {}
// 1-13 topics
impl_constructor_args_for_tuple! { T0 0 }
impl_constructor_args_for_tuple! { T0 0 T1 1 }
impl_constructor_args_for_tuple! { T0 0 T1 1 T2 2 }
impl_constructor_args_for_tuple! { T0 0 T1 1 T2 2 T3 3 }
impl_constructor_args_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 }
impl_constructor_args_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 }
impl_constructor_args_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 }
impl_constructor_args_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 }
impl_constructor_args_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 }
impl_constructor_args_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 }
impl_constructor_args_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 }
impl_constructor_args_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 T11 11 }
impl_constructor_args_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 T11 11 T12 12 }
