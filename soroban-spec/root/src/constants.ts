/**
 * Matches the name given by Freighter's `getNetworkDetails().network` for the network used to initialize this library.
 *
 * You can override this by setting a `SOROBAN_NETWORK_NAME` or
 * `PUBLIC_SOROBAN_NETWORK_NAME` environment variable.
 */
export const NETWORK_NAME = import.meta.env.PUBLIC_SOROBAN_NETWORK_NAME
  ?? import.meta.env.SOROBAN_NETWORK_NAME
  ?? 'FUTURENET'

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


    /**
     * The Soroban contract ID for the `abundance-toke` contract.
     * 
     * You can override this by setting a `SOROBAN_ABUNDANCE_TOKE_CONTRACT_ID` or
     * `PUBLIC_SOROBAN_ABUNDANCE_TOKE_CONTRACT_ID` environment variable.
     */
    export const CONTRACT_ID = import.meta.env.PUBLIC_SOROBAN_ABUNDANCE_TOKE_CONTRACT_ID
      ?? import.meta.env.SOROBAN_ABUNDANCE_TOKE_CONTRACT_ID
      ?? '2c6c3b8ba9923d029d8ef7eb80080384b1da32bcf0698290119fdfbf3f2a01de'