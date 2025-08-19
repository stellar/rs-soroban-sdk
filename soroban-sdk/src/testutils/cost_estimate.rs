use soroban_env_host::{fees::FeeConfiguration, FeeEstimate, InvocationResources, EnvBase};

use crate::{testutils::budget::Budget, Env};

pub struct CostEstimate {
    env: Env,
}

impl CostEstimate {
    pub(crate) fn new(env: Env) -> Self {
        Self { env }
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
            panic!("Invocation cost estimate is not available. Make sure invocation cost metering is enabled in the EnvTestConfig and this is called after an invocation.")
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
        // This is a snapshot of the fees as of 2024-12-11 with slight
        // adjustments for p23.
        // This has to be updated before p23 goes live with the configuration
        // used at the network upgrade time.
        let pubnet_fee_config = FeeConfiguration {
            fee_per_instruction_increment: 25,
            fee_per_disk_read_entry: 6250,
            fee_per_write_entry: 10000,
            fee_per_disk_read_1kb: 1786,
            fee_per_write_1kb: 3500,
            fee_per_historical_1kb: 16235,
            fee_per_contract_event_1kb: 10000,
            fee_per_transaction_size_1kb: 1624,
        };
        let pubnet_persistent_rent_rate_denominator = 2103;
        let pubnet_temp_rent_rate_denominator = 4206;
        // This is a bit higher than the current network fee, it's an
        // overestimate for the sake of providing a bit more conservative
        // results in case if the state grows.
        let fee_per_rent_1kb = 12000;
        self.resources().estimate_fees(
            &pubnet_fee_config,
            fee_per_rent_1kb,
            pubnet_persistent_rent_rate_denominator,
            pubnet_temp_rent_rate_denominator,
        )
    }

    /// Returns the budget object that provides the detailed CPU and memory
    /// metering information recorded thus far.
    ///
    /// The budget metering resets before every top-level contract level
    /// invocation.
    ///
    /// budget() may also be used to adjust the CPU and memory limits via the
    /// `reset_` methods.
    ///
    /// Note, that unlike `resources()`/`fee()` this will always return some
    /// value. If there was no contract call, then the resulting value will
    /// correspond to metering any environment setup that has been made thus
    /// far.
    pub fn budget(&self) -> Budget {
        Budget::new(self.env.host().budget_cloned())
    }

    /// Set stellar-core resource limits for testing.
    /// 
    /// This allows you to test contract behavior under resource constraints
    /// that match the limits enforced by stellar-core in production.
    pub fn set_stellar_limits(&self, limits: crate::testutils::budget::StellarCoreLimits) {
        self.env.set_stellar_limits(limits);
    }

    /// Get the current stellar-core resource limits.
    pub fn get_stellar_limits(&self) -> crate::testutils::budget::StellarCoreLimits {
        self.env.get_stellar_limits()
    }

    /// Check if current resource usage exceeds the set stellar-core limits.
    /// 
    /// This returns an error if any limits are exceeded, otherwise returns Ok(()).
    pub fn check_stellar_limits(&self) -> Result<(), String> {
        let resources = self.resources();
        let limits = self.get_stellar_limits();

        if let Some(limit) = limits.read_entries {
            if resources.disk_read_entries as u32 + resources.memory_read_entries > limit {
                return Err(format!(
                    "Read entries limit exceeded: {} > {}",
                    resources.disk_read_entries as u32 + resources.memory_read_entries,
                    limit
                ));
            }
        }

        if let Some(limit) = limits.write_entries {
            if resources.write_entries > limit {
                return Err(format!(
                    "Write entries limit exceeded: {} > {}",
                    resources.write_entries, limit
                ));
            }
        }

        if let Some(limit) = limits.read_bytes {
            if resources.disk_read_bytes as u32 > limit {
                return Err(format!(
                    "Read bytes limit exceeded: {} > {}",
                    resources.disk_read_bytes, limit
                ));
            }
        }

        if let Some(limit) = limits.write_bytes {
            if resources.write_bytes as u32 > limit {
                return Err(format!(
                    "Write bytes limit exceeded: {} > {}",
                    resources.write_bytes, limit
                ));
            }
        }

        if let Some(limit) = limits.contract_events_size_bytes {
            if resources.contract_events_size_bytes > limit {
                return Err(format!(
                    "Contract events size limit exceeded: {} > {}",
                    resources.contract_events_size_bytes, limit
                ));
            }
        }

        Ok(())
    }

    /// Enforce stellar-core resource limits by panicking if they are exceeded.
    /// 
    /// This is a convenience method that calls check_stellar_limits() and panics
    /// with a budget error if limits are exceeded. This provides automatic 
    /// enforcement similar to how CPU/memory budget limits work.
    pub fn enforce_stellar_limits(&self) {
        if let Err(_msg) = self.check_stellar_limits() {
            // Use escalate_error_to_panic to mimic the behavior of budget limit failures
            self.env.host().escalate_error_to_panic(
                crate::env::internal::Error::from_type_and_code(
                    crate::xdr::ScErrorType::Budget,
                    crate::xdr::ScErrorCode::ExceededLimit,
                ).into()
            );
        }
    }
}
