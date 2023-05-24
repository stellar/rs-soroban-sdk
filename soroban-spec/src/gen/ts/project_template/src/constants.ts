/**
 * The Soroban contract ID for the INSERT_CONTRACT_NAME_HERE contract.
 *
 * You can override this by setting a `SOROBAN_INSERT_SCREAMING_SNAKE_CASE_CONTRACT_NAME_HERE_CONTRACT_ID` or
 * `PUBLIC_SOROBAN_INSERT_SCREAMING_SNAKE_CASE_CONTRACT_NAME_HERE_CONTRACT_ID` environment variable.
 */
export const CONTRACT_ID = import.meta.env.PUBLIC_SOROBAN_INSERT_SCREAMING_SNAKE_CASE_CONTRACT_NAME_HERE_CONTRACT_ID
    ?? import.meta.env.SOROBAN_INSERT_SCREAMING_SNAKE_CASE_CONTRACT_NAME_HERE_CONTRACT_ID
    ?? 'INSERT_CONTRACT_ID_HERE'

/**
 * Matches the name given by Freighter's `getNetworkDetails().network` for the network used to initialize this library.
 *
 * You can override this by setting a `SOROBAN_NETWORK_NAME` or
 * `PUBLIC_SOROBAN_NETWORK_NAME` environment variable.
 */
export const NETWORK_NAME = import.meta.env.PUBLIC_SOROBAN_NETWORK_NAME
  ?? import.meta.env.SOROBAN_NETWORK_NAME
  ?? 'INSERT_NETWORK_NAME_HERE'

/**
 * The Soroban network passphrase used to initialize this library.
 *
 * You can override this by setting a `SOROBAN_NETWORK_PASSPHRASE` or
 * `PUBLIC_SOROBAN_NETWORK_PASSPHRASE` environment variable.
 */
export const NETWORK_PASSPHRASE = import.meta.env.PUBLIC_SOROBAN_NETWORK_PASSPHRASE
  ?? import.meta.env.SOROBAN_NETWORK_PASSPHRASE
  ?? 'INSERT_NETWORK_PASSPHRASE_HERE'

/**
 * The Soroban RPC endpoint used to initialize this library.
 *
 * You can override this by setting a `SOROBAN_RPC_URL` or
 * `PUBLIC_SOROBAN_RPC_URL` environment variable.
 */
export const RPC_URL = import.meta.env.PUBLIC_SOROBAN_RPC_URL
  ?? import.meta.env.SOROBAN_RPC_URL
  ?? 'INSERT_RPC_URL_HERE'

