#[cfg(not(target_family = "wasm"))]
use crate::xdr::ScVal;
use crate::{
    crypto::utils::BigInt,
    env::internal::{self, BytesObject, U256Val, U64Val},
    unwrap::{UnwrapInfallible, UnwrapOptimized},
    Bytes, BytesN, ConversionError, Env, IntoVal, TryFromVal, Val, Vec, U256,
};
use core::{
    cmp::Ordering,
    fmt::Debug,
    ops::{Add, Mul, Neg, Sub},
};

pub const BN254_FP_SERIALIZED_SIZE: usize = 32; // Size in bytes of a serialized Bn254Fp element in BN254. The field modulus is 254 bits, requiring 32 bytes (256 bits).
pub const BN254_G1_SERIALIZED_SIZE: usize = BN254_FP_SERIALIZED_SIZE * 2; // Size in bytes of a serialized G1 element in BN254. Each coordinate (X, Y) is 32 bytes.
pub const BN254_G2_SERIALIZED_SIZE: usize = BN254_G1_SERIALIZED_SIZE * 2; // Size in bytes of a serialized G2 element in BN254. Each coordinate (X, Y) is 64 bytes (2 Bn254Fp elements per coordinate).

/// Bn254 provides access to curve and pairing operations on the BN254
/// (also known as alt_bn128) curve.
pub struct Bn254 {
    env: Env,
}

/// `Bn254G1Affine` is a point in the G1 group (subgroup defined over the base field
/// `Fq` with prime order `q =
/// 0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47`) of the
/// BN254 elliptic curve
///
/// # Serialization (Ethereum-compatible format):
/// - The 64 bytes represent the **uncompressed encoding** of a point in G1
/// - Format: `be_bytes(X) || be_bytes(Y)` where `||` denotes concatenation
/// - X and Y are curve coordinates, each a 32-byte big-endian Bn254Fp field element
/// - The two flag bits (bits 0x80 and 0x40 of the first byte) must be unset
/// - The point at infinity is encoded as 64 zero bytes
/// - Points must be on the curve (no subgroup check required for G1)
#[derive(Clone)]
#[repr(transparent)]
pub struct Bn254G1Affine(BytesN<BN254_G1_SERIALIZED_SIZE>);

/// `Bn254G2Affine` is a point in the G2 group (subgroup defined over the quadratic
/// extension field `Fq2`) of the BN254 elliptic curve
///
/// # Serialization (Ethereum-compatible format):
/// - The 128 bytes represent the **uncompressed encoding** of a point in G2
/// - Format: `be_bytes(X) || be_bytes(Y)` where each coordinate is an Fp2
/// element (64 bytes) - Fp2 element encoding: `be_bytes(c1) || be_bytes(c0)`
/// where:
///   - c0 is the real component (32-byte big-endian Bn254Fp element)
///   - c1 is the imaginary component (32-byte big-endian Bn254Fp element)
/// - The two flag bits (bits 0x80 and 0x40 of the first byte) must be unset
/// - The point at infinity is encoded as 128 zero bytes
/// - Points must be on the curve AND in the correct subgroup
#[derive(Clone)]
#[repr(transparent)]
pub struct Bn254G2Affine(BytesN<BN254_G2_SERIALIZED_SIZE>);

/// `Bn254Fr` represents an element in the BN254 scalar field, which is a prime
/// field of order `r =
/// 0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001`. The
/// struct is internally represented with a `U256`, all arithmetic operations
/// follow modulo `r`.
#[derive(Clone)]
#[repr(transparent)]
pub struct Bn254Fr(U256);

/// Deprecated type alias for `Bn254Fr`.
/// Use `Bn254Fr` to avoid ambiguity with `Bls12381Fr`.
#[deprecated(note = "use `Bn254Fr` instead to avoid ambiguity with `Bls12381Fr`")]
pub type Fr = Bn254Fr;

/// `Bn254Fp` represents an element of the base field `Bn254Fp` of the BN254 elliptic curve
///
/// # Serialization:
/// - The 32 bytes represent the **big-endian encoding** of an element in the
///   field `Bn254Fp`. The value is serialized as a big-endian integer.
#[derive(Clone)]
#[repr(transparent)]
pub struct Bn254Fp(BytesN<BN254_FP_SERIALIZED_SIZE>);

impl_bytesn_repr!(Bn254G1Affine, BN254_G1_SERIALIZED_SIZE);
impl_bytesn_repr!(Bn254G2Affine, BN254_G2_SERIALIZED_SIZE);
impl_bytesn_repr!(Bn254Fp, BN254_FP_SERIALIZED_SIZE);

// BN254 base field modulus p in big-endian bytes.
// p = 0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47
const BN254_FP_MODULUS_BE: [u8; BN254_FP_SERIALIZED_SIZE] = [
    0x30, 0x64, 0x4e, 0x72, 0xe1, 0x31, 0xa0, 0x29, 0xb8, 0x50, 0x45, 0xb6, 0x81, 0x81, 0x58, 0x5d,
    0x97, 0x81, 0x6a, 0x91, 0x68, 0x71, 0xca, 0x8d, 0x3c, 0x20, 0x8c, 0x16, 0xd8, 0x7c, 0xfd, 0x47,
];

fn validate_bn254_fp(bytes: &[u8; BN254_FP_SERIALIZED_SIZE]) {
    if bytes >= &BN254_FP_MODULUS_BE {
        sdk_panic!("Bn254: Invalid Fp");
    }
}

impl Bn254G1Affine {
    pub fn from_bytes(bytes: BytesN<BN254_G1_SERIALIZED_SIZE>) -> Self {
        Self(bytes)
    }
}

impl Bn254G2Affine {
    pub fn from_bytes(bytes: BytesN<BN254_G2_SERIALIZED_SIZE>) -> Self {
        Self(bytes)
    }
}

impl Bn254Fp {
    pub fn from_bytes(bytes: BytesN<BN254_FP_SERIALIZED_SIZE>) -> Self {
        validate_bn254_fp(&bytes.to_array());
        Self(bytes)
    }
}

impl Bn254G1Affine {
    pub fn env(&self) -> &Env {
        self.0.env()
    }
}

impl Bn254Fp {
    pub fn env(&self) -> &Env {
        self.0.env()
    }

    // `Bn254Fp` represents an element in the base field of the BN254 elliptic curve.
    // For an element a ∈ Bn254Fp, its negation `-a` is defined as:
    //   a + (-a) = 0 (mod p)
    // where `p` is the field modulus, and to make a valid point coordinate on the
    // curve, `a` also must be within the field range (i.e., 0 ≤ a < p).
    fn checked_neg(&self) -> Option<Bn254Fp> {
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

        let mut bytes = [0u8; BN254_FP_SERIALIZED_SIZE];
        res.copy_into_array(&mut bytes);
        Some(Bn254Fp::from_array(self.env(), &bytes))
    }
}

impl Neg for &Bn254Fp {
    type Output = Bn254Fp;

    fn neg(self) -> Self::Output {
        match self.checked_neg() {
            Some(v) => v,
            None => sdk_panic!("invalid input - Bn254Fp is larger than the field modulus"),
        }
    }
}

impl Neg for Bn254Fp {
    type Output = Bn254Fp;

    fn neg(self) -> Self::Output {
        (&self).neg()
    }
}

impl Add for Bn254G1Affine {
    type Output = Bn254G1Affine;

    fn add(self, rhs: Self) -> Self::Output {
        self.env().crypto().bn254().g1_add(&self, &rhs)
    }
}

impl Mul<Bn254Fr> for Bn254G1Affine {
    type Output = Bn254G1Affine;

    fn mul(self, rhs: Bn254Fr) -> Self::Output {
        self.env().crypto().bn254().g1_mul(&self, &rhs)
    }
}

// Bn254G1Affine represents a point (X, Y) on the BN254 curve where X, Y ∈ Bn254Fp
// Negation of (X, Y) is defined as (X, -Y)
impl Neg for &Bn254G1Affine {
    type Output = Bn254G1Affine;

    fn neg(self) -> Self::Output {
        let mut inner: Bytes = (&self.0).into();
        let y = Bn254Fp::try_from_val(
            inner.env(),
            inner.slice(BN254_FP_SERIALIZED_SIZE as u32..).as_val(),
        )
        .unwrap_optimized();
        let neg_y = -y;
        inner.copy_from_slice(BN254_FP_SERIALIZED_SIZE as u32, &neg_y.to_array());
        Bn254G1Affine::from_bytes(
            BytesN::try_from_val(inner.env(), inner.as_val()).unwrap_optimized(),
        )
    }
}

impl Neg for Bn254G1Affine {
    type Output = Bn254G1Affine;

    fn neg(self) -> Self::Output {
        (&self).neg()
    }
}

impl Bn254G2Affine {
    pub fn env(&self) -> &Env {
        self.0.env()
    }
}

impl Bn254Fr {
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

    pub fn pow(&self, rhs: u64) -> Self {
        self.env().crypto().bn254().fr_pow(self, rhs)
    }

    pub fn inv(&self) -> Self {
        self.env().crypto().bn254().fr_inv(self)
    }
}

impl Add for Bn254Fr {
    type Output = Bn254Fr;

    fn add(self, rhs: Self) -> Self::Output {
        self.env().crypto().bn254().fr_add(&self, &rhs)
    }
}

impl Sub for Bn254Fr {
    type Output = Bn254Fr;

    fn sub(self, rhs: Self) -> Self::Output {
        self.env().crypto().bn254().fr_sub(&self, &rhs)
    }
}

impl Mul for Bn254Fr {
    type Output = Bn254Fr;

    fn mul(self, rhs: Self) -> Self::Output {
        self.env().crypto().bn254().fr_mul(&self, &rhs)
    }
}

// BN254 scalar field modulus r in big-endian bytes.
// r = 0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001
const BN254_FR_MODULUS_BE: [u8; 32] = [
    0x30, 0x64, 0x4e, 0x72, 0xe1, 0x31, 0xa0, 0x29, 0xb8, 0x50, 0x45, 0xb6, 0x81, 0x81, 0x58, 0x5d,
    0x28, 0x33, 0xe8, 0x48, 0x79, 0xb9, 0x70, 0x91, 0x43, 0xe1, 0xf5, 0x93, 0xf0, 0x00, 0x00, 0x01,
];

fn fr_modulus(env: &Env) -> U256 {
    U256::from_be_bytes(env, &Bytes::from_array(env, &BN254_FR_MODULUS_BE))
}

impl From<U256> for Bn254Fr {
    fn from(value: U256) -> Self {
        // Keep all Bn254Fr construction paths canonical by reducing modulo r here.
        // Constructors and deserialization paths should route through this impl.
        // Skip the expensive rem_euclid when value is already canonical (< r),
        // which is always the case for host-returned arithmetic results.
        let modulus = fr_modulus(value.env());
        if value >= modulus {
            Self(value.rem_euclid(&modulus))
        } else {
            Self(value)
        }
    }
}

impl From<&Bn254Fr> for U256Val {
    fn from(value: &Bn254Fr) -> Self {
        value.as_u256().into()
    }
}

impl TryFromVal<Env, Val> for Bn254Fr {
    type Error = ConversionError;

    fn try_from_val(env: &Env, val: &Val) -> Result<Self, Self::Error> {
        let u = U256::try_from_val(env, val)?;
        Ok(u.into())
    }
}

impl TryFromVal<Env, Bn254Fr> for Val {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, fr: &Bn254Fr) -> Result<Self, Self::Error> {
        Ok(fr.to_val())
    }
}

impl TryFromVal<Env, &Bn254Fr> for Val {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, fr: &&Bn254Fr) -> Result<Self, Self::Error> {
        Ok(fr.to_val())
    }
}

#[cfg(not(target_family = "wasm"))]
impl From<&Bn254Fr> for ScVal {
    fn from(v: &Bn254Fr) -> Self {
        Self::from(&v.0)
    }
}

#[cfg(not(target_family = "wasm"))]
impl From<Bn254Fr> for ScVal {
    fn from(v: Bn254Fr) -> Self {
        (&v).into()
    }
}

impl Eq for Bn254Fr {}

impl PartialEq for Bn254Fr {
    fn eq(&self, other: &Self) -> bool {
        self.as_u256().partial_cmp(other.as_u256()) == Some(core::cmp::Ordering::Equal)
    }
}

impl Debug for Bn254Fr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Bn254Fr({:?})", self.as_u256())
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
    pub fn g1_add(&self, p0: &Bn254G1Affine, p1: &Bn254G1Affine) -> Bn254G1Affine {
        let env = self.env();
        let bin =
            internal::Env::bn254_g1_add(env, p0.to_object(), p1.to_object()).unwrap_infallible();
        unsafe { Bn254G1Affine::from_bytes(BytesN::unchecked_new(env.clone(), bin)) }
    }

    /// Multiplies a point `p0` in G1 by a scalar.
    pub fn g1_mul(&self, p0: &Bn254G1Affine, scalar: &Bn254Fr) -> Bn254G1Affine {
        let env = self.env();
        let bin =
            internal::Env::bn254_g1_mul(env, p0.to_object(), scalar.into()).unwrap_infallible();
        unsafe { Bn254G1Affine::from_bytes(BytesN::unchecked_new(env.clone(), bin)) }
    }

    // pairing

    /// Performs a multi-pairing check between vectors of points in G1 and G2.
    ///
    /// This function computes the pairing for each pair of points in the
    /// provided vectors `vp1` (G1 points) and `vp2` (G2 points) and verifies if
    /// the product of all pairings is equal to 1 in the target group Bn254Fp.
    ///
    /// # Returns:
    /// - `true` if the pairing check holds (i.e., the product of pairings equals 1),
    ///   otherwise `false`.
    ///
    /// # Panics:
    /// - If the lengths of `vp1` and `vp2` are not equal or if they are empty.
    pub fn pairing_check(&self, vp1: Vec<Bn254G1Affine>, vp2: Vec<Bn254G2Affine>) -> bool {
        let env = self.env();
        internal::Env::bn254_multi_pairing_check(env, vp1.into(), vp2.into())
            .unwrap_infallible()
            .into()
    }

    /// Performs a multi-scalar multiplication (MSM) operation in G1.
    pub fn g1_msm(&self, vp: Vec<Bn254G1Affine>, vs: Vec<Bn254Fr>) -> Bn254G1Affine {
        let env = self.env();
        let bin = internal::Env::bn254_g1_msm(env, vp.into(), vs.into()).unwrap_infallible();
        unsafe { Bn254G1Affine::from_bytes(BytesN::unchecked_new(env.clone(), bin)) }
    }

    /// Checks if a G1 point is on the BN254 curve.
    pub fn g1_is_on_curve(&self, point: &Bn254G1Affine) -> bool {
        let env = self.env();
        internal::Env::bn254_g1_is_on_curve(env, point.to_object())
            .unwrap_infallible()
            .into()
    }

    // scalar arithmetic

    /// Adds two scalars in the BN254 scalar field `Bn254Fr`.
    pub fn fr_add(&self, lhs: &Bn254Fr, rhs: &Bn254Fr) -> Bn254Fr {
        let env = self.env();
        let v = internal::Env::bn254_fr_add(env, lhs.into(), rhs.into()).unwrap_infallible();
        U256::try_from_val(env, &v).unwrap_infallible().into()
    }

    /// Subtracts one scalar from another in the BN254 scalar field `Bn254Fr`.
    pub fn fr_sub(&self, lhs: &Bn254Fr, rhs: &Bn254Fr) -> Bn254Fr {
        let env = self.env();
        let v = internal::Env::bn254_fr_sub(env, lhs.into(), rhs.into()).unwrap_infallible();
        U256::try_from_val(env, &v).unwrap_infallible().into()
    }

    /// Multiplies two scalars in the BN254 scalar field `Bn254Fr`.
    pub fn fr_mul(&self, lhs: &Bn254Fr, rhs: &Bn254Fr) -> Bn254Fr {
        let env = self.env();
        let v = internal::Env::bn254_fr_mul(env, lhs.into(), rhs.into()).unwrap_infallible();
        U256::try_from_val(env, &v).unwrap_infallible().into()
    }

    /// Raises a scalar to the power of a given exponent in the BN254 scalar field `Bn254Fr`.
    pub fn fr_pow(&self, lhs: &Bn254Fr, rhs: u64) -> Bn254Fr {
        let env = self.env();
        let rhs = U64Val::try_from_val(env, &rhs).unwrap_optimized();
        let v = internal::Env::bn254_fr_pow(env, lhs.into(), rhs).unwrap_infallible();
        U256::try_from_val(env, &v).unwrap_infallible().into()
    }

    /// Computes the multiplicative inverse of a scalar in the BN254 scalar field `Bn254Fr`.
    pub fn fr_inv(&self, lhs: &Bn254Fr) -> Bn254Fr {
        let env = self.env();
        let v = internal::Env::bn254_fr_inv(env, lhs.into()).unwrap_infallible();
        U256::try_from_val(env, &v).unwrap_infallible().into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_g1affine_to_val() {
        let env = Env::default();

        let g1 = Bn254G1Affine::from_bytes(BytesN::from_array(&env, &[1; 64]));
        let val: Val = g1.clone().into_val(&env);
        let rt: Bn254G1Affine = val.into_val(&env);

        assert_eq!(g1, rt);
    }

    #[test]
    fn test_ref_g1affine_to_val() {
        let env = Env::default();

        let g1 = Bn254G1Affine::from_bytes(BytesN::from_array(&env, &[1; 64]));
        let val: Val = (&g1).into_val(&env);
        let rt: Bn254G1Affine = val.into_val(&env);

        assert_eq!(g1, rt);
    }

    #[test]
    fn test_double_ref_g1affine_to_val() {
        let env = Env::default();

        let g1 = Bn254G1Affine::from_bytes(BytesN::from_array(&env, &[1; 64]));
        let val: Val = (&&g1).into_val(&env);
        let rt: Bn254G1Affine = val.into_val(&env);

        assert_eq!(g1, rt);
    }

    #[test]
    fn test_fr_to_val() {
        let env = Env::default();

        let fr = Bn254Fr::from_bytes(BytesN::from_array(&env, &[1; 32]));
        let val: Val = fr.clone().into_val(&env);
        let rt: Bn254Fr = val.into_val(&env);

        assert_eq!(fr, rt);
    }

    #[test]
    fn test_ref_fr_to_val() {
        let env = Env::default();

        let fr = Bn254Fr::from_bytes(BytesN::from_array(&env, &[1; 32]));
        let val: Val = (&fr).into_val(&env);
        let rt: Bn254Fr = val.into_val(&env);

        assert_eq!(fr, rt);
    }

    #[test]
    fn test_double_ref_fr_to_val() {
        let env = Env::default();

        let fr = Bn254Fr::from_bytes(BytesN::from_array(&env, &[1; 32]));
        let val: Val = (&&fr).into_val(&env);
        let rt: Bn254Fr = val.into_val(&env);

        assert_eq!(fr, rt);
    }

    #[test]
    fn test_fr_eq_both_unreduced() {
        // Both inputs are user-provided unreduced values representing the same field element
        let env = Env::default();
        let r = fr_modulus(&env);
        let one = U256::from_u32(&env, 1);

        let a = Bn254Fr::from_u256(r.add(&one)); // r+1 ≡ 1 (mod r)
        let b = Bn254Fr::from_u256(one.clone()); // 1
        assert_eq!(a, b);

        // Both unreduced by different multiples of r
        let two_r_plus_one = r.add(&r).add(&one);
        let c = Bn254Fr::from_u256(two_r_plus_one); // 2r+1 ≡ 1 (mod r)
        assert_eq!(a, c);
        assert_eq!(b, c);
    }

    #[test]
    fn test_fr_eq_unreduced_vs_zero() {
        // value == r should reduce to 0
        let env = Env::default();
        let r = fr_modulus(&env);
        let zero = U256::from_u32(&env, 0);

        let a = Bn254Fr::from_u256(r);
        let b = Bn254Fr::from_u256(zero);
        assert_eq!(a, b);
    }

    #[test]
    fn test_fr_reduced_value_unchanged() {
        // value < r should be preserved as-is
        let env = Env::default();
        let r = fr_modulus(&env);
        let val = r.sub(&U256::from_u32(&env, 1)); // r-1

        let fr = Bn254Fr::from_u256(val.clone());
        assert_eq!(fr.to_u256(), val);

        // small values
        let fr42 = Bn254Fr::from_u256(U256::from_u32(&env, 42));
        assert_eq!(fr42.to_u256(), U256::from_u32(&env, 42));
    }

    #[test]
    fn test_fr_from_bytes_reduces() {
        // from_bytes should also reduce since it goes through From<U256>
        let env = Env::default();
        let one_fr = Bn254Fr::from_u256(U256::from_u32(&env, 1));

        // BN254 r+1 as big-endian bytes
        let fr_from_bytes = Bn254Fr::from_bytes(bytesn!(
            &env,
            0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000002
        ));
        assert_eq!(fr_from_bytes, one_fr);
    }

    #[test]
    fn test_fr_try_from_val_reduces() {
        // TryFromVal<Env, Val> path must also reduce
        let env = Env::default();
        let r = fr_modulus(&env);
        let one = U256::from_u32(&env, 1);

        // Create an unreduced U256 value (r+1), convert to Val, then to Fr
        let unreduced_u256 = r.add(&one);
        let val: Val = unreduced_u256.into_val(&env);
        let fr_from_val: Bn254Fr = val.into_val(&env);
        let fr_one = Bn254Fr::from_u256(one);
        assert_eq!(fr_from_val, fr_one);
    }

    #[test]
    fn test_fr_u256_into_reduces() {
        // Direct From<U256>::from / .into() path must reduce
        let env = Env::default();
        let r = fr_modulus(&env);
        let one = U256::from_u32(&env, 1);

        let fr: Bn254Fr = r.add(&one).into(); // r+1 via .into()
        let fr_one: Bn254Fr = one.into();
        assert_eq!(fr, fr_one);
    }

    // Bn254Fp validation tests

    #[test]
    fn test_bn254_fp_max_valid_accepted() {
        let env = Env::default();
        // p - 1 (last byte 0x46 instead of 0x47)
        let mut p_minus_1 = BN254_FP_MODULUS_BE;
        p_minus_1[BN254_FP_SERIALIZED_SIZE - 1] -= 1;
        let _ = Bn254Fp::from_array(&env, &p_minus_1);
    }

    #[test]
    #[should_panic(expected = "Bn254: Invalid Fp")]
    fn test_bn254_fp_at_modulus_panics() {
        let env = Env::default();
        let _ = Bn254Fp::from_array(&env, &BN254_FP_MODULUS_BE);
    }

    #[test]
    #[should_panic(expected = "Bn254: Invalid Fp")]
    fn test_bn254_fp_above_modulus_panics() {
        let env = Env::default();
        let mut above = BN254_FP_MODULUS_BE;
        above[BN254_FP_SERIALIZED_SIZE - 1] += 1; // p + 1
        let _ = Bn254Fp::from_array(&env, &above);
    }

    #[test]
    fn test_bn254_fp_from_bytes_validates() {
        let env = Env::default();
        // Zero should be valid
        let _ = Bn254Fp::from_bytes(BytesN::from_array(&env, &[0u8; BN254_FP_SERIALIZED_SIZE]));
    }

    #[test]
    #[should_panic(expected = "Bn254: Invalid Fp")]
    fn test_bn254_fp_from_bytes_rejects_modulus() {
        let env = Env::default();
        let _ = Bn254Fp::from_bytes(BytesN::from_array(&env, &BN254_FP_MODULUS_BE));
    }

    #[test]
    #[should_panic(expected = "Bn254: Invalid Fp")]
    fn test_bn254_fp_try_from_val_rejects_modulus() {
        let env = Env::default();
        let bytes = BytesN::from_array(&env, &BN254_FP_MODULUS_BE);
        let val: Val = bytes.into_val(&env);
        let _: Bn254Fp = val.into_val(&env);
    }

    #[test]
    fn test_bn254_fp_modulus_matches_arkworks() {
        use ark_bn254::Fq;
        use ark_ff::{BigInteger, PrimeField};

        let be_bytes = Fq::MODULUS.to_bytes_be();
        assert_eq!(
            be_bytes.as_slice(),
            &BN254_FP_MODULUS_BE,
            "BN254 Fp modulus does not match arkworks"
        );
    }

    #[test]
    fn test_bn254_fr_modulus_matches_arkworks() {
        use ark_bn254::Fr as ArkFr;
        use ark_ff::{BigInteger, PrimeField};

        let be_bytes = ArkFr::MODULUS.to_bytes_be();
        assert_eq!(
            be_bytes.as_slice(),
            &BN254_FR_MODULUS_BE,
            "BN254 Fr modulus does not match arkworks"
        );
    }
}
