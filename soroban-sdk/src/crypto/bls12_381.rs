use crate::{
    env::internal::{self, BytesObject, U64Val},
    impl_bytesn_repr,
    unwrap::{UnwrapInfallible, UnwrapOptimized},
    Bytes, BytesN, ConversionError, Env, IntoVal, TryFromVal, Val, Vec, U256,
};
use core::{cmp::Ordering, fmt::Debug};
/// Bls12_381 provides access to curve and field arithmetics on the BLS12-381
/// curve.
pub struct Bls12_381 {
    env: Env,
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
///   - compression_flag (bit 0): Must always be set (1), as only uncompressed
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
pub struct G1Affine(BytesN<96>);

/// # `G2Affine` is a point in the G2 group (subgroup defined over the quadratic
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
///   - compression_flag (bit 0): Must always be set (1), as only uncompressed
///     points are supported.
///   - infinity_flag (bit 1): Set if the point is the point at infinity (zero
///     point), in which case all other bits must be zero.
///   - sort_flag (bit 2): Must always be unset (0).
#[derive(Clone)]
#[repr(transparent)]
pub struct G2Affine(BytesN<192>);

/// # `Fp` represents an element of the base field `Fq` of the BLS12-381 elliptic
/// curve
///
/// # Serialization:
/// - The 48 bytes represent the **big-endian encoding** of an element in the
///   field `Fp`. The value is serialized as a big-endian integer.
#[derive(Clone)]
#[repr(transparent)]
pub struct Fp(BytesN<48>);

/// # `Fp2` represents an element of the quadratic extension field `Fq2` of the
/// BLS12-381 elliptic curve
///
/// # Serialization:
/// - The 96 bytes represent the **big-endian encoding** of an element in the
///   field `Fp2`. The bytes consist of `be_bytes(c1) || be_bytes(c0)` (`||` is
///   concatenation), where `c0` and `c1` are the two `Fp` elements (the real
///   and imaginary components).
#[derive(Clone)]
#[repr(transparent)]
pub struct Fp2(BytesN<96>);

impl_bytesn_repr!(G1Affine, 96);
impl_bytesn_repr!(G2Affine, 192);
impl_bytesn_repr!(Fp, 48);
impl_bytesn_repr!(Fp2, 96);

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
        G1Affine::from_bytes(bin.into_val(env))
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
        let res = G1Affine::from_bytes(bin.into_val(env));
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
    pub fn g1_mul(&self, p0: &G1Affine, scalar: &U256) -> G1Affine {
        let env = self.env();
        let bin =
            internal::Env::bls12_381_g1_mul(env, p0.to_object(), scalar.into()).unwrap_infallible();
        G1Affine::from_bytes(bin.into_val(env))
    }

    /// Performs a multi-scalar multiplication (MSM) operation in G1.
    pub fn g1_msm(&self, vp: Vec<G1Affine>, vs: Vec<U256>) -> G1Affine {
        let env = self.env();
        let bin = internal::Env::bls12_381_g1_msm(env, vp.into(), vs.into()).unwrap_infallible();
        G1Affine::from_bytes(bin.into_val(env))
    }

    /// Maps an element in the base field `Fp` to a point in G1.
    pub fn map_fp_to_g1(&self, fp: &Fp) -> G1Affine {
        let env = self.env();
        let bin = internal::Env::bls12_381_map_fp_to_g1(env, fp.to_object()).unwrap_infallible();
        G1Affine::from_bytes(bin.into_val(env))
    }

    /// Hashes a message `msg` to a point in G1, using a domain separation tag `dst`.
    pub fn hash_to_g1(&self, msg: &Bytes, dst: &Bytes) -> G1Affine {
        let env = self.env();
        let bin = internal::Env::bls12_381_hash_to_g1(env, msg.into(), dst.to_object())
            .unwrap_infallible();
        G1Affine::from_bytes(bin.into_val(env))
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
        G2Affine::from_bytes(bin.into_val(env))
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
        let res = G2Affine::from_bytes(bin.into_val(env));
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
    pub fn g2_mul(&self, p0: &G2Affine, scalar: &U256) -> G2Affine {
        let env = self.env();
        let bin =
            internal::Env::bls12_381_g2_mul(env, p0.to_object(), scalar.into()).unwrap_infallible();
        G2Affine::from_bytes(bin.into_val(env))
    }

    /// Performs a multi-scalar multiplication (MSM) operation in G2.
    pub fn g2_msm(&self, vp: Vec<G2Affine>, vs: Vec<U256>) -> G2Affine {
        let env = self.env();
        let bin = internal::Env::bls12_381_g2_msm(env, vp.into(), vs.into()).unwrap_infallible();
        G2Affine::from_bytes(bin.into_val(env))
    }

    /// Maps an element in the base field `Fp2` to a point in G2.
    pub fn map_fp2_to_g2(&self, fp2: &Fp2) -> G2Affine {
        let env = self.env();
        let bin = internal::Env::bls12_381_map_fp2_to_g2(env, fp2.to_object()).unwrap_infallible();
        G2Affine::from_bytes(bin.into_val(env))
    }

    /// Hashes a message `msg` to a point in G2, using a domain separation tag `dst`.
    pub fn hash_to_g2(&self, msg: &Bytes, dst: &Bytes) -> G2Affine {
        let env = self.env();
        let bin = internal::Env::bls12_381_hash_to_g2(env, msg.into(), dst.to_object())
            .unwrap_infallible();
        G2Affine::from_bytes(bin.into_val(env))
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
    pub fn fr_add(&self, lhs: &U256, rhs: &U256) -> U256 {
        let env = self.env();
        let v = internal::Env::bls12_381_fr_add(env, lhs.into(), rhs.into()).unwrap_infallible();
        U256::try_from_val(env, &v).unwrap_infallible()
    }

    /// Subtracts one scalar from another in the BLS12-381 scalar field `Fr`.
    pub fn fr_sub(&self, lhs: &U256, rhs: &U256) -> U256 {
        let env = self.env();
        let v = internal::Env::bls12_381_fr_sub(env, lhs.into(), rhs.into()).unwrap_infallible();
        U256::try_from_val(env, &v).unwrap_infallible()
    }

    /// Multiplies two scalars in the BLS12-381 scalar field `Fr`.
    pub fn fr_mul(&self, lhs: &U256, rhs: &U256) -> U256 {
        let env = self.env();
        let v = internal::Env::bls12_381_fr_mul(env, lhs.into(), rhs.into()).unwrap_infallible();
        U256::try_from_val(env, &v).unwrap_infallible()
    }

    /// Raises a scalar to the power of a given exponent in the BLS12-381 scalar field `Fr`.
    pub fn fr_pow(&self, lhs: &U256, rhs: u64) -> U256 {
        let env = self.env();
        let rhs = U64Val::try_from_val(env, &rhs).unwrap_optimized();
        let v = internal::Env::bls12_381_fr_pow(env, lhs.into(), rhs).unwrap_infallible();
        U256::try_from_val(env, &v).unwrap_infallible()
    }

    /// Computes the multiplicative inverse of a scalar in the BLS12-381 scalar field `Fr`.
    pub fn fr_inv(&self, lhs: &U256) -> U256 {
        let env = self.env();
        let v = internal::Env::bls12_381_fr_inv(env, lhs.into()).unwrap_infallible();
        U256::try_from_val(env, &v).unwrap_infallible()
    }
}
