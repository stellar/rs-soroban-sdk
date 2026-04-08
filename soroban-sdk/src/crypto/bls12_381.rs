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

pub const FP_SERIALIZED_SIZE: usize = 48; // Size in bytes of a serialized Fp element in BLS12-381. The field modulus is 381 bits, requiring 48 bytes (384 bits) with 3 bits reserved for flags.
pub const FP2_SERIALIZED_SIZE: usize = FP_SERIALIZED_SIZE * 2;
pub const G1_SERIALIZED_SIZE: usize = FP_SERIALIZED_SIZE * 2; // Must match soroban_sdk_macro::map_type::G1_SERIALIZED_SIZE.
pub const G2_SERIALIZED_SIZE: usize = FP2_SERIALIZED_SIZE * 2; // Must match soroban_sdk_macro::map_type::G2_SERIALIZED_SIZE.

/// Bls12_381 provides access to curve and field arithmetics on the BLS12-381
/// curve.
pub struct Bls12_381 {
    env: Env,
}

/// `Bls12381G1Affine` is a point in the G1 group (subgroup defined over the base field
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
/// use soroban_sdk::{Env, bytesn, crypto::bls12_381::{Bls12_381, Bls12381G1Affine}};
/// let env = Env::default();
/// let bls12_381 = env.crypto().bls12_381();
/// let zero = Bls12381G1Affine::from_bytes(bytesn!(&env, 0x400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000));
/// let one = Bls12381G1Affine::from_bytes(bytesn!(&env, 0x17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb08b3f481e3aaa0f1a09e30ed741d8ae4fcf5e095d5d00af600db18cb2c04b3edd03cc744a2888ae40caa232946c5e7e1));
/// let res = bls12_381.g1_add(&zero, &one);
/// assert_eq!(res, one);
/// ```
#[derive(Clone)]
#[repr(transparent)]
pub struct Bls12381G1Affine(BytesN<G1_SERIALIZED_SIZE>);

/// Type alias for `Bls12381G1Affine` for convenience
pub type G1Affine = Bls12381G1Affine;

/// `Bls12381G2Affine` is a point in the G2 group (subgroup defined over the quadratic
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
pub struct Bls12381G2Affine(BytesN<G2_SERIALIZED_SIZE>);

/// Type alias for `Bls12381G2Affine` for convenience
pub type G2Affine = Bls12381G2Affine;

/// `Bls12381Fp` represents an element of the base field `Fq` of the BLS12-381 elliptic
/// curve
///
/// # Serialization:
/// - The 48 bytes represent the **big-endian encoding** of an element in the
///   field `Fp`. The value is serialized as a big-endian integer.
#[derive(Clone)]
#[repr(transparent)]
pub struct Bls12381Fp(BytesN<FP_SERIALIZED_SIZE>);

/// Type alias for `Bls12381Fp` for convenience
pub type Fp = Bls12381Fp;

/// `Bls12381Fp2` represents an element of the quadratic extension field `Fq2` of the
/// BLS12-381 elliptic curve
///
/// # Serialization:
/// - The 96 bytes represent the **big-endian encoding** of an element in the
///   field `Fp2`. The bytes consist of `be_bytes(c1) || be_bytes(c0)` (`||` is
///   concatenation), where `c0` and `c1` are the two `Fp` elements (the real
///   and imaginary components).
#[derive(Clone)]
#[repr(transparent)]
pub struct Bls12381Fp2(BytesN<FP2_SERIALIZED_SIZE>);

/// Type alias for `Bls12381Fp2` for convenience
pub type Fp2 = Bls12381Fp2;

/// `Fr` represents an element in the BLS12-381 scalar field, which is a prime
/// field of order `r` (the order of the G1 and G2 groups). The struct is
/// internally represented with an `U256`, all arithmetic operations follow
/// modulo `r`.
#[derive(Clone)]
#[repr(transparent)]
pub struct Fr(U256);

impl_bytesn_repr!(Bls12381G1Affine, G1_SERIALIZED_SIZE);
impl_bytesn_repr!(Bls12381G2Affine, G2_SERIALIZED_SIZE);
impl_bytesn_repr!(Bls12381Fp, FP_SERIALIZED_SIZE);
impl_bytesn_repr!(Bls12381Fp2, FP2_SERIALIZED_SIZE);

// BLS12-381 base field modulus p in big-endian bytes.
// p = 0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab
const BLS12_381_FP_MODULUS_BE: [u8; FP_SERIALIZED_SIZE] = [
    0x1a, 0x01, 0x11, 0xea, 0x39, 0x7f, 0xe6, 0x9a, 0x4b, 0x1b, 0xa7, 0xb6, 0x43, 0x4b, 0xac, 0xd7,
    0x64, 0x77, 0x4b, 0x84, 0xf3, 0x85, 0x12, 0xbf, 0x67, 0x30, 0xd2, 0xa0, 0xf6, 0xb0, 0xf6, 0x24,
    0x1e, 0xab, 0xff, 0xfe, 0xb1, 0x53, 0xff, 0xff, 0xb9, 0xfe, 0xff, 0xff, 0xff, 0xff, 0xaa, 0xab,
];

fn validate_fp(bytes: &[u8; FP_SERIALIZED_SIZE]) {
    if bytes >= &BLS12_381_FP_MODULUS_BE {
        sdk_panic!("Bls12-381: Invalid Fp");
    }
}

fn validate_fp2(bytes: &[u8; FP2_SERIALIZED_SIZE]) {
    validate_fp(bytes[0..FP_SERIALIZED_SIZE].try_into().unwrap());
    validate_fp(bytes[FP_SERIALIZED_SIZE..].try_into().unwrap());
}

impl Bls12381G1Affine {
    pub fn from_bytes(bytes: BytesN<G1_SERIALIZED_SIZE>) -> Self {
        Self(bytes)
    }
}

impl Bls12381G2Affine {
    pub fn from_bytes(bytes: BytesN<G2_SERIALIZED_SIZE>) -> Self {
        Self(bytes)
    }
}

impl Bls12381Fp {
    pub fn from_bytes(bytes: BytesN<FP_SERIALIZED_SIZE>) -> Self {
        validate_fp(&bytes.to_array());
        Self(bytes)
    }
}

impl Bls12381Fp2 {
    pub fn from_bytes(bytes: BytesN<FP2_SERIALIZED_SIZE>) -> Self {
        validate_fp2(&bytes.to_array());
        Self(bytes)
    }
}

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

// BLS12-381 scalar field modulus r in big-endian bytes.
// r = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001
const BLS12_381_FR_MODULUS_BE: [u8; 32] = [
    0x73, 0xed, 0xa7, 0x53, 0x29, 0x9d, 0x7d, 0x48, 0x33, 0x39, 0xd8, 0x08, 0x09, 0xa1, 0xd8, 0x05,
    0x53, 0xbd, 0xa4, 0x02, 0xff, 0xfe, 0x5b, 0xfe, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x01,
];

fn fr_modulus(env: &Env) -> U256 {
    U256::from_be_bytes(env, &Bytes::from_array(env, &BLS12_381_FR_MODULUS_BE))
}

impl From<U256> for Fr {
    fn from(value: U256) -> Self {
        // Keep all Fr construction paths canonical by reducing modulo r here.
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

impl From<&Fr> for U256Val {
    fn from(value: &Fr) -> Self {
        value.as_u256().into()
    }
}

impl TryFromVal<Env, Val> for Fr {
    type Error = ConversionError;

    fn try_from_val(env: &Env, val: &Val) -> Result<Self, Self::Error> {
        let u = U256::try_from_val(env, val)?;
        Ok(u.into())
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

    /// Checks if a G1 point is on the BLS12-381 curve (no subgroup check).
    pub fn g1_is_on_curve(&self, point: &G1Affine) -> bool {
        let env = self.env();
        internal::Env::bls12_381_g1_is_on_curve(env, point.to_object())
            .unwrap_infallible()
            .into()
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

    /// Checks if a G2 point is on the BLS12-381 curve (no subgroup check).
    pub fn g2_is_on_curve(&self, point: &G2Affine) -> bool {
        let env = self.env();
        internal::Env::bls12_381_g2_is_on_curve(env, point.to_object())
            .unwrap_infallible()
            .into()
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_g1affine_to_val() {
        let env = Env::default();

        let g1 = G1Affine::from_bytes(BytesN::from_array(&env, &[1; 96]));
        let val: Val = g1.clone().into_val(&env);
        let rt: G1Affine = val.into_val(&env);

        assert_eq!(g1, rt);
    }

    #[test]
    fn test_ref_g1affine_to_val() {
        let env = Env::default();

        let g1 = G1Affine::from_bytes(BytesN::from_array(&env, &[1; 96]));
        let val: Val = (&g1).into_val(&env);
        let rt: G1Affine = val.into_val(&env);

        assert_eq!(g1, rt);
    }

    #[test]
    fn test_doule_ref_g1affine_to_val() {
        let env = Env::default();

        let g1 = G1Affine::from_bytes(BytesN::from_array(&env, &[1; 96]));
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

    #[test]
    fn test_fr_eq_both_unreduced() {
        let env = Env::default();
        let r = fr_modulus(&env);
        let one = U256::from_u32(&env, 1);

        let a = Fr::from_u256(r.add(&one));
        let b = Fr::from_u256(one.clone());
        assert_eq!(a, b);

        let two_r_plus_one = r.add(&r).add(&one);
        let c = Fr::from_u256(two_r_plus_one);
        assert_eq!(a, c);
        assert_eq!(b, c);
    }

    #[test]
    fn test_fr_eq_unreduced_vs_zero() {
        let env = Env::default();
        let r = fr_modulus(&env);
        let zero = U256::from_u32(&env, 0);

        let a = Fr::from_u256(r);
        let b = Fr::from_u256(zero);
        assert_eq!(a, b);
    }

    #[test]
    fn test_fr_reduced_value_unchanged() {
        let env = Env::default();
        let r = fr_modulus(&env);
        let val = r.sub(&U256::from_u32(&env, 1));

        let fr = Fr::from_u256(val.clone());
        assert_eq!(fr.to_u256(), val);

        let fr42 = Fr::from_u256(U256::from_u32(&env, 42));
        assert_eq!(fr42.to_u256(), U256::from_u32(&env, 42));
    }

    #[test]
    fn test_fr_from_bytes_reduces() {
        let env = Env::default();
        let one_fr = Fr::from_u256(U256::from_u32(&env, 1));

        // BLS12-381 r+1 as big-endian bytes
        let fr_from_bytes = Fr::from_bytes(bytesn!(
            &env,
            0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000002
        ));
        assert_eq!(fr_from_bytes, one_fr);
    }

    #[test]
    fn test_fr_try_from_val_reduces() {
        let env = Env::default();
        let r = fr_modulus(&env);
        let one = U256::from_u32(&env, 1);

        let unreduced_u256 = r.add(&one);
        let val: Val = unreduced_u256.into_val(&env);
        let fr_from_val: Fr = val.into_val(&env);
        let fr_one = Fr::from_u256(one);
        assert_eq!(fr_from_val, fr_one);
    }

    #[test]
    fn test_fr_u256_into_reduces() {
        // Direct From<U256>::from / .into() path must reduce
        let env = Env::default();
        let r = fr_modulus(&env);
        let one = U256::from_u32(&env, 1);

        let fr: Fr = r.add(&one).into(); // r+1 via .into()
        let fr_one: Fr = one.into();
        assert_eq!(fr, fr_one);
    }

    #[test]
    fn test_fr_eq_unreduced_vs_host_computed() {
        // User-provided unreduced Fr vs host-computed Fr
        let env = Env::default();
        let bls = Bls12_381::new(&env);
        let r = fr_modulus(&env);
        let five = U256::from_u32(&env, 5);

        // User provides r+5 (unreduced)
        let user_fr = Fr::from_u256(r.add(&five));
        // Host computes 2+3 = 5 (always reduced)
        let host_fr = bls.fr_add(
            &Fr::from_u256(U256::from_u32(&env, 2)),
            &Fr::from_u256(U256::from_u32(&env, 3)),
        );
        assert_eq!(user_fr, host_fr);
    }

    // Fp validation tests

    #[test]
    fn test_fp_max_valid_accepted() {
        let env = Env::default();
        // p - 1 (last byte 0xaa instead of 0xab)
        let mut p_minus_1 = BLS12_381_FP_MODULUS_BE;
        p_minus_1[FP_SERIALIZED_SIZE - 1] -= 1;
        let _ = Fp::from_array(&env, &p_minus_1);
    }

    #[test]
    #[should_panic(expected = "Bls12-381: Invalid Fp")]
    fn test_fp_at_modulus_panics() {
        let env = Env::default();
        let _ = Fp::from_array(&env, &BLS12_381_FP_MODULUS_BE);
    }

    #[test]
    #[should_panic(expected = "Bls12-381: Invalid Fp")]
    fn test_fp_above_modulus_panics() {
        let env = Env::default();
        let mut above = BLS12_381_FP_MODULUS_BE;
        above[FP_SERIALIZED_SIZE - 1] += 1; // p + 1
        let _ = Fp::from_array(&env, &above);
    }

    #[test]
    fn test_fp_from_bytes_validates() {
        let env = Env::default();
        // Zero should be valid
        let _ = Fp::from_bytes(BytesN::from_array(&env, &[0u8; FP_SERIALIZED_SIZE]));
    }

    #[test]
    #[should_panic(expected = "Bls12-381: Invalid Fp")]
    fn test_fp_from_bytes_rejects_modulus() {
        let env = Env::default();
        let _ = Fp::from_bytes(BytesN::from_array(&env, &BLS12_381_FP_MODULUS_BE));
    }

    #[test]
    #[should_panic(expected = "Bls12-381: Invalid Fp")]
    fn test_fp_try_from_val_rejects_modulus() {
        let env = Env::default();
        let bytes = BytesN::from_array(&env, &BLS12_381_FP_MODULUS_BE);
        let val: Val = bytes.into_val(&env);
        let _: Fp = val.into_val(&env);
    }

    #[test]
    #[should_panic(expected = "Bls12-381: Invalid Fp")]
    fn test_fp2_component_above_modulus_panics() {
        let env = Env::default();
        // First Fp component is the modulus (invalid), second is zero (valid)
        let mut fp2_bytes = [0u8; FP2_SERIALIZED_SIZE];
        fp2_bytes[0..FP_SERIALIZED_SIZE].copy_from_slice(&BLS12_381_FP_MODULUS_BE);
        let _ = Fp2::from_array(&env, &fp2_bytes);
    }

    #[test]
    #[should_panic(expected = "Bls12-381: Invalid Fp")]
    fn test_fp2_second_component_above_modulus_panics() {
        let env = Env::default();
        // First Fp component is zero (valid), second is the modulus (invalid)
        let mut fp2_bytes = [0u8; FP2_SERIALIZED_SIZE];
        fp2_bytes[FP_SERIALIZED_SIZE..].copy_from_slice(&BLS12_381_FP_MODULUS_BE);
        let _ = Fp2::from_array(&env, &fp2_bytes);
    }

    #[test]
    fn test_fp2_max_valid_accepted() {
        let env = Env::default();
        // Both components are p-1 (valid)
        let mut p_minus_1 = BLS12_381_FP_MODULUS_BE;
        p_minus_1[FP_SERIALIZED_SIZE - 1] -= 1;
        let mut fp2_bytes = [0u8; FP2_SERIALIZED_SIZE];
        fp2_bytes[0..FP_SERIALIZED_SIZE].copy_from_slice(&p_minus_1);
        fp2_bytes[FP_SERIALIZED_SIZE..].copy_from_slice(&p_minus_1);
        let _ = Fp2::from_array(&env, &fp2_bytes);
    }

    #[test]
    fn test_bls12_381_fp_modulus_matches_arkworks() {
        use ark_bls12_381::Fq;
        use ark_ff::{BigInteger, PrimeField};

        let be_bytes = Fq::MODULUS.to_bytes_be();
        assert_eq!(
            be_bytes.as_slice(),
            &BLS12_381_FP_MODULUS_BE,
            "BLS12-381 Fp modulus does not match arkworks"
        );
    }

    #[test]
    fn test_bls12_381_fr_modulus_matches_arkworks() {
        use ark_bls12_381::Fr as ArkFr;
        use ark_ff::{BigInteger, PrimeField};

        let be_bytes = ArkFr::MODULUS.to_bytes_be();
        assert_eq!(
            be_bytes.as_slice(),
            &BLS12_381_FR_MODULUS_BE,
            "BLS12-381 Fr modulus does not match arkworks"
        );
    }
}
