/**
 * The Soroban contract ID for the contract-data-example contract.
 *
 * You can override this by setting a `SOROBAN_CONTRACT_DATA_EXAMPLE_CONTRACT_ID` or
 * `PUBLIC_SOROBAN_CONTRACT_DATA_EXAMPLE_CONTRACT_ID` environment variable.
 */
export const CONTRACT_ID = import.meta.env.PUBLIC_SOROBAN_CONTRACT_DATA_EXAMPLE_CONTRACT_ID
    ?? import.meta.env.SOROBAN_CONTRACT_DATA_EXAMPLE_CONTRACT_ID
    ?? 'C…'

/**
 * The Soroban network passphrase used to initialize this library.
 *
 * You can override this by setting a `SOROBAN_NETWORK_PASSPHRASE` or
 * `PUBLIC_SOROBAN_NETWORK_PASSPHRASE` environment variable.
 */
export const NETWORK_PASSPHRASE = import.meta.env.PUBLIC_SOROBAN_NETWORK_PASSPHRASE
  ?? import.meta.env.SOROBAN_NETWORK_PASSPHRASE
  ?? 'Test SDF Future Network ; October 2022'

/**
 * The Soroban RPC endpoint used to initialize this library.
 *
 * You can override this by setting a `SOROBAN_RPC_URL` or
 * `PUBLIC_SOROBAN_RPC_URL` environment variable.
 */
export const RPC_URL = import.meta.env.PUBLIC_SOROBAN_RPC_URL
  ?? import.meta.env.SOROBAN_RPC_URL
  ?? 'https://rpc-futurenet.stellar.org:443/soroban/rpc'

