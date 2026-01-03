use super::poseidon2_params::{
    get_mat_diag_m_1_bn254, get_rc_bn254, get_rounds_f, get_rounds_p, SBOX_D,
};
use crate::{
    crypto::poseidon2_params::{get_mat_diag_m_1_bls12_381, get_rc_bls12_381},
    env::internal,
    unwrap::UnwrapInfallible,
    vec, Env, Symbol, TryIntoVal, Vec, U256,
};

const CAPACITY: u32 = 1;

pub struct Poseidon2Config {
    field_type: Symbol,
    rate: u32,
    capacity: u32,
    rounds_f: u32,
    rounds_p: u32,
    m_diag: Vec<U256>,
    rc: Vec<Vec<U256>>,
}

impl Poseidon2Config {
    pub fn new(env: &Env, field_type: Symbol, rate: u32) -> Self {
        let t = rate + CAPACITY;
        if field_type == soroban_sdk_macros::internal_symbol_short!("BN254") {
            Poseidon2Config {
                field_type,
                rate,
                capacity: CAPACITY,
                rounds_f: get_rounds_f(t),
                rounds_p: get_rounds_p(t),
                m_diag: get_mat_diag_m_1_bn254(env, t),
                rc: get_rc_bn254(env, t),
            }
        } else if field_type == soroban_sdk_macros::internal_symbol_short!("BLS12_381") {
            Poseidon2Config {
                field_type,
                rate,
                capacity: CAPACITY,
                rounds_f: get_rounds_f(t),
                rounds_p: get_rounds_p(t),
                m_diag: get_mat_diag_m_1_bls12_381(env, t),
                rc: get_rc_bls12_381(env, t),
            }
        } else {
            panic!()
        }
    }
}

pub struct Poseidon2Sponge {
    env: Env,
    squeezed: bool,
    cache: Vec<U256>,
    state: Vec<U256>,
    config: Poseidon2Config,
}

impl Poseidon2Sponge {
    pub fn new(env: &Env, iv: U256, config: Poseidon2Config) -> Self {
        // the last element is reserved for capacity
        let mut state = vec![env];
        for _ in 0..config.rate {
            state.push_back(U256::from_u32(env, 0));
        }
        state.push_back(iv);

        Self {
            env: env.clone(),
            cache: Vec::new(env),
            state,
            squeezed: false,
            config,
        }
    }

    fn perform_duplex(&mut self) {
        // zero-pad the cache
        for _ in self.cache.len()..self.config.rate {
            self.cache.push_back(U256::from_u32(&self.env, 0));
        }
        // add the cache into sponge state
        for i in 0..self.config.rate {
            let elem = self
                .state
                .get_unchecked(i)
                .add(&self.cache.get_unchecked(i));
            self.state.set(i, elem);
        }

        self.state = internal::Env::poseidon2_permutation(
            &self.env,
            self.state.to_object(),
            self.config.field_type.to_symbol_val(),
            self.state.len().into(), // t = rate + capacity
            SBOX_D.into(),
            self.config.rounds_f.into(),
            self.config.rounds_p.into(),
            self.config.m_diag.to_object(),
            self.config.rc.to_object(),
        )
        .unwrap_infallible()
        .try_into_val(&self.env)
        .unwrap_infallible();
    }

    pub fn absorb(&mut self, inputs: &Vec<U256>) {
        assert!(!self.squeezed);
        let cache_len = self.cache.len();
        let inputs_len = inputs.len();

        if cache_len + inputs_len > self.config.rate {
            // if cache does not have enough room, absorb the remaining room.
            // Remain must be positive, since cache size starts at 0 (<=rate),
            // and after each iteration cache size <= rate.
            let remain = self.config.rate - cache_len;
            self.cache.append(&inputs.slice(0..remain));
            // apply the sponge permutation to compress the cache
            self.perform_duplex();
            self.cache = vec![&self.env];
            // call absorb with the leftover inputs
            self.absorb(&inputs.slice(remain..));
        } else {
            // If the cache is not full, add the input into the cache
            self.cache.append(inputs);
        }
    }

    pub fn squeeze(&mut self) -> U256 {
        assert!(!self.squeezed);
        self.perform_duplex();
        self.squeezed = true;
        self.state.get_unchecked(0)
    }
}

pub fn hash<const N: usize>(env: &Env, inputs: &[U256; N], config: Poseidon2Config) -> U256 {
    // The initial value for the capacity element initialized with `input.len() * 2^64` for Poseidon2
    let iv = U256::from_u128(env, (N as u128) << 64);
    let mut sponge = Poseidon2Sponge::new(env, iv, config);
    let input_vec = Vec::from_array(env, inputs.clone());
    sponge.absorb(&input_vec);
    sponge.squeeze()
}
