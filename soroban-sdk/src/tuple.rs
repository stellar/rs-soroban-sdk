//! This module contains conversion helpers for tuple

use crate::{vec, ConversionError, Env, IntoVal, Topics, TryFromVal, Val, Vec};

impl TryFromVal<Env, ()> for Vec<Val> {
    type Error = ConversionError;

    fn try_from_val(env: &Env, _v: &()) -> Result<Self, Self::Error> {
        Ok(Vec::<Val>::new(env))
    }
}

macro_rules! impl_into_vec_for_tuple {
    ( $($typ:ident $idx:tt)* ) => {
        impl<$($typ),*> TryFromVal<Env, ($($typ,)*)> for Vec<Val>
        where
            $($typ: IntoVal<Env, Val>),*
        {
            type Error = ConversionError;
            fn try_from_val(env: &Env, v: &($($typ,)*)) -> Result<Self, Self::Error> {
                Ok(vec![&env, $(v.$idx.into_val(env), )*])
            }
        }
    };
}
impl_into_vec_for_tuple! { T0 0 }
impl_into_vec_for_tuple! { T0 0 T1 1 }
impl_into_vec_for_tuple! { T0 0 T1 1 T2 2 }
impl_into_vec_for_tuple! { T0 0 T1 1 T2 2 T3 3 }
impl_into_vec_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 }
impl_into_vec_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 }
impl_into_vec_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 }
impl_into_vec_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 }
impl_into_vec_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 }
impl_into_vec_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 }
impl_into_vec_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 }
impl_into_vec_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 T11 11 }
impl_into_vec_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 T11 11 T12 12 }

macro_rules! impl_topics_for_tuple {
    ( $($typ:ident $idx:tt)* ) => {
        impl<$($typ),*> Topics for ($($typ,)*)
        where
            $($typ: IntoVal<Env, Val>),*
        {
        }
    };
}

// 0 topics
impl Topics for () {}
// 1-13 topics
impl_topics_for_tuple! { T0 0 }
impl_topics_for_tuple! { T0 0 T1 1 }
impl_topics_for_tuple! { T0 0 T1 1 T2 2 }
impl_topics_for_tuple! { T0 0 T1 1 T2 2 T3 3 }
impl_topics_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 }
impl_topics_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 }
impl_topics_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 }
impl_topics_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 }
impl_topics_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 }
impl_topics_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 }
impl_topics_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 }
impl_topics_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 T11 11 }
impl_topics_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 T11 11 T12 12 }
