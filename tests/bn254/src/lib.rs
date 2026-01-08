#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    crypto::bn254::{Bn254G1Affine, Bn254G2Affine, Fr},
    Env, Vec,
};

#[derive(Clone)]
#[contracttype]
pub struct MockProof {
    pub g1: Vec<Bn254G1Affine>,
    pub g2: Vec<Bn254G2Affine>,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn verify_pairing(env: Env, proof: MockProof) -> bool {
        env.crypto().bn254().pairing_check(proof.g1, proof.g2)
    }

    pub fn g1_add(a: Bn254G1Affine, b: Bn254G1Affine) -> Bn254G1Affine {
        a + b
    }

    pub fn g1_mul(p: Bn254G1Affine, s: Fr) -> Bn254G1Affine {
        p * s
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{vec, Env, U256};
    extern crate std;

    use crate::{Contract, ContractClient};

    // From https://github.com/ethereum/go-ethereum/blob/master/core/vm/testdata/precompiles/bn256Add.json
    fn parse_ethereum_g1_add_input(input: &str) -> ([u8; 64], [u8; 64]) {
        let bytes = hex::decode(input).unwrap();
        assert_eq!(bytes.len(), 128); // Two G1 points (64 bytes each)

        let g1_1: [u8; 64] = bytes[0..64].try_into().unwrap();
        let g1_2: [u8; 64] = bytes[64..128].try_into().unwrap();
        (g1_1, g1_2)
    }

    fn parse_ethereum_pairing_input(
        input: &str,
    ) -> (std::vec::Vec<[u8; 64]>, std::vec::Vec<[u8; 128]>) {
        let bytes = hex::decode(input).unwrap();
        assert_eq!(bytes.len() % 192, 0); // Each pair is 192 bytes

        let num_pairs = bytes.len() / 192;
        let mut g1_points = std::vec::Vec::new();
        let mut g2_points = std::vec::Vec::new();

        for i in 0..num_pairs {
            let offset = i * 192;
            g1_points.push(bytes[offset..offset + 64].try_into().unwrap());
            g2_points.push(bytes[offset + 64..offset + 192].try_into().unwrap());
        }
        (g1_points, g2_points)
    }

    #[test]
    fn test_add_and_mul() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        let add_input = "23f16f1bcc31bd002746da6fa3825209af9a356ccd99cf79604a430dd592bcd90a03caeda9c5aa40cdc9e4166e083492885dad36c72714e3697e34a4bc72ccaa21315394462f1a39f87462dbceb92718b220e4f80af516f727ad85380fadefbc2e4f40ea7bbe2d4d71f13c84fd2ae24a4a24d9638dd78349d0dee8435a67cca6";
        let (g1_x_bytes, g1_y_bytes) = parse_ethereum_g1_add_input(add_input);

        let x_bn254 = Bn254G1Affine::from_array(&env, &g1_x_bytes);
        let y_bn254 = Bn254G1Affine::from_array(&env, &g1_y_bytes);

        let expected_x_plus_y = hex::decode("013f227997b410cbd96b137a114f5b12d5a3a53d7482797bcd1f116ff30ff1931effebc79dee208d036553beae8ca71afb3b4c00979560db3991c7e67c49103c").unwrap();
        assert_eq!(
            client.g1_add(&x_bn254, &y_bn254).to_array(),
            expected_x_plus_y.as_slice()
        );

        let scalar: Fr = U256::from_u32(&env, 2).into();

        // G + G = 2G
        assert_eq!(
            client.g1_add(&x_bn254, &x_bn254),
            client.g1_mul(&x_bn254, &scalar)
        );
    }

    // From https://github.com/ethereum/go-ethereum/blob/master/core/vm/testdata/precompiles/bn256Pairing.json
    #[test]
    fn test_pairing() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        // Ethereum pairing check input
        let pairing_input = "1c76476f4def4bb94541d57ebba1193381ffa7aa76ada664dd31c16024c43f593034dd2920f673e204fee2811c678745fc819b55d3e9d294e45c9b03a76aef41209dd15ebff5d46c4bd888e51a93cf99a7329636c63514396b4a452003a35bf704bf11ca01483bfa8b34b43561848d28905960114c8ac04049af4b6315a416782bb8324af6cfc93537a2ad1a445cfd0ca2a71acd7ac41fadbf933c2a51be344d120a2a4cf30c1bf9845f20c6fe39e07ea2cce61f0c9bb048165fe5e4de877550111e129f1cf1097710d41c4ac70fcdfa5ba2023c6ff1cbeac322de49d1b6df7c2032c61a830e3c17286de9462bf242fca2883585b93870a73853face6a6bf411198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa";

        let (g1_points, g2_points) = parse_ethereum_pairing_input(pairing_input);

        // Convert to Soroban SDK types
        let g1_vec = vec![
            &env,
            Bn254G1Affine::from_array(&env, &g1_points[0]),
            Bn254G1Affine::from_array(&env, &g1_points[1]),
        ];

        let g2_vec = vec![
            &env,
            Bn254G2Affine::from_array(&env, &g2_points[0]),
            Bn254G2Affine::from_array(&env, &g2_points[1]),
        ];

        let proof = MockProof {
            g1: g1_vec,
            g2: g2_vec,
        };

        // This should return true for valid pairing
        assert!(client.verify_pairing(&proof));
    }

    #[test]
    fn test_g1_negation() {
        let env = Env::default();

        let negated_input = "00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000130644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd45";
        let bytes = hex::decode(negated_input).unwrap();
        assert_eq!(bytes.len(), 128);

        let g1_bytes: [u8; 64] = bytes[0..64].try_into().unwrap();
        let g1_negaed_bytes: [u8; 64] = bytes[64..128].try_into().unwrap();

        let g1 = Bn254G1Affine::from_array(&env, &g1_bytes);
        let g1_negated = Bn254G1Affine::from_array(&env, &g1_negaed_bytes);

        assert_eq!(-g1, g1_negated);
    }
}
