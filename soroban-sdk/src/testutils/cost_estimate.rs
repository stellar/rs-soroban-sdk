use soroban_env_host::{
    fees::FeeConfiguration, FeeEstimate, InvocationResourceLimits, InvocationResources,
};

use crate::{testutils::budget::Budget, Env};

/// CostEstimate measures the resources that contract invocations use in tests,
/// estimates the fees for them, and configures the limits that invocations must
/// stay within.
///
/// ### Limits
///
/// Two independent limits apply to contract invocations in tests, and both need
/// changing to run an invocation with no limits:
///
/// 1. The CPU instruction and memory budget, which is metered while a contract
///    executes and is enforced the moment it is exceeded, stopping the
///    invocation with `Error(Budget, ExceededLimit)`. It is configured with the
///    [`Budget`] `reset_*` functions, reachable via [`budget()`][Self::budget].
///
/// 2. The invocation resource limits, which are checked once a top-level
///    invocation completes and cover the full set of resources that a network
///    charges a transaction for: instructions, memory, ledger entries read and
///    written, bytes read and written, event sizes, and data key and entry
///    sizes. Exceeding any of them panics with a message naming the limits
///    exceeded. They are configured with
///    [`enforce_resource_limits()`][Self::enforce_resource_limits] and
///    [`disable_resource_limits()`][Self::disable_resource_limits].
///
/// An [`Env::default()`][Env::default] starts with the budget at its default
/// limits, listed in [`Budget`], and with the Mainnet resource limits
/// ([`InvocationResourceLimits::mainnet()`][NetworkInvocationResourceLimits::mainnet])
/// enforced. Note that the default budget instruction limit is lower than the
/// Mainnet instruction limit, so an invocation heavy enough to exceed it fails
/// on the budget before the resource limit check happens.
///
/// ### Examples
///
/// Run invocations with no limits at all, by disabling the resource limit
/// checks and lifting the budget limits:
///
/// ```
/// use soroban_sdk::{contract, contractimpl, Env};
///
/// #[contract]
/// pub struct Contract;
///
/// #[contractimpl]
/// impl Contract {
///     pub fn f() {
///         // ... resource heavy code
///     }
/// }
///
/// #[test]
/// fn test() {
/// # }
/// # #[cfg(feature = "testutils")]
/// # fn main() {
///     let env = Env::default();
///
///     // Turn off the check that happens after each invocation.
///     env.cost_estimate().disable_resource_limits();
///     // Lift the CPU instruction and memory limits enforced during execution.
///     env.cost_estimate().budget().reset_unlimited();
///
///     let contract_id = env.register(Contract, ());
///     let client = ContractClient::new(&env, &contract_id);
///
///     client.f();
/// }
/// # #[cfg(not(feature = "testutils"))]
/// # fn main() { }
/// ```
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
    /// The fees are computed using a snapshot of the Stellar Mainnet fees made
    /// on 2026-07-10. Because the fees are hardcoded rather than pulled
    /// dynamically, they may drift from the live network over time; the current
    /// values can be checked via `stellar network settings --network mainnet`
    /// or on Stellar Lab: <https://lab.stellar.org/network-limits>. The one
    /// exception is the per-1KB storage rent rate, which is a deliberate
    /// conservative overestimate rather than the snapshot value, so storage
    /// rent estimates may be higher than the live network charges.
    ///
    /// Take the return value with a grain of salt as both the resource estimate
    /// and the fee rates may be imprecise.
    ///
    /// The returned value is as useful as the preceding setup, e.g. if a test
    /// contract is used instead of a Wasm contract, all the costs related to
    /// VM instantiation and execution, as well as Wasm reads/rent bumps will be
    /// missed.
    pub fn fee(&self) -> FeeEstimate {
        // This is a snapshot of the Stellar Mainnet fees as of 2026-07-10.
        // Refresh it with the values from `stellar network settings --network
        // mainnet` (or <https://lab.stellar.org/network-limits>) when it drifts.
        let pubnet_fee_config = FeeConfiguration {
            fee_per_instruction_increment: 7,
            fee_per_disk_read_entry: 1563,
            fee_per_write_entry: 2500,
            fee_per_disk_read_1kb: 447,
            fee_per_write_1kb: 875,
            fee_per_historical_1kb: 4059,
            fee_per_contract_event_1kb: 5000,
            fee_per_transaction_size_1kb: 406,
        };
        let pubnet_persistent_rent_rate_denominator = 1215;
        let pubnet_temp_rent_rate_denominator = 2430;
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
    /// invocation, so the costs read describe the last invocation only. See
    /// [`Budget::cpu_instruction_cost`], [`Budget::memory_bytes_cost`], and
    /// [`Budget::tracker`] for reading them, and [`Budget::print`] or the
    /// [`Display`][core::fmt::Display] impl for printing all of them.
    ///
    /// budget() may also be used to adjust the CPU and memory limits via the
    /// `reset_` methods, i.e. [`Budget::reset_limits`],
    /// [`Budget::reset_unlimited`], and [`Budget::reset_default`]. Unlike the
    /// metering the limits are not reset between invocations, and stay in
    /// effect until changed by one of those methods.
    ///
    /// The budget limits are enforced while a contract executes, and are
    /// independent of the invocation resource limits that
    /// [`enforce_resource_limits()`][Self::enforce_resource_limits] and
    /// [`disable_resource_limits()`][Self::disable_resource_limits] configure,
    /// which are checked once an invocation completes.
    ///
    /// Note, that unlike `resources()`/`fee()` this will always return some
    /// value. If there was no contract call, then the resulting value will
    /// correspond to metering any environment setup that has been made thus
    /// far.
    ///
    /// ### Examples
    ///
    /// ```
    /// use soroban_sdk::{contract, contractimpl, Env};
    ///
    /// #[contract]
    /// pub struct Contract;
    ///
    /// #[contractimpl]
    /// impl Contract {
    ///     pub fn f() {
    ///         // ... code
    ///     }
    /// }
    ///
    /// #[test]
    /// fn test() {
    /// # }
    /// # #[cfg(feature = "testutils")]
    /// # fn main() {
    ///     let env = Env::default();
    ///     let contract_id = env.register(Contract, ());
    ///     let client = ContractClient::new(&env, &contract_id);
    ///
    ///     client.f();
    ///
    ///     // The costs are for the invocation above only.
    ///     let budget = env.cost_estimate().budget();
    ///     println!("cpu instructions: {}", budget.cpu_instruction_cost());
    ///     println!("memory bytes: {}", budget.memory_bytes_cost());
    ///
    ///     // Print the limits, and the cost of every cost type.
    ///     budget.print();
    /// }
    /// # #[cfg(not(feature = "testutils"))]
    /// # fn main() { }
    /// ```
    pub fn budget(&self) -> Budget {
        Budget::new(self.env.host().budget_cloned())
    }

    /// Enforces custom resource limits for contract invocations in tests.
    ///
    /// When limit enforcement is enabled, for every contract invocation the
    /// resource usage is checked against the provided limits, and if any of the
    /// limits is exceeded, the contract invocation will result in a panic
    /// that indicates which limits were exceeded.
    ///
    /// The check happens once a top-level invocation completes, using the
    /// resources that [`resources()`][Self::resources] reports. The limits are
    /// checked for every subsequent invocation until they are changed by
    /// another call to this function, or removed by
    /// [`disable_resource_limits()`][Self::disable_resource_limits].
    ///
    /// Limit enforcement is meant to provide an early warning sign that a
    /// contract might be too resource heavy to run on a real network. If the
    /// high resource usage is intentional and expected (e.g. for
    /// experimentation), disable the enforcement via
    /// [`disable_resource_limits()`][Self::disable_resource_limits].
    ///
    /// By default,
    /// [`InvocationResourceLimits::mainnet()`][NetworkInvocationResourceLimits::mainnet]
    /// limits are enforced. Prefer starting from those limits and changing the
    /// fields of interest, so the limits that are not the focus of the test
    /// stay realistic.
    ///
    /// This does not change the CPU instruction and memory budget, which is
    /// metered while a contract executes and enforced independently of these
    /// limits. Setting `instructions` or `mem_bytes` above the budget's limits
    /// has no effect on its own, because execution stops with
    /// `Error(Budget, ExceededLimit)` at the budget limit before the check here
    /// happens. Raise the budget limits as well via
    /// [`budget()`][Self::budget] and [`Budget::reset_limits`].
    ///
    /// ### Examples
    ///
    /// Enforce a limit stricter than Mainnet's, to check that a contract has
    /// headroom:
    ///
    /// ```
    /// use soroban_env_host::InvocationResourceLimits;
    /// use soroban_sdk::testutils::cost_estimate::NetworkInvocationResourceLimits;
    /// use soroban_sdk::{contract, contractimpl, Env};
    ///
    /// #[contract]
    /// pub struct Contract;
    ///
    /// #[contractimpl]
    /// impl Contract {
    ///     pub fn f() {
    ///         // ... code
    ///     }
    /// }
    ///
    /// #[test]
    /// fn test() {
    /// # }
    /// # #[cfg(feature = "testutils")]
    /// # fn main() {
    ///     let env = Env::default();
    ///
    ///     let mut limits = InvocationResourceLimits::mainnet();
    ///     limits.instructions = 50_000_000;
    ///     env.cost_estimate().enforce_resource_limits(limits);
    ///
    ///     let contract_id = env.register(Contract, ());
    ///     let client = ContractClient::new(&env, &contract_id);
    ///
    ///     client.f(); // Panics if any of the limits are exceeded.
    /// }
    /// # #[cfg(not(feature = "testutils"))]
    /// # fn main() { }
    /// ```
    ///
    /// Enforce a limit higher than the budget's default limits, raising the
    /// budget limits too so that the budget does not stop execution first:
    ///
    /// ```
    /// use soroban_env_host::InvocationResourceLimits;
    /// use soroban_sdk::testutils::cost_estimate::NetworkInvocationResourceLimits;
    /// use soroban_sdk::{contract, contractimpl, Env};
    ///
    /// #[contract]
    /// pub struct Contract;
    ///
    /// #[contractimpl]
    /// impl Contract {
    ///     pub fn f() {
    ///         // ... resource heavy code
    ///     }
    /// }
    ///
    /// #[test]
    /// fn test() {
    /// # }
    /// # #[cfg(feature = "testutils")]
    /// # fn main() {
    ///     let env = Env::default();
    ///
    ///     let mut limits = InvocationResourceLimits::mainnet();
    ///     limits.instructions = 1_000_000_000;
    ///
    ///     // Raise the budget to match, otherwise the invocation fails at the
    ///     // budget's default instruction limit, which is lower.
    ///     env.cost_estimate()
    ///         .budget()
    ///         .reset_limits(limits.instructions as u64, limits.mem_bytes as u64);
    ///     env.cost_estimate().enforce_resource_limits(limits);
    ///
    ///     let contract_id = env.register(Contract, ());
    ///     let client = ContractClient::new(&env, &contract_id);
    ///
    ///     client.f();
    /// }
    /// # #[cfg(not(feature = "testutils"))]
    /// # fn main() { }
    /// ```
    pub fn enforce_resource_limits(&self, limits: InvocationResourceLimits) {
        self.env
            .host()
            .set_invocation_resource_limits(Some(limits))
            .unwrap();
    }

    /// Disables resource limit enforcement for contract invocations in tests.
    ///
    /// This may be useful for the experimental contracts that are still being
    /// optimized.
    ///
    /// Only the resource limit check that happens after each top-level
    /// invocation is disabled. The CPU instruction and memory budget is metered
    /// while a contract executes and is enforced independently of these limits,
    /// and it keeps the limits it already has. An invocation that exceeds them
    /// still fails with `Error(Budget, ExceededLimit)`. To run invocations with
    /// no limits at all, lift the budget limits as well via
    /// [`budget()`][Self::budget] and [`Budget::reset_unlimited`].
    ///
    /// Both settings persist for the life of the [`Env`], and are undone
    /// separately: resource limit checks come back with
    /// [`enforce_resource_limits()`][Self::enforce_resource_limits], and the
    /// budget limits go back to their defaults with [`Budget::reset_default`].
    ///
    /// Resources are still measured, and available via
    /// [`resources()`][Self::resources] and [`fee()`][Self::fee]; they are just
    /// no longer checked against any limits.
    ///
    /// ### Examples
    ///
    /// ```
    /// use soroban_sdk::{contract, contractimpl, Env};
    ///
    /// #[contract]
    /// pub struct Contract;
    ///
    /// #[contractimpl]
    /// impl Contract {
    ///     pub fn f() {
    ///         // ... resource heavy code
    ///     }
    /// }
    ///
    /// #[test]
    /// fn test() {
    /// # }
    /// # #[cfg(feature = "testutils")]
    /// # fn main() {
    ///     let env = Env::default();
    ///
    ///     env.cost_estimate().disable_resource_limits();
    ///     // The budget is a separate limit, lift it as well.
    ///     env.cost_estimate().budget().reset_unlimited();
    ///
    ///     let contract_id = env.register(Contract, ());
    ///     let client = ContractClient::new(&env, &contract_id);
    ///
    ///     client.f(); // Does not panic, no matter the resources used.
    ///
    ///     // Resources are still measured, just not limited.
    ///     println!("{:?}", env.cost_estimate().resources());
    /// }
    /// # #[cfg(not(feature = "testutils"))]
    /// # fn main() { }
    /// ```
    pub fn disable_resource_limits(&self) {
        self.env
            .host()
            .set_invocation_resource_limits(None)
            .unwrap();
    }
}

/// Predefined network invocation resource limits.
pub trait NetworkInvocationResourceLimits {
    fn mainnet() -> Self;
}

impl NetworkInvocationResourceLimits for InvocationResourceLimits {
    /// Returns the invocation resource limits used on Stellar Mainnet.
    ///
    /// These values are a snapshot of the Mainnet network settings as of
    /// 2026-07-10; they are hardcoded rather than pulled dynamically, so
    /// updating the SDK is necessary to pick up the most recent values. The
    /// current values can be checked via `stellar network settings --network
    /// mainnet` or on Stellar Lab: <https://lab.stellar.org/network-limits>.
    fn mainnet() -> Self {
        InvocationResourceLimits {
            instructions: 400_000_000,
            mem_bytes: 41943040,
            disk_read_entries: 200,
            write_entries: 200,
            ledger_entries: 400,
            disk_read_bytes: 200000,
            write_bytes: 132096,
            contract_events_size_bytes: 16384,
            max_contract_data_key_size_bytes: 250,
            max_contract_data_entry_size_bytes: 65536,
            max_contract_code_entry_size_bytes: 131072,
        }
    }
}
