use super::poseidon2_params::{get_mat_diag4_m_1, get_rc4, ROUNDS_F, ROUNDS_P, SBOX_D, T};
use crate::{env::internal, unwrap::UnwrapInfallible, vec, Env, Symbol, TryIntoVal, Vec, U256};

const RATE: u32 = 3;

pub struct Poseidon2Sponge {
    env: Env,
    field: Symbol,
    rate: u32,
    squeezed: bool,
    cache: Vec<U256>,
    state: Vec<U256>,
    m_diag: Vec<U256>,
    rc: Vec<Vec<U256>>,
}

impl Poseidon2Sponge {
    pub fn new(env: &Env, iv: U256, field: Symbol) -> Poseidon2Sponge {
        let mut state = vec![env];
        for _ in 0..RATE {
            state.push_back(U256::from_u32(env, 0));
        }
        state.push_back(iv);

        // Get parameters from poseidon2_params
        let m_diag = get_mat_diag4_m_1(env);
        let rc = get_rc4(env);

        Poseidon2Sponge {
            env: env.clone(),
            field,
            rate: RATE,
            cache: Vec::new(env),
            state,
            squeezed: false,
            m_diag,
            rc,
        }
    }

    fn perform_duplex(&mut self) {
        // zero-pad the cache
        for _ in self.cache.len()..RATE {
            self.cache.push_back(U256::from_u32(&self.env, 0));
        }
        // add the cache into sponge state
        for i in 0..RATE {
            let elem = self
                .state
                .get_unchecked(i)
                .add(&self.cache.get_unchecked(i));
            self.state.set(i, elem);
        }

        // t = rate + 1 = 4
        let t = T as u32;
        let d = SBOX_D;
        let rounds_f = ROUNDS_F;
        let rounds_p = ROUNDS_P;

        self.state = internal::Env::poseidon2_permutation(
            &self.env,
            self.state.to_object(),
            self.field.to_symbol_val(),
            t.into(),
            d.into(),
            rounds_f.into(),
            rounds_p.into(),
            self.m_diag.to_object(),
            self.rc.to_object(),
        )
        .unwrap_infallible()
        .try_into_val(&self.env)
        .unwrap_infallible();
    }

    pub fn absorb(&mut self, input: U256) {
        assert!(!self.squeezed);
        if self.cache.len() == RATE {
            // If we're absorbing, and the cache is full, apply the sponge permutation to compress the cache
            self.perform_duplex();
            self.cache = vec![&self.env, input];
        } else {
            // If we're absorbing, and the cache is not full, add the input into the cache
            self.cache.push_back(input);
        }
    }

    pub fn squeeze(&mut self) -> U256 {
        assert!(!self.squeezed);
        // If we're in absorb mode, apply sponge permutation to compress the cache.
        self.perform_duplex();
        self.squeezed = true;

        // Pop one item off the top of the permutation and return it.
        self.state.get_unchecked(0)
    }
}
