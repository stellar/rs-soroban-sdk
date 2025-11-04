#[cfg(not(target_family = "wasm"))]
use crate::xdr::ScVal;
use crate::{
    env::internal::{self, BytesObject, U256Val},
    impl_bytesn_repr,
    unwrap::{UnwrapInfallible, UnwrapOptimized},
    Bytes, BytesN, ConversionError, Env, IntoVal, TryFromVal, Val, Vec, U256,
};
use core::{
    cmp::Ordering,
    fmt::Debug,
    ops::{Add, Mul},
};

const FP_SERIALIZED_SIZE: usize = 32; // Size in bytes of a serialized Fp element in BN254.
pub const G1_SERIALIZED_SIZE: usize = FP_SERIALIZED_SIZE * 2; // Size in bytes of a serialized G1 element in BN254. Each coordinate (X, Y) is 32 bytes.
pub const G2_SERIALIZED_SIZE: usize = G1_SERIALIZED_SIZE * 2; // Size in bytes of a serialized G1 element in BN254. Each coordinate (X, Y) is 32 bytes.

/// Bn254 provides access to curve and pairing operations on the BN254
/// (also known as alt_bn128) curve.
pub struct Bn254 {
    env: Env,
}

// TODO: Add comments

#[derive(Clone)]
#[repr(transparent)]
pub struct G1Affine(BytesN<G1_SERIALIZED_SIZE>);

#[derive(Clone)]
#[repr(transparent)]
pub struct G2Affine(BytesN<G2_SERIALIZED_SIZE>);

#[derive(Clone)]
#[repr(transparent)]
pub struct Fr(U256);

impl_bytesn_repr!(G1Affine, G1_SERIALIZED_SIZE);
impl_bytesn_repr!(G2Affine, G2_SERIALIZED_SIZE);

struct Fp(BytesN<FP_SERIALIZED_SIZE>);

impl G1Affine {
    pub fn env(&self) -> &Env {
        self.0.env()
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
    /// the product of all pairings is equal to 1 in the target group Fq12.
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
