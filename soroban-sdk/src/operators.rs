/// Implement an operator for a type for all the ref variations.
///
/// Implements:
///  - ref-op-ref
///  - val-op-ref
///  - ref-op-val
///
/// Assumes that a ref-op-ref variaton already exists.
///
/// See [crate::BigInt] for example usage.
macro_rules! impl_ref_op {
    ($ty:ident, $op:ident<$rhs:ident> :: $op_fn:ident) => {
        impl<'a> $op<&'a $rhs> for &'a $ty {
            type Output = $ty;
            #[inline(always)]
            fn $op_fn(self, rhs: &'a $rhs) -> Self::Output {
                self.clone().$op_fn(rhs.clone())
            }
        }

        impl<'a> $op<$rhs> for &'a $ty {
            type Output = $ty;
            #[inline(always)]
            fn $op_fn(self, rhs: $rhs) -> Self::Output {
                self.clone().$op_fn(rhs)
            }
        }

        impl<'a> $op<&'a $rhs> for $ty {
            type Output = $ty;
            #[inline(always)]
            fn $op_fn(self, rhs: &'a $rhs) -> Self::Output {
                self.$op_fn(rhs.clone())
            }
        }
    };
    ($ty:ident, $op:ident :: $op_fn:ident) => {
        impl<'a> $op for &'a $ty {
            type Output = $ty;
            #[inline(always)]
            fn $op_fn(self) -> Self::Output {
                self.clone().$op_fn()
            }
        }
    };
}

pub(crate) use impl_ref_op;
