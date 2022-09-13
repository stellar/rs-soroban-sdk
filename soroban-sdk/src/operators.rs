/// Implement an operator for a type for all the ref variations.
///
/// For operators with a RHS, implements:
///  - ref-op-ref
///  - val-op-ref
///  - ref-op-val
///  - ref-op-mut ref
///  - val-op-mut ref
///  - mut ref-op-ref
///  - mut ref-op-val
///  - mut ref-op-mut ref
///
/// For operators without a RHS, implements:
///  - ref-op
///  - mut ref-op
///
/// Only operators with an Output are supported.
///
/// Assumes that a val-op-val variaton already exists and calls it.
///
/// See [crate::BigInt] for example usage.
macro_rules! impl_ref_op {
    // Special case: PartialEq.
    ($ty:ident, PartialEq<$rhs:ident> :: eq) => {
        impl<'a> PartialEq<$rhs> for &'a mut $ty {
            #[inline(always)]
            fn eq(&self, other: &$rhs) -> bool {
                (*self).clone().eq(other)
            }
        }
        impl<'a> PartialEq<&$rhs> for $ty {
            #[inline(always)]
            fn eq(&self, other: &&$rhs) -> bool {
                (*self).clone().eq(*other)
            }
        }
    };
    // Special case: PartialOrd.
    ($ty:ident, PartialOrd<$rhs:ident> :: eq) => {
        impl<'a> PartialOrd<$rhs> for &'a mut $ty {
            #[inline(always)]
            fn partial_cmp(&self, other: &$rhs) -> Option<Ordering> {
                (*self).clone().partial_cmp(other)
            }
        }
        impl<'a> PartialOrd<&$rhs> for &'a mut $ty {
            #[inline(always)]
            fn partial_cmp(&self, other: &&$rhs) -> Option<Ordering> {
                (*self).clone().partial_cmp(*other)
            }
        }
    };
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

        impl<'a> $op<&'a $rhs> for &'a mut $ty {
            type Output = $ty;
            #[inline(always)]
            fn $op_fn(self, rhs: &'a $rhs) -> Self::Output {
                self.clone().$op_fn(rhs.clone())
            }
        }

        impl<'a> $op<$rhs> for &'a mut $ty {
            type Output = $ty;
            #[inline(always)]
            fn $op_fn(self, rhs: $rhs) -> Self::Output {
                self.clone().$op_fn(rhs)
            }
        }

        impl<'a> $op<&'a mut $rhs> for &'a $ty {
            type Output = $ty;
            #[inline(always)]
            fn $op_fn(self, rhs: &'a mut $rhs) -> Self::Output {
                self.clone().$op_fn(rhs.clone())
            }
        }

        impl<'a> $op<&'a mut $rhs> for $ty {
            type Output = $ty;
            #[inline(always)]
            fn $op_fn(self, rhs: &'a mut $rhs) -> Self::Output {
                self.$op_fn(rhs.clone())
            }
        }

        impl<'a> $op<&'a mut $rhs> for &'a mut $ty {
            type Output = $ty;
            #[inline(always)]
            fn $op_fn(self, rhs: &'a mut $rhs) -> Self::Output {
                self.clone().$op_fn(rhs.clone())
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

        impl<'a> $op for &'a mut $ty {
            type Output = $ty;
            #[inline(always)]
            fn $op_fn(self) -> Self::Output {
                self.clone().$op_fn()
            }
        }
    };
}

pub(crate) use impl_ref_op;

/// Implement an assign operator for a type for all the ref variations.
///
/// Implements:
///  - ref-op-mut ref
///  - val-op-mut ref
///  - ref-op-val
///  - mut ref-op-mut ref
///  - mut ref-op-val
///
/// Only operators with a RHS, &mut self and no Output are supported.
///
/// Assumes that a val-op-val variaton already exists and calls it.
///
/// See [crate::BigInt] for example usage.
macro_rules! impl_ref_assign_op {
    ($ty:ident, $op:ident<$rhs:ident> :: $op_fn:ident) => {
        impl<'a> $op<&'a $rhs> for &'a mut $ty {
            #[inline(always)]
            fn $op_fn(&mut self, rhs: &'a $rhs) {
                (*self).$op_fn(rhs.clone());
            }
        }

        impl<'a> $op<$rhs> for &'a mut $ty {
            #[inline(always)]
            fn $op_fn(&mut self, rhs: $rhs) {
                (*self).$op_fn(rhs);
            }
        }

        impl<'a> $op<&'a $rhs> for $ty {
            #[inline(always)]
            fn $op_fn(&mut self, rhs: &'a $rhs) {
                (*self).$op_fn(rhs.clone());
            }
        }

        impl<'a> $op<&'a mut $rhs> for $ty {
            #[inline(always)]
            fn $op_fn(&mut self, rhs: &'a mut $rhs) {
                (*self).$op_fn(rhs.clone());
            }
        }
    };
}

pub(crate) use impl_ref_assign_op;
