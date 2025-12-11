#[cfg(not(target_family = "wasm"))]
use crate::xdr::ScVal;
use crate::{
    crypto::utils::BigInt,
    env::internal::{self, BytesObject, U256Val},
    impl_bytesn_repr,
    unwrap::{UnwrapInfallible, UnwrapOptimized},
    Bytes, BytesN, ConversionError, Env, IntoVal, TryFromVal, Val, Vec, U256,
};
use core::{
    cmp::Ordering,
    fmt::Debug,
    ops::{Add, Mul, Neg},
};

const FP_SERIALIZED_SIZE: usize = 32; // Size in bytes of a serialized Fp element in BN254. The field modulus is 254 bits, requiring 32 bytes (256 bits).
pub const G1_SERIALIZED_SIZE: usize = FP_SERIALIZED_SIZE * 2; // Size in bytes of a serialized G1 element in BN254. Each coordinate (X, Y) is 32 bytes.
pub const G2_SERIALIZED_SIZE: usize = G1_SERIALIZED_SIZE * 2; // Size in bytes of a serialized G2 element in BN254. Each coordinate (X, Y) is 64 bytes (2 Fp elements per coordinate).

/// Bn254 provides access to curve and pairing operations on the BN254
/// (also known as alt_bn128) curve.
pub struct Bn254 {
    env: Env,
}

/// `G1Affine` is a point in the G1 group (subgroup defined over the base field
/// `Fp` with prime order `q =
/// 0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47`) of the
/// BN254 elliptic curve
///
/// # Serialization:
/// - The 64 bytes represent the **uncompressed encoding** of a point in G1. The
///   bytes consist of `be_bytes(X) || be_bytes(Y)` (`||` is concatenation),
///   where 'X' and 'Y' are the two coordinates, each being a base field element
///   `Fp` (32 bytes each).
#[derive(Clone)]
#[repr(transparent)]
pub struct G1Affine(BytesN<G1_SERIALIZED_SIZE>);

/// `G2Affine` is a point in the G2 group (subgroup defined over the quadratic
/// extension field `Fp2`) of the BN254 elliptic curve
///
/// # Serialization:
/// - The 128 bytes represent the **uncompressed encoding** of a point in G2.
///   The bytes consist of `be_bytes(X_im) || be_bytes(X_re) || be_bytes(Y_im)
///   || be_bytes(Y_re)` (`||` is concatenation), where 'X' and 'Y' are the two
///   coordinates, each being an extension field element `Fp2`. Each component
///   (real and imaginary parts) is an `Fp` element (32 bytes each).
#[derive(Clone)]
#[repr(transparent)]
pub struct G2Affine(BytesN<G2_SERIALIZED_SIZE>);

/// `Fr` represents an element in the BN254 scalar field, which is a prime field
/// of order `r =
/// 0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001`. The
/// struct is internally represented with a `U256`, all arithmetic operations
/// follow modulo `r`.
#[derive(Clone)]
#[repr(transparent)]
pub struct Fr(U256);

/// `Fp` represents an element of the base field `Fp` of the BN254 elliptic curve
///
/// # Serialization:
/// - The 32 bytes represent the **big-endian encoding** of an element in the
///   field `Fp`. The value is serialized as a big-endian integer.
#[derive(Clone)]
#[repr(transparent)]
pub struct Fp(BytesN<FP_SERIALIZED_SIZE>);

impl_bytesn_repr!(G1Affine, G1_SERIALIZED_SIZE);
impl_bytesn_repr!(G2Affine, G2_SERIALIZED_SIZE);
impl_bytesn_repr!(Fp, FP_SERIALIZED_SIZE);

impl G1Affine {
    pub fn env(&self) -> &Env {
        self.0.env()
    }
}

impl Fp {
    pub fn env(&self) -> &Env {
        self.0.env()
    }

    // `Fp` represents an element in the base field of the BN254 elliptic curve.
    // For an element a ∈ Fp, its negation `-a` is defined as:
    //   a + (-a) = 0 (mod p)
    // where `p` is the field modulus, and to make a valid point coordinate on the
    // curve, `a` also must be within the field range (i.e., 0 ≤ a < p).
    fn checked_neg(&self) -> Option<Fp> {
        let fq_bigint: BigInt<4> = (&self.0).into();
        if fq_bigint.is_zero() {
            return Some(self.clone());
        }

        //BN254 base field modulus
        const BN254_MODULUS: [u64; 4] = [
            4332616871279656263,
            10917124144477883021,
            13281191951274694749,
            3486998266802970665,
        ];
        let mut res = BigInt(BN254_MODULUS);

        // Compute modulus - value
        let borrow = res.sub_with_borrow(&fq_bigint);
        if borrow {
            return None;
        }

        let mut bytes = [0u8; FP_SERIALIZED_SIZE];
        res.copy_into_array(&mut bytes);
        Some(Fp::from_array(self.env(), &bytes))
    }
}

impl Neg for &Fp {
    type Output = Fp;

    fn neg(self) -> Self::Output {
        match self.checked_neg() {
            Some(v) => v,
            None => sdk_panic!("invalid input - Fp is larger than the field modulus"),
        }
    }
}

impl Neg for Fp {
    type Output = Fp;

    fn neg(self) -> Self::Output {
        (&self).neg()
    }
}

impl Add for G1Affine {
    type Output = G1Affine;

    fn add(self, rhs: Self) -> Self::Output {
        self.env().crypto().bn254().g1_add(&self, &rhs)
    }
}

impl Mul<Fr> for G1Affine {
    type Output = G1Affine;

    fn mul(self, rhs: Fr) -> Self::Output {
        self.env().crypto().bn254().g1_mul(&self, &rhs)
    }
}

// G1Affine represents a point (X, Y) on the BN254 curve where X, Y ∈ Fr
// Negation of (X, Y) is defined as (X, -Y)
impl Neg for &G1Affine {
    type Output = G1Affine;

    fn neg(self) -> Self::Output {
        let mut inner: Bytes = (&self.0).into();
        let y = Fp::try_from_val(
            inner.env(),
            inner.slice(FP_SERIALIZED_SIZE as u32..).as_val(),
        )
        .unwrap_optimized();
        let neg_y = -y;
        inner.copy_from_slice(FP_SERIALIZED_SIZE as u32, &neg_y.to_array());
        G1Affine::from_bytes(BytesN::try_from_val(inner.env(), inner.as_val()).unwrap_optimized())
    }
}

impl Neg for G1Affine {
    type Output = G1Affine;

    fn neg(self) -> Self::Output {
        (&self).neg()
    }
}

impl G2Affine {
    pub fn env(&self) -> &Env {
        self.0.env()
    }
}

impl Fr {
    pub fn env(&self) -> &Env {
        self.0.env()
    }

    pub fn from_u256(value: U256) -> Self {
        value.into()
    }

    pub fn to_u256(&self) -> U256 {
        self.0.clone()
    }

    pub fn as_u256(&self) -> &U256 {
        &self.0
    }

    pub fn from_bytes(bytes: BytesN<32>) -> Self {
        U256::from_be_bytes(bytes.env(), bytes.as_ref()).into()
    }

    pub fn to_bytes(&self) -> BytesN<32> {
        self.as_u256().to_be_bytes().try_into().unwrap_optimized()
    }

    pub fn as_val(&self) -> &Val {
        self.0.as_val()
    }

    pub fn to_val(&self) -> Val {
        self.0.to_val()
    }
}

impl From<U256> for Fr {
    fn from(value: U256) -> Self {
        Self(value)
    }
}

impl From<&Fr> for U256Val {
    fn from(value: &Fr) -> Self {
        value.as_u256().into()
    }
}

impl TryFromVal<Env, Val> for Fr {
    type Error = ConversionError;

    fn try_from_val(env: &Env, val: &Val) -> Result<Self, Self::Error> {
        let u = U256::try_from_val(env, val)?;
        Ok(Fr(u))
    }
}

impl TryFromVal<Env, Fr> for Val {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, fr: &Fr) -> Result<Self, Self::Error> {
        Ok(fr.to_val())
    }
}

impl TryFromVal<Env, &Fr> for Val {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, fr: &&Fr) -> Result<Self, Self::Error> {
        Ok(fr.to_val())
    }
}

#[cfg(not(target_family = "wasm"))]
impl From<&Fr> for ScVal {
    fn from(v: &Fr) -> Self {
        Self::from(&v.0)
    }
}

#[cfg(not(target_family = "wasm"))]
impl From<Fr> for ScVal {
    fn from(v: Fr) -> Self {
        (&v).into()
    }
}

impl Eq for Fr {}

impl PartialEq for Fr {
    fn eq(&self, other: &Self) -> bool {
        self.as_u256().partial_cmp(other.as_u256()) == Some(core::cmp::Ordering::Equal)
    }
}

impl Debug for Fr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Fr({:?})", self.as_u256())
    }
}

impl Bn254 {
    pub(crate) fn new(env: &Env) -> Bn254 {
        Bn254 { env: env.clone() }
    }

    pub fn env(&self) -> &Env {
        &self.env
    }

    /// Adds two points `p0` and `p1` in G1.
    pub fn g1_add(&self, p0: &G1Affine, p1: &G1Affine) -> G1Affine {
        let env = self.env();
        let bin =
            internal::Env::bn254_g1_add(env, p0.to_object(), p1.to_object()).unwrap_infallible();
        unsafe { G1Affine::from_bytes(BytesN::unchecked_new(env.clone(), bin)) }
    }

    /// Multiplies a point `p0` in G1 by a scalar.
    pub fn g1_mul(&self, p0: &G1Affine, scalar: &Fr) -> G1Affine {
        let env = self.env();
        let bin =
            internal::Env::bn254_g1_mul(env, p0.to_object(), scalar.into()).unwrap_infallible();
        unsafe { G1Affine::from_bytes(BytesN::unchecked_new(env.clone(), bin)) }
    }

    // pairing

    /// Performs a multi-pairing check between vectors of points in G1 and G2.
    ///
    /// This function computes the pairing for each pair of points in the
    /// provided vectors `vp1` (G1 points) and `vp2` (G2 points) and verifies if
    /// the product of all pairings is equal to 1 in the target group Fp.
    ///
    /// # Returns:
    /// - `true` if the pairing check holds (i.e., the product of pairings equals 1),
    ///   otherwise `false`.
    ///
    /// # Panics:
    /// - If the lengths of `vp1` and `vp2` are not equal or if they are empty.
    pub fn pairing_check(&self, vp1: Vec<G1Affine>, vp2: Vec<G2Affine>) -> bool {
        let env = self.env();
        internal::Env::bn254_multi_pairing_check(env, vp1.into(), vp2.into())
            .unwrap_infallible()
            .into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_g1affine_to_val() {
        let env = Env::default();

        let g1 = G1Affine::from_bytes(BytesN::from_array(&env, &[1; 64]));
        let val: Val = g1.clone().into_val(&env);
        let rt: G1Affine = val.into_val(&env);

        assert_eq!(g1, rt);
    }

    #[test]
    fn test_ref_g1affine_to_val() {
        let env = Env::default();

        let g1 = G1Affine::from_bytes(BytesN::from_array(&env, &[1; 64]));
        let val: Val = (&g1).into_val(&env);
        let rt: G1Affine = val.into_val(&env);

        assert_eq!(g1, rt);
    }

    #[test]
    fn test_double_ref_g1affine_to_val() {
        let env = Env::default();

        let g1 = G1Affine::from_bytes(BytesN::from_array(&env, &[1; 64]));
        let val: Val = (&&g1).into_val(&env);
        let rt: G1Affine = val.into_val(&env);

        assert_eq!(g1, rt);
    }

    #[test]
    fn test_fr_to_val() {
        let env = Env::default();

        let fr = Fr::from_bytes(BytesN::from_array(&env, &[1; 32]));
        let val: Val = fr.clone().into_val(&env);
        let rt: Fr = val.into_val(&env);

        assert_eq!(fr, rt);
    }

    #[test]
    fn test_ref_fr_to_val() {
        let env = Env::default();

        let fr = Fr::from_bytes(BytesN::from_array(&env, &[1; 32]));
        let val: Val = (&fr).into_val(&env);
        let rt: Fr = val.into_val(&env);

        assert_eq!(fr, rt);
    }

    #[test]
    fn test_double_ref_fr_to_val() {
        let env = Env::default();

        let fr = Fr::from_bytes(BytesN::from_array(&env, &[1; 32]));
        let val: Val = (&&fr).into_val(&env);
        let rt: Fr = val.into_val(&env);

        assert_eq!(fr, rt);
    }
}
