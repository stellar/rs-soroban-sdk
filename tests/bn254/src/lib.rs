#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    crypto::bn254::{Fr, G1Affine, G2Affine},
    Env, Vec,
};

#[derive(Clone)]
#[contracttype]
pub struct MockProof {
    pub g1: Vec<G1Affine>,
    pub g2: Vec<G2Affine>,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn verify_pairing(env: Env, proof: MockProof) -> bool {
        env.crypto().bn254().pairing_check(proof.g1, proof.g2)
    }

    pub fn g1_add(env: Env, a: G1Affine, b: G1Affine) -> G1Affine {
        env.crypto().bn254().g1_add(&a, &b)
    }

    pub fn g1_mul(env: Env, p: G1Affine, s: Fr) -> G1Affine {
        env.crypto().bn254().g1_mul(&p, &s)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{crypto::bn254, vec, Env, U256};
    extern crate std;

    use std::ops::Add;

    use crate::{Contract, ContractClient};

    use ark_bn254::{G1Affine, G2Affine};
    use ark_ec::CurveGroup;
    use ark_ff::UniformRand;
    use ark_serialize::CanonicalSerialize;

    #[test]
    fn test_add_and_mul() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        // Generate random points
        let mut rng = ark_std::test_rng();

        let mut a_bytes = [0u8; 64];
        G1Affine::rand(&mut rng)
            .serialize_uncompressed(&mut a_bytes[..])
            .unwrap();

        let a_bn254 = bn254::G1Affine::from_array(&env, &a_bytes);

        let scalar: bn254::Fr = U256::from_u32(&env, 2).into();

        // G + G = 2G
        assert_eq!(
            client.g1_add(&a_bn254, &a_bn254),
            client.g1_mul(&a_bn254, &scalar)
        );
    }

    // Test e(P, Q+R) = e(P, Q)*e(P, R)
    #[test]
    fn test_pairing() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        // Generate random points
        let mut rng = ark_std::test_rng();
        let p = G1Affine::rand(&mut rng);
        let neg_p = -p;
        let q = G2Affine::rand(&mut rng);
        let r = G2Affine::rand(&mut rng);
        let q_plus_r = &q.add(&r).into_affine();
        // Serialize points
        let mut p_bytes = [0u8; 64];
        let mut neg_p_bytes = [0u8; 64];

        let mut q_bytes = [0u8; 128];
        let mut r_bytes = [0u8; 128];
        let mut q_plus_r_bytes = [0u8; 128];
        p.serialize_uncompressed(&mut p_bytes[..]).unwrap();
        neg_p.serialize_uncompressed(&mut neg_p_bytes[..]).unwrap();
        q.serialize_uncompressed(&mut q_bytes[..]).unwrap();
        r.serialize_uncompressed(&mut r_bytes[..]).unwrap();
        q_plus_r
            .serialize_uncompressed(&mut q_plus_r_bytes[..])
            .unwrap();

        // Create proof
        let proof = MockProof {
            g1: vec![
                &env,
                bn254::G1Affine::from_array(&env, &neg_p_bytes),
                bn254::G1Affine::from_array(&env, &p_bytes),
                bn254::G1Affine::from_array(&env, &p_bytes),
            ],
            g2: vec![
                &env,
                bn254::G2Affine::from_array(&env, &q_plus_r_bytes),
                bn254::G2Affine::from_array(&env, &q_bytes),
                bn254::G2Affine::from_array(&env, &r_bytes),
            ],
        };

        assert!(client.verify_pairing(&proof));
    }
}
