/// Implement an operator for a type for all the ref variations.
///
/// For operators with a RHS, implements:
///  - ref-op-ref
///  - val-op-ref
///  - ref-op-val
///
/// For operators without a RHS, implements:
///  - ref-op
///
/// Only operators with an Output are supported.
///
/// Assumes that a val-op-val variaton already exists and calls it.
///
/// See [crate::BigInt] for example usage.
macro_rules! impl_ref_op {
    // Operators with a RHS.
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
    // Operators without a RHS.
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
