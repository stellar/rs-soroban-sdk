import freighter from "@stellar/freighter-api";
// working around ESM compatibility issues
const {
  isConnected,
  getPublicKey,
  signTransaction,
} = freighter;
import * as SorobanClient from 'soroban-client'
import type { Account, Memo, MemoType, Operation, Transaction } from 'soroban-client';
import { NETWORK_PASSPHRASE, CONTRACT_ID } from './constants.js'
import { Server } from './server.js'

export type Tx = Transaction<Memo<MemoType>, Operation[]>

export type Simulation = NonNullable<SorobanClient.SorobanRpc.SimulateTransactionResponse['results']>[0]

export type TxResponse = SorobanClient.SorobanRpc.GetTransactionResponse;

export type InvokeArgs = {
  method: string
  args?: any[]
  signAndSend?: boolean
  fee?: number
}

/**
 * Get account details from the Soroban network for the publicKey currently
 * selected in Freighter. If not connected to Freighter, throws errors. Will
 * pop up Freighter's modal to request user permissions, if this hasn't been
 * done already.
 */
export async function getAccount(): Promise<Account> {
  if (!await isConnected()) {
    throw new Error('Freighter not connected')
  }
  const publicKey = await getPublicKey()
  if (!publicKey) {
    throw new Error('Freighter not initialized')
  }
  return await Server.getAccount(publicKey)
}

export class NotImplementedError extends Error { }

/**
 * Invoke a method on the contract-data-example contract.
 *
 * Uses Freighter to determine the current user and if necessary sign the transaction.
 *
 * @param {string} obj.method - The method to invoke.
 * @param {any[]} obj.args - The arguments to pass to the method.
 * @param {boolean} obj.signAndSend - Whether to sign and send the transaction, or just simulate it. Unless the method requires authentication.
 * @param {number} obj.fee - The fee to pay for the transaction.
 * @returns The transaction response, or the simulation result if signing isn't required.
 */
export async function invoke({ method, args = [], fee = 100, signAndSend = false }: InvokeArgs): Promise<(TxResponse & { xdr: string }) | Simulation> {
  const account = await getAccount()

  const contract = new SorobanClient.Contract(CONTRACT_ID)

  let tx = new SorobanClient.TransactionBuilder(account, {
    fee: fee.toString(10),
    networkPassphrase: NETWORK_PASSPHRASE,
  })
    .addOperation(contract.call(method, ...args))
    .setTimeout(SorobanClient.TimeoutInfinite)
    .build()

  const simulated = await Server.simulateTransaction(tx)
  const auths = simulated.results?.[0]?.auth
  // is it possible for `auths` to be present but empty? Probably not, but let's be safe.
  if (signAndSend) {
    if (auths.length > 1) {
      throw new NotImplementedError("Multiple auths not yet supported")
    }

    const auth = SorobanClient.xdr.ContractAuth.fromXDR(auths[0], 'base64')

    if (auth.addressWithNonce() !== undefined) {
      throw new NotImplementedError(
        `This transaction needs to be signed by ${auth.addressWithNonce()
        }; Not yet supported`
      )
    }

    tx = await signTx(
      SorobanClient.assembleTransaction(tx, NETWORK_PASSPHRASE, simulated) as Tx
    );

    const raw = await sendTx(tx);
    return {
      ...raw,
      xdr: raw.resultXdr,
    };
  }

  const { results } = simulated
  if (!results || results[0] === undefined) {
    if (simulated.error) {
      throw new Error(simulated.error as unknown as string)
    }
    throw new Error(`Invalid response from simulateTransaction:\n{simulated}`)
  }
  return results[0]
}

/**
 * Sign a transaction with Freighter and return the fully-reconstructed
 * transaction ready to send with {@link sendTx}.
 *
 * If you need to construct a transaction yourself rather than using `invoke`
 * or one of the exported contract methods, you may want to use this function
 * to sign the transaction with Freighter.
 */
export async function signTx(tx: Tx): Promise<Tx> {
  const signed = await signTransaction(tx.toXDR(), {
    networkPassphrase: NETWORK_PASSPHRASE,
  })

  return SorobanClient.TransactionBuilder.fromXDR(
    signed,
    NETWORK_PASSPHRASE
  ) as Tx
}

/**
 * Send a transaction to the Soroban network.
 *
 * Wait `secondsToWait` seconds for the transaction to complete (default: 10).
 *
 * If you need to construct or sign a transaction yourself rather than using
 * `invoke` or one of the exported contract methods, you may want to use this
 * function for its timeout/`secondsToWait` logic, rather than implementing
 * your own.
 */
export async function sendTx(tx: Tx, secondsToWait = 10): Promise<TxResponse> {
  const sendTransactionResponse = await Server.sendTransaction(tx);
  let getTransactionResponse = await Server.getTransaction(sendTransactionResponse.hash);

  const waitUntil = new Date((Date.now() + secondsToWait * 1000)).valueOf()

  let waitTime = 1000;
  let exponentialFactor = 1.5

  while ((Date.now() < waitUntil) && getTransactionResponse.status === "NOT_FOUND") {
    // Wait a beat
    await new Promise(resolve => setTimeout(resolve, waitTime))
    /// Exponential backoff
    waitTime = waitTime * exponentialFactor;
    // See if the transaction is complete
    getTransactionResponse = await Server.getTransaction(sendTransactionResponse.hash)
  }

  if (getTransactionResponse.status === "NOT_FOUND") {
    console.log(
      `Waited ${secondsToWait} seconds for transaction to complete, but it did not. Returning anyway. Check the transaction status manually. Info: ${JSON.stringify(sendTransactionResponse, null, 2)}`
    )
  }

  return getTransactionResponse
}
