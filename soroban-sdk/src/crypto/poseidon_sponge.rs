use super::poseidon_params::{
    get_mds_bls12_381, get_mds_bn254, get_rc_bls12_381, get_rc_bn254, SBOX_D,
};
use crate::{
    crypto::poseidon_params::{
        get_rounds_f_bls12_381, get_rounds_f_bn254, get_rounds_p_bls12_381, get_rounds_p_bn254,
    },
    env::internal,
    unwrap::UnwrapInfallible,
    vec, Env, Symbol, TryIntoVal, Vec, U256,
};

const CAPACITY: u32 = 1;

pub struct PoseidonConfig {
    field_type: Symbol,
    rate: u32,
    capacity: u32,
    rounds_f: u32,
    rounds_p: u32,
    mds: Vec<Vec<U256>>,
    rc: Vec<Vec<U256>>,
}

impl PoseidonConfig {
    pub fn new(env: &Env, field_type: Symbol, rate: u32) -> Self {
        let t = rate + CAPACITY;
        if field_type == soroban_sdk_macros::internal_symbol_short!("BN254") {
            PoseidonConfig {
                field_type,
                rate,
                capacity: CAPACITY,
                rounds_f: get_rounds_f_bn254(t),
                rounds_p: get_rounds_p_bn254(t),
                mds: get_mds_bn254(env, t),
                rc: get_rc_bn254(env, t),
            }
        } else if field_type == soroban_sdk_macros::internal_symbol_short!("BLS12_381") {
            PoseidonConfig {
                field_type,
                rate,
                capacity: CAPACITY,
                rounds_f: get_rounds_f_bls12_381(t),
                rounds_p: get_rounds_p_bls12_381(t),
                mds: get_mds_bls12_381(env, t),
                rc: get_rc_bls12_381(env, t),
            }
        } else {
            panic!()
        }
    }
}

pub struct PoseidonSponge {
    env: Env,
    squeezed: bool,
    cache: Vec<U256>,
    state: Vec<U256>, // state size = rate + capacity
    config: PoseidonConfig,
}

impl PoseidonSponge {
    // field_type - 'BN254' or 'BLS12_381'
    // rate - number of elements the sponge can absorb/squeeze continously at once, after which a permutation is
    // required.
    // capacity - number of extra elements the state contains, capacity provides more safety. capacity = 1 in our simplified sponge (matching circom's).
    // state.len() == rate + capacity
    // returns a Poseidon object initialize with the prepulated parameters for use, you can call hash on it.
    // limitation - single sqeeze, no duplex switching between absorb and sqeeze
    pub fn new(env: &Env, iv: U256, config: PoseidonConfig) -> Self {
        // the 0-th element is reserved for capacity
        let mut state = vec![env, iv];
        for _ in 0..config.rate {
            state.push_back(U256::from_u32(env, 0));
        }
        Self {
            env: env.clone(),
            squeezed: false,
            cache: Vec::new(env),
            state,
            config,
        }
    }

    fn perform_duplex(&mut self) {
        // zero-pad the cache
        for _ in self.cache.len()..self.config.rate {
            self.cache.push_back(U256::from_u32(&self.env, 0));
        }
        // add the cache into sponge state, leaving the 0-th element
        for i in 0..self.config.rate {
            let elem = self
                .state
                .get_unchecked(i + CAPACITY)
                .add(&self.cache.get_unchecked(i));
            self.state.set(i + CAPACITY, elem);
        }

        let d = SBOX_D;

        self.state = internal::Env::poseidon_permutation(
            &self.env,
            self.state.to_object(),
            self.config.field_type.to_symbol_val(),
            self.state.len().into(), // t = rate + capacity
            d.into(),
            self.config.rounds_f.into(),
            self.config.rounds_p.into(),
            self.config.mds.to_object(),
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

pub fn hash<const N: usize>(env: &Env, inputs: &[U256; N], config: PoseidonConfig) -> U256 {
    // The initial value for the capacity element initialized with 0 for standard Poseidon
    let iv = U256::from_u32(env, 0);
    let mut sponge = PoseidonSponge::new(env, iv, config);
    let input_vec = Vec::from_array(env, inputs.clone());
    sponge.absorb(&input_vec);
    sponge.squeeze()
}
