import { Contract } from 'soroban-client'

/**
 * The Soroban contract ID for the INSERT_CONTRACT_NAME_HERE contract.
 */
export const CONTRACT_ID = 'INSERT_CONTRACT_ID_HERE'

/**
 * The Soroban contract ID for the INSERT_CONTRACT_NAME_HERE contract, in hex.
 * If {@link CONTRACT_ID} is a new-style `C…` string, you will need this hex
 * version when making calls to RPC for now.
 */
export const CONTRACT_ID_HEX = new Contract(CONTRACT_ID).contractId('hex')


/**
 * The Soroban network passphrase used to initialize this library.
 */
export const NETWORK_PASSPHRASE = 'INSERT_NETWORK_PASSPHRASE_HERE'

/**
 * The Soroban RPC endpoint used to initialize this library.
 */
export const RPC_URL = 'INSERT_RPC_URL_HERE'

