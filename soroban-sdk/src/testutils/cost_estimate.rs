use soroban_env_host::{fees::FeeConfiguration, FeeEstimate, InvocationResources};

use crate::{testutils::budget::Budget, Env};

pub struct CostEstimate {
    env: Env,
}

impl CostEstimate {
    pub(crate) fn new(env: Env) -> Self {
        Self { env }
    }

    /// Enables detailed per-invocation resource cost metering.
    ///
    /// The top-level contract invocations and lifecycle operations (such as
    /// `register` or `env.deployer()` operations) will be metered and
    /// all the metering information will reset in-between them. Metering will
    /// not be reset while inside the call (e.g. if a contract calls or creates
    /// another contract, that won't reset metering).
    ///
    /// The metered resources for the last invocation can be retrieved with
    /// `resources()`, the estimated fee corresponding to these resources can be
    /// retrieved with `fee()`, and the detailed CPU and memory memory
    /// breakdown can be retrieved with `detailed_metering()`.
    ///
    /// While the resource metering may be useful for contract optimization,
    /// keep in mind that resource and fee estimation may be imprecise. Use
    /// simulation with RPC in order to get the exact resources for submitting
    /// the transactions to the network.    
    pub fn enable(&self) {
        self.env.host().enable_invocation_metering();
    }

    /// Returns the resources metered during the last top level contract
    /// invocation.    
    /// Take the return value with a grain of salt. The returned resources mostly
    /// correspond only to the operations that have happened during the host
    /// invocation, i.e. this won't try to simulate the work that happens in
    /// production scenarios (e.g. certain XDR rountrips). This also doesn't try
    /// to model resources related to the transaction size.
    ///
    /// The returned value is as useful as the preceding setup, e.g. if a test
    /// contract is used instead of a Wasm contract, all the costs related to
    /// VM instantiation and execution, as well as Wasm reads/rent bumps will be
    /// missed.    
    pub fn resources(&self) -> InvocationResources {
        if let Some(res) = self.env.host().get_last_invocation_resources() {
            res
        } else {
            panic!("Invocation cost estimate is not available. Make sure invocation cost metering is enabled and this is called after an invocation.")
        }
    }

    /// Estimates the fee for the last invocation's resources, i.e. the
    /// resources returned by `resources()`.
    ///
    /// The fees are computed using the snapshot of the Stellar Pubnet fees made
    /// on 2024-12-11.
    ///
    /// Take the return value with a grain of salt as both the resource estimate
    /// and the fee rates may be imprecise.
    ///
    /// The returned value is as useful as the preceding setup, e.g. if a test
    /// contract is used instead of a Wasm contract, all the costs related to
    /// VM instantiation and execution, as well as Wasm reads/rent bumps will be
    /// missed.    
    pub fn fee(&self) -> FeeEstimate {
        // This is a snapshot of the fees as of 2024-12-11.
        let pubnet_fee_config = FeeConfiguration {
            fee_per_instruction_increment: 25,
            fee_per_read_entry: 6250,
            fee_per_write_entry: 10000,
            fee_per_read_1kb: 1786,
            // This is a bit higher than the current network fee, it's an
            // overestimate for the sake of providing a bit more conservative
            // results in case if the state grows.
            fee_per_write_1kb: 12000,
            fee_per_historical_1kb: 16235,
            fee_per_contract_event_1kb: 10000,
            fee_per_transaction_size_1kb: 1624,
        };
        let pubnet_persistent_rent_rate_denominator = 2103;
        let pubnet_temp_rent_rate_denominator = 4206;

        self.resources().estimate_fees(
            &pubnet_fee_config,
            pubnet_persistent_rent_rate_denominator,
            pubnet_temp_rent_rate_denominator,
        )
    }

    /// Returns the detailed CPU and memory metering information recorded thus
    /// far.
    ///
    /// The metering resets before every top-level contract level invocation.
    ///
    /// Note, that unlike `resources()`/`fee()` this will always return some
    /// value. If there was no contract call, then the resulting value will
    /// correspond to metering any environment setup that has been made thus
    /// far.
    pub fn detailed_metering(&self) -> Budget {
        Budget::new(self.env.host().budget_cloned())
    }
}
