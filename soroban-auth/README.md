Soroban auth provides basic authentication capabilities to Soroban
contracts.

For contracts that require basic authentication capabilities this crate may
do some of the heavy lifting for supporting authentication by Stellar
accounts, ed25519 signatures, or other contracts. For contracts that require
more bespoke authentication this crate may not be suitable.

See [`verify`] for how to use.

See [`testutils`] for test utilities.

**The utilities in this crate provide no replay protection. Contracts must
provide their own mechanism suitable for replay prevention that prevents
contract invocations to be replayable if it is important they are not.**
