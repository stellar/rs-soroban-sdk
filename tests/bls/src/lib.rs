#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    crypto::bls12_381::{Fr, G1Affine, G2Affine},
    Env,
};

#[derive(Clone)]
#[contracttype]
pub struct DummyProof {
    pub g1: G1Affine,
    pub g2: G2Affine,
    pub fr: Fr,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn g1_mul(env: Env, p: G1Affine, s: Fr) -> G1Affine {
        env.crypto().bls12_381().g1_mul(&p, &s)
    }

    pub fn g2_mul(env: Env, p: G2Affine, s: Fr) -> G2Affine {
        env.crypto().bls12_381().g2_mul(&p, &s)
    }

    pub fn dummy_verify(env: Env, proof: DummyProof) -> bool {
        let g1_mul = env.crypto().bls12_381().g1_mul(&proof.g1, &proof.fr);
        let g2_mul = env.crypto().bls12_381().g2_mul(&proof.g2, &proof.fr);
        let vp1 = soroban_sdk::Vec::from_array(&env, [g1_mul]);
        let vp2 = soroban_sdk::Vec::from_array(&env, [g2_mul]);
        env.crypto().bls12_381().pairing_check(vp1, vp2)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{bytesn, Env};

    use crate::{Contract, ContractClient};

    #[test]
    fn test_g1_mul() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        // G1 generator and zero scalar
        let g1 = G1Affine::from_bytes(bytesn!(&env, 0x17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb08b3f481e3aaa0f1a09e30ed741d8ae4fcf5e095d5d00af600db18cb2c04b3edd03cc744a2888ae40caa232946c5e7e1));
        let zero = Fr::from_bytes(bytesn!(
            &env,
            0x0000000000000000000000000000000000000000000000000000000000000000
        ));
        let inf = G1Affine::from_bytes(bytesn!(&env, 0x400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000));
        let res = client.g1_mul(&g1, &zero);
        assert_eq!(res, inf);
    }

    #[test]
    fn test_g2_mul() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        // G2 generator and zero scalar
        let g2 = G2Affine::from_bytes(bytesn!(&env, 0x13e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb80606c4a02ea734cc32acd2b02bc28b99cb3e287e85a763af267492ab572e99ab3f370d275cec1da1aaa9075ff05f79be0ce5d527727d6e118cc9cdc6da2e351aadfd9baa8cbdd3a76d429a695160d12c923ac9cc3baca289e193548608b82801));
        let zero = Fr::from_bytes(bytesn!(
            &env,
            0x0000000000000000000000000000000000000000000000000000000000000000
        ));
        let inf = G2Affine::from_bytes(bytesn!(&env, 0x400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000));
        let res = client.g2_mul(&g2, &zero);
        assert_eq!(res, inf);
    }

    #[test]
    fn test_dummy_verify() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        // Use generator points
        let g1 = G1Affine::from_bytes(bytesn!(&env, 0x17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb08b3f481e3aaa0f1a09e30ed741d8ae4fcf5e095d5d00af600db18cb2c04b3edd03cc744a2888ae40caa232946c5e7e1));
        let g2 = G2Affine::from_bytes(bytesn!(&env, 0x13e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb80606c4a02ea734cc32acd2b02bc28b99cb3e287e85a763af267492ab572e99ab3f370d275cec1da1aaa9075ff05f79be0ce5d527727d6e118cc9cdc6da2e351aadfd9baa8cbdd3a76d429a695160d12c923ac9cc3baca289e193548608b82801));

        // Create a scalar value
        let fr = Fr::from_bytes(bytesn!(
            &env,
            0x0000000000000000000000000000000000000000000000000000000000000001
        ));

        let proof = DummyProof { g1, g2, fr };
        let res = client.dummy_verify(&proof);
        assert!(!res); // The pairing of generator points multiplied by the same scalar should not be the identity
    }
}
