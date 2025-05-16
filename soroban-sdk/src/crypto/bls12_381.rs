#[cfg(not(target_family = "wasm"))]
use crate::xdr::ScVal;
use crate::{
    env::internal::{self, BytesObject, U256Val, U64Val},
    impl_bytesn_repr,
    unwrap::{UnwrapInfallible, UnwrapOptimized},
    Bytes, BytesN, ConversionError, Env, IntoVal, TryFromVal, Val, Vec, U256,
};
use core::{
    cmp::Ordering,
    fmt::Debug,
    ops::{Add, Mul, Neg, Sub},
};

pub const FP_SERIALIZED_SIZE: usize = 48; // Size in bytes of a serialized Fp element in BLS12-381. The field modulus is 381 bits, requiring 48 bytes (384 bits) with 3 bits reserved for flags.
pub const FP2_SERIALIZED_SIZE: usize = FP_SERIALIZED_SIZE * 2;
pub const G1_SERIALIZED_SIZE: usize = FP_SERIALIZED_SIZE * 2; // Must match soroban_sdk_macro::map_type::G1_SERIALIZED_SIZE.
pub const G2_SERIALIZED_SIZE: usize = FP2_SERIALIZED_SIZE * 2; // Must match soroban_sdk_macro::map_type::G2_SERIALIZED_SIZE.

/// Bls12_381 provides access to curve and field arithmetics on the BLS12-381
/// curve.
pub struct Bls12_381 {
    env: Env,
}

// This routine was copied with slight modification from the arkworks library:
// https://github.com/arkworks-rs/algebra/blob/bf1c9b22b30325ef4df4f701dedcb6dea904c587/ff/src/biginteger/arithmetic.rs#L66-L79
fn sbb_for_sub_with_borrow(a: &mut u64, b: u64, borrow: u8) -> u8 {
    let tmp = (1u128 << 64) + u128::from(*a) - u128::from(b) - u128::from(borrow);
    // casting is safe here because `tmp` can only exceed u64 by a single
    // (borrow) bit, which we capture in the next line.
    *a = tmp as u64;
    u8::from(tmp >> 64 == 0)
}

#[derive(Debug)]
pub(crate) struct BigInt<const N: usize>(pub [u64; N]);

impl<const N: usize> BigInt<N> {
    pub fn sub_with_borrow(&mut self, other: &Self) -> bool {
        let mut borrow = 0;
        for i in 0..N {
            borrow = sbb_for_sub_with_borrow(&mut self.0[i], other.0[i], borrow);
        }
        borrow != 0
    }

    pub fn copy_into_array<const M: usize>(&self, slice: &mut [u8; M]) {
        const {
            if M != N * 8 {
                panic!("BigInt::copy_into_array with mismatched array length")
            }
        }

        for i in 0..N {
            let limb_bytes = self.0[N - 1 - i].to_be_bytes();
            slice[i * 8..(i + 1) * 8].copy_from_slice(&limb_bytes);
        }
    }

    pub fn is_zero(&self) -> bool {
        self.0 == [0; N]
    }
}

impl<const N: usize, const M: usize> From<&BytesN<M>> for BigInt<N> {
    fn from(bytes: &BytesN<M>) -> Self {
        if M != N * 8 {
            panic!("BytesN::Into<BigInt> - length mismatch")
        }

        let array = bytes.to_array();
        let mut limbs = [0u64; N];
        for i in 0..N {
            let start = i * 8;
            let end = start + 8;
            let mut chunk = [0u8; 8];
            chunk.copy_from_slice(&array[start..end]);
            limbs[N - 1 - i] = u64::from_be_bytes(chunk);
        }
        BigInt(limbs)
    }
}

/// `G1Affine` is a point in the G1 group (subgroup defined over the base field
///  `Fq`) of the BLS12-381 elliptic curve
///
/// # Serialization:
/// - The 96 bytes represent the **uncompressed encoding** of a point in G1. The
///   Bytes consist of `be_byte(X) || be_byte(Y)`  (`||` is concatenation),
///   where 'X' and 'Y' are the two coordinates, each being a base field element
///   `Fp`
/// - The most significant three bits (bits 0-3) of the first byte are reserved
///   for encoding flags:
///   - compression_flag (bit 0): Must always be unset (0), as only uncompressed
///     points are supported.
///   - infinity_flag (bit 1): Set if the point is the point at infinity (zero
///     point), in which case all other bits must be zero.
///   - sort_flag (bit 2): Must always be unset (0).
///
/// # Example Usage:
/// ```rust
/// use soroban_sdk::{Env, bytesn, crypto::bls12_381::{Bls12_381, G1Affine}};
/// let env = Env::default();
/// let bls12_381 = env.crypto().bls12_381();
/// let zero = G1Affine::from_bytes(bytesn!(&env, 0x400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000));
/// let one = G1Affine::from_bytes(bytesn!(&env, 0x17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb08b3f481e3aaa0f1a09e30ed741d8ae4fcf5e095d5d00af600db18cb2c04b3edd03cc744a2888ae40caa232946c5e7e1));
/// let res = bls12_381.g1_add(&zero, &one);
/// assert_eq!(res, one);
/// ```
#[derive(Clone)]
#[repr(transparent)]
pub struct G1Affine(BytesN<G1_SERIALIZED_SIZE>);

/// `G2Affine` is a point in the G2 group (subgroup defined over the quadratic
/// extension field `Fq2`) of the BLS12-381 elliptic curve
///
/// # Serialization:
/// - The 192 bytes represent the **uncompressed encoding** of a point in G2.
///   The bytes consist of `be_bytes(X_c1) || be_bytes(X_c0) || be_bytes(Y_c1)
///   || be_bytes(Y_c0)` (`||` is concatenation), where 'X' and 'Y' are the two
///   coordinates, each being an extension field element `Fp2` and `c0`, `c1`
///   are components of `Fp2` (each being `Fp`).
/// - The most significant three bits (bits 0-3) of the first byte are reserved
///   for encoding flags:
///   - compression_flag (bit 0): Must always be unset (0), as only uncompressed
///     points are supported.
///   - infinity_flag (bit 1): Set if the point is the point at infinity (zero
///     point), in which case all other bits must be zero.
///   - sort_flag (bit 2): Must always be unset (0).
#[derive(Clone)]
#[repr(transparent)]
pub struct G2Affine(BytesN<G2_SERIALIZED_SIZE>);

/// `Fp` represents an element of the base field `Fq` of the BLS12-381 elliptic
/// curve
///
/// # Serialization:
/// - The 48 bytes represent the **big-endian encoding** of an element in the
///   field `Fp`. The value is serialized as a big-endian integer.
#[derive(Clone)]
#[repr(transparent)]
pub struct Fp(BytesN<FP_SERIALIZED_SIZE>);

/// `Fp2` represents an element of the quadratic extension field `Fq2` of the
/// BLS12-381 elliptic curve
///
/// # Serialization:
/// - The 96 bytes represent the **big-endian encoding** of an element in the
///   field `Fp2`. The bytes consist of `be_bytes(c1) || be_bytes(c0)` (`||` is
///   concatenation), where `c0` and `c1` are the two `Fp` elements (the real
///   and imaginary components).
#[derive(Clone)]
#[repr(transparent)]
pub struct Fp2(BytesN<FP2_SERIALIZED_SIZE>);

/// `Fr` represents an element in the BLS12-381 scalar field, which is a prime
/// field of order `r` (the order of the G1 and G2 groups). The struct is
/// internally represented with an `U256`, all arithmetic operations follow
/// modulo `r`.
#[derive(Clone)]
#[repr(transparent)]
pub struct Fr(U256);

impl_bytesn_repr!(G1Affine, G1_SERIALIZED_SIZE);
impl_bytesn_repr!(G2Affine, G2_SERIALIZED_SIZE);
impl_bytesn_repr!(Fp, FP_SERIALIZED_SIZE);
impl_bytesn_repr!(Fp2, FP2_SERIALIZED_SIZE);

impl Fp {
    pub fn env(&self) -> &Env {
        self.0.env()
    }

    // `Fp` represents an element in the base field of the BLS12-381 elliptic curve.
    // For an element a ∈ Fp, its negation `-a` is defined as:
    //   a + (-a) = 0 (mod p)
    // where `p` is the field modulus, and to make a valid point coordinate on the
    // curve, `a` also must be within the field range (i.e., 0 ≤ a < p).
    fn checked_neg(&self) -> Option<Fp> {
        let fp_bigint: BigInt<6> = (&self.0).into();
        if fp_bigint.is_zero() {
            return Some(self.clone());
        }

        // BLS12-381 base field modulus
        const BLS12_381_MODULUS: [u64; 6] = [
            13402431016077863595,
            2210141511517208575,
            7435674573564081700,
            7239337960414712511,
            5412103778470702295,
            1873798617647539866,
        ];
        let mut res = BigInt(BLS12_381_MODULUS);

        // Compute modulus - value
        let borrow = res.sub_with_borrow(&fp_bigint);
        if borrow {
            return None;
        }

        let mut bytes = [0u8; FP_SERIALIZED_SIZE];
        res.copy_into_array(&mut bytes);
        Some(Fp::from_array(self.env(), &bytes))
    }

    /// Maps this `Fp` element to a `G1Affine` point using the [simplified SWU
    /// mapping](https://www.rfc-editor.org/rfc/rfc9380.html#name-simplified-swu-for-ab-0).
    ///
    /// <div class="warning">
    /// <h6>Warning</h6>
    /// The resulting point is on the curve but may not be in the prime-order subgroup (operations
    /// like pairing may fail). To ensure the point is in the prime-order subgroup, cofactor
    /// clearing must be performed on the output.
    ///
    /// For applications requiring a point directly in the prime-order subgroup, consider using
    /// `hash_to_g1`, which handles subgroup checks and cofactor clearing internally.
    /// </div>
    pub fn map_to_g1(&self) -> G1Affine {
        self.env().crypto().bls12_381().map_fp_to_g1(self)
    }
}

impl From<Fp> for BigInt<6> {
    fn from(fp: Fp) -> Self {
        let inner: Bytes = fp.0.into();
        let mut limbs = [0u64; 6];
        for i in 0..6u32 {
            let start = i * 8;
            let mut slice = [0u8; 8];
            inner.slice(start..start + 8).copy_into_slice(&mut slice);
            limbs[5 - i as usize] = u64::from_be_bytes(slice);
        }
        BigInt(limbs)
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

impl G1Affine {
    pub fn env(&self) -> &Env {
        self.0.env()
    }

    pub fn is_in_subgroup(&self) -> bool {
        self.env().crypto().bls12_381().g1_is_in_subgroup(self)
    }

    pub fn checked_add(&self, rhs: &Self) -> Option<Self> {
        self.env().crypto().bls12_381().g1_checked_add(self, rhs)
    }
}

impl Add for G1Affine {
    type Output = G1Affine;

    fn add(self, rhs: Self) -> Self::Output {
        self.env().crypto().bls12_381().g1_add(&self, &rhs)
    }
}

impl Mul<Fr> for G1Affine {
    type Output = G1Affine;

    fn mul(self, rhs: Fr) -> Self::Output {
        self.env().crypto().bls12_381().g1_mul(&self, &rhs)
    }
}

// G1Affine represents a point (X, Y) on the BLS12-381 curve where X, Y ∈ Fp
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

impl Fp2 {
    pub fn env(&self) -> &Env {
        self.0.env()
    }

    // An Fp2 element is represented as c0 + c1 * X, where:
    // - c0, c1 are base field elements (Fp)
    // - X is the quadratic non-residue used to construct the field extension
    // The negation of c0 + c1 * X is (-c0) + (-c1) * X.
    fn checked_neg(&self) -> Option<Fp2> {
        let mut inner = self.to_array();
        let mut slice0 = [0; FP_SERIALIZED_SIZE];
        let mut slice1 = [0; FP_SERIALIZED_SIZE];
        slice0.copy_from_slice(&inner[0..FP_SERIALIZED_SIZE]);
        slice1.copy_from_slice(&inner[FP_SERIALIZED_SIZE..FP2_SERIALIZED_SIZE]);

        // Convert both components to Fp and negate them
        let c0 = Fp::from_array(self.env(), &slice0);
        let c1 = Fp::from_array(self.env(), &slice1);

        // If either component's negation fails, the whole operation fails
        let neg_c0 = c0.checked_neg()?;
        let neg_c1 = c1.checked_neg()?;

        // Reconstruct the Fp2 element from negated components
        inner[0..FP_SERIALIZED_SIZE].copy_from_slice(&neg_c0.to_array());
        inner[FP_SERIALIZED_SIZE..FP2_SERIALIZED_SIZE].copy_from_slice(&neg_c1.to_array());

        Some(Fp2::from_array(self.env(), &inner))
    }

    /// Maps this `Fp2` element to a `G2Affine` point using the [simplified SWU
    /// mapping](https://www.rfc-editor.org/rfc/rfc9380.html#name-simplified-swu-for-ab-0).
    ///
    /// <div class="warning">
    /// <h6>Warning</h6>
    /// The resulting point is on the curve but may not be in the prime-order subgroup (operations
    /// like pairing may fail). To ensure the point is in the prime-order subgroup, cofactor
    /// clearing must be performed on the output.
    ///
    /// For applications requiring a point directly in the prime-order subgroup, consider using
    /// `hash_to_g2`, which handles subgroup checks and cofactor clearing internally.
    /// </div>
    pub fn map_to_g2(&self) -> G2Affine {
        self.env().crypto().bls12_381().map_fp2_to_g2(self)
    }
}

impl Neg for &Fp2 {
    type Output = Fp2;

    fn neg(self) -> Self::Output {
        match self.checked_neg() {
            Some(v) => v,
            None => sdk_panic!("invalid input - Fp2 component is larger than the field modulus"),
        }
    }
}

impl Neg for Fp2 {
    type Output = Fp2;

    fn neg(self) -> Self::Output {
        (&self).neg()
    }
}

impl G2Affine {
    pub fn env(&self) -> &Env {
        self.0.env()
    }

    pub fn is_in_subgroup(&self) -> bool {
        self.env().crypto().bls12_381().g2_is_in_subgroup(self)
    }

    pub fn checked_add(&self, rhs: &Self) -> Option<Self> {
        self.env().crypto().bls12_381().g2_checked_add(self, rhs)
    }
}

impl Add for G2Affine {
    type Output = G2Affine;

    fn add(self, rhs: Self) -> Self::Output {
        self.env().crypto().bls12_381().g2_add(&self, &rhs)
    }
}

impl Mul<Fr> for G2Affine {
    type Output = G2Affine;

    fn mul(self, rhs: Fr) -> Self::Output {
        self.env().crypto().bls12_381().g2_mul(&self, &rhs)
    }
}

// G2Affine represents a point (X, Y) on the BLS12-381 quadratic extension curve where X, Y ∈ Fp2
// Negation of (X, Y) is defined as (X, -Y)
impl Neg for &G2Affine {
    type Output = G2Affine;

    fn neg(self) -> Self::Output {
        let mut inner: Bytes = (&self.0).into();
        let y = Fp2::try_from_val(
            inner.env(),
            inner.slice(FP2_SERIALIZED_SIZE as u32..).as_val(),
        )
        .unwrap_optimized();
        let neg_y = -y;
        inner.copy_from_slice(FP2_SERIALIZED_SIZE as u32, &neg_y.to_array());
        G2Affine::from_bytes(BytesN::try_from_val(inner.env(), inner.as_val()).unwrap_optimized())
    }
}

impl Neg for G2Affine {
    type Output = G2Affine;

    fn neg(self) -> Self::Output {
        (&self).neg()
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

    pub fn pow(&self, rhs: u64) -> Self {
        self.env().crypto().bls12_381().fr_pow(self, rhs)
    }

    pub fn inv(&self) -> Self {
        self.env().crypto().bls12_381().fr_inv(self)
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
        self.as_u256().partial_cmp(other.as_u256()) == Some(Ordering::Equal)
    }
}

impl Debug for Fr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Fr({:?})", self.as_u256())
    }
}

impl Add for Fr {
    type Output = Fr;

    fn add(self, rhs: Self) -> Self::Output {
        self.env().crypto().bls12_381().fr_add(&self, &rhs)
    }
}

impl Sub for Fr {
    type Output = Fr;

    fn sub(self, rhs: Self) -> Self::Output {
        self.env().crypto().bls12_381().fr_sub(&self, &rhs)
    }
}

impl Mul for Fr {
    type Output = Fr;

    fn mul(self, rhs: Self) -> Self::Output {
        self.env().crypto().bls12_381().fr_mul(&self, &rhs)
    }
}

impl Bls12_381 {
    pub(crate) fn new(env: &Env) -> Bls12_381 {
        Bls12_381 { env: env.clone() }
    }

    pub fn env(&self) -> &Env {
        &self.env
    }

    // g1

    /// Checks if a point `p` in G1 is in the correct subgroup.
    pub fn g1_is_in_subgroup(&self, p: &G1Affine) -> bool {
        let env = self.env();
        let res = internal::Env::bls12_381_check_g1_is_in_subgroup(env, p.to_object())
            .unwrap_infallible();
        res.into()
    }

    /// Adds two points `p0` and `p1` in G1.
    pub fn g1_add(&self, p0: &G1Affine, p1: &G1Affine) -> G1Affine {
        let env = self.env();
        let bin = internal::Env::bls12_381_g1_add(env, p0.to_object(), p1.to_object())
            .unwrap_infallible();
        unsafe { G1Affine::from_bytes(BytesN::unchecked_new(env.clone(), bin)) }
    }

    /// Adds two points `p0` and `p1` in G1, ensuring that the result is in the
    /// correct subgroup. Note the subgroup check is computationally expensive,
    /// so if want to perform a series of additions i.e. `agg = p0 + p1 + .. + pn`,
    /// it may make sense to only call g1_checked_add on the final addition,
    /// while using `g1_add` (non-checked version) on the intermediate ones.
    pub fn g1_checked_add(&self, p0: &G1Affine, p1: &G1Affine) -> Option<G1Affine> {
        let env = self.env();
        let bin = internal::Env::bls12_381_g1_add(env, p0.to_object(), p1.to_object())
            .unwrap_infallible();
        let res = unsafe { G1Affine::from_bytes(BytesN::unchecked_new(env.clone(), bin)) };
        let is_in_correct_subgroup: bool =
            internal::Env::bls12_381_check_g1_is_in_subgroup(env, res.to_object())
                .unwrap_optimized()
                .into();
        match is_in_correct_subgroup {
            true => Some(res),
            false => None,
        }
    }

    /// Multiplies a point `p0` in G1 by a scalar.
    pub fn g1_mul(&self, p0: &G1Affine, scalar: &Fr) -> G1Affine {
        let env = self.env();
        let bin =
            internal::Env::bls12_381_g1_mul(env, p0.to_object(), scalar.into()).unwrap_infallible();
        unsafe { G1Affine::from_bytes(BytesN::unchecked_new(env.clone(), bin)) }
    }

    /// Performs a multi-scalar multiplication (MSM) operation in G1.
    pub fn g1_msm(&self, vp: Vec<G1Affine>, vs: Vec<Fr>) -> G1Affine {
        let env = self.env();
        let bin = internal::Env::bls12_381_g1_msm(env, vp.into(), vs.into()).unwrap_infallible();
        unsafe { G1Affine::from_bytes(BytesN::unchecked_new(env.clone(), bin)) }
    }

    /// Maps an element in the base field `Fp` to a point in G1.
    pub fn map_fp_to_g1(&self, fp: &Fp) -> G1Affine {
        let env = self.env();
        let bin = internal::Env::bls12_381_map_fp_to_g1(env, fp.to_object()).unwrap_infallible();
        unsafe { G1Affine::from_bytes(BytesN::unchecked_new(env.clone(), bin)) }
    }

    /// Hashes a message `msg` to a point in G1, using a domain separation tag `dst`.
    pub fn hash_to_g1(&self, msg: &Bytes, dst: &Bytes) -> G1Affine {
        let env = self.env();
        let bin = internal::Env::bls12_381_hash_to_g1(env, msg.into(), dst.to_object())
            .unwrap_infallible();
        unsafe { G1Affine::from_bytes(BytesN::unchecked_new(env.clone(), bin)) }
    }

    // g2

    /// Checks if a point `p` in G2 is in the correct subgroup.
    pub fn g2_is_in_subgroup(&self, p: &G2Affine) -> bool {
        let env = self.env();
        let res = internal::Env::bls12_381_check_g2_is_in_subgroup(env, p.to_object())
            .unwrap_infallible();
        res.into()
    }

    /// Adds two points `p0` and `p1` in G2.
    pub fn g2_add(&self, p0: &G2Affine, p1: &G2Affine) -> G2Affine {
        let env = self.env();
        let bin = internal::Env::bls12_381_g2_add(env, p0.to_object(), p1.to_object())
            .unwrap_infallible();
        unsafe { G2Affine::from_bytes(BytesN::unchecked_new(env.clone(), bin)) }
    }

    /// Adds two points `p0` and `p1` in G2, ensuring that the result is in the
    /// correct subgroup. Note the subgroup check is computationally expensive,
    /// so if want to perform a series of additions i.e. `agg = p0 + p1 + .. +pn`,     
    /// it may make sense to only call g2_checked_add on the final addition,
    /// while using `g2_add` (non-checked version) on the intermediate ones.
    pub fn g2_checked_add(&self, p0: &G2Affine, p1: &G2Affine) -> Option<G2Affine> {
        let env = self.env();
        let bin = internal::Env::bls12_381_g2_add(env, p0.to_object(), p1.to_object())
            .unwrap_infallible();
        let res = unsafe { G2Affine::from_bytes(BytesN::unchecked_new(env.clone(), bin)) };
        let is_in_correct_subgroup: bool =
            internal::Env::bls12_381_check_g2_is_in_subgroup(env, res.to_object())
                .unwrap_optimized()
                .into();
        match is_in_correct_subgroup {
            true => Some(res),
            false => None,
        }
    }

    /// Multiplies a point `p0` in G2 by a scalar.
    pub fn g2_mul(&self, p0: &G2Affine, scalar: &Fr) -> G2Affine {
        let env = self.env();
        let bin =
            internal::Env::bls12_381_g2_mul(env, p0.to_object(), scalar.into()).unwrap_infallible();
        unsafe { G2Affine::from_bytes(BytesN::unchecked_new(env.clone(), bin)) }
    }

    /// Performs a multi-scalar multiplication (MSM) operation in G2.
    pub fn g2_msm(&self, vp: Vec<G2Affine>, vs: Vec<Fr>) -> G2Affine {
        let env = self.env();
        let bin = internal::Env::bls12_381_g2_msm(env, vp.into(), vs.into()).unwrap_infallible();
        unsafe { G2Affine::from_bytes(BytesN::unchecked_new(env.clone(), bin)) }
    }

    /// Maps an element in the base field `Fp2` to a point in G2.
    pub fn map_fp2_to_g2(&self, fp2: &Fp2) -> G2Affine {
        let env = self.env();
        let bin = internal::Env::bls12_381_map_fp2_to_g2(env, fp2.to_object()).unwrap_infallible();
        unsafe { G2Affine::from_bytes(BytesN::unchecked_new(env.clone(), bin)) }
    }

    /// Hashes a message `msg` to a point in G2, using a domain separation tag `dst`.
    pub fn hash_to_g2(&self, msg: &Bytes, dst: &Bytes) -> G2Affine {
        let env = self.env();
        let bin = internal::Env::bls12_381_hash_to_g2(env, msg.into(), dst.to_object())
            .unwrap_infallible();
        unsafe { G2Affine::from_bytes(BytesN::unchecked_new(env.clone(), bin)) }
    }

    // pairing

    /// Performs a pairing check between vectors of points in G1 and G2.
    ///
    /// This function computes the pairing for each pair of points in the
    /// provided vectors `vp1` (G1 points) and `vp2` (G2 points) and verifies if
    /// the overall pairing result is equal to the identity in the target group.
    ///
    /// # Returns:
    /// - `true` if the pairing check holds (i.e., the pairing result is valid
    ///   and equal to the identity element), otherwise `false`.
    ///
    /// # Panics:
    /// - If the lengths of `vp1` and `vp2` are not equal or if they are empty.
    pub fn pairing_check(&self, vp1: Vec<G1Affine>, vp2: Vec<G2Affine>) -> bool {
        let env = self.env();
        internal::Env::bls12_381_multi_pairing_check(env, vp1.into(), vp2.into())
            .unwrap_infallible()
            .into()
    }

    // scalar arithmetic

    /// Adds two scalars in the BLS12-381 scalar field `Fr`.
    pub fn fr_add(&self, lhs: &Fr, rhs: &Fr) -> Fr {
        let env = self.env();
        let v = internal::Env::bls12_381_fr_add(env, lhs.into(), rhs.into()).unwrap_infallible();
        U256::try_from_val(env, &v).unwrap_infallible().into()
    }

    /// Subtracts one scalar from another in the BLS12-381 scalar field `Fr`.
    pub fn fr_sub(&self, lhs: &Fr, rhs: &Fr) -> Fr {
        let env = self.env();
        let v = internal::Env::bls12_381_fr_sub(env, lhs.into(), rhs.into()).unwrap_infallible();
        U256::try_from_val(env, &v).unwrap_infallible().into()
    }

    /// Multiplies two scalars in the BLS12-381 scalar field `Fr`.
    pub fn fr_mul(&self, lhs: &Fr, rhs: &Fr) -> Fr {
        let env = self.env();
        let v = internal::Env::bls12_381_fr_mul(env, lhs.into(), rhs.into()).unwrap_infallible();
        U256::try_from_val(env, &v).unwrap_infallible().into()
    }

    /// Raises a scalar to the power of a given exponent in the BLS12-381 scalar field `Fr`.
    pub fn fr_pow(&self, lhs: &Fr, rhs: u64) -> Fr {
        let env = self.env();
        let rhs = U64Val::try_from_val(env, &rhs).unwrap_optimized();
        let v = internal::Env::bls12_381_fr_pow(env, lhs.into(), rhs).unwrap_infallible();
        U256::try_from_val(env, &v).unwrap_infallible().into()
    }

    /// Computes the multiplicative inverse of a scalar in the BLS12-381 scalar field `Fr`.
    pub fn fr_inv(&self, lhs: &Fr) -> Fr {
        let env = self.env();
        let v = internal::Env::bls12_381_fr_inv(env, lhs.into()).unwrap_infallible();
        U256::try_from_val(env, &v).unwrap_infallible().into()
    }
}
