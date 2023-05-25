import {
  isConnected,
  getPublicKey,
  signTransaction,
} from "@stellar/freighter-api";
import * as SorobanClient from 'soroban-client'
import { Buffer } from "buffer";
import type { Account, Memo, MemoType, Operation, Transaction } from 'soroban-client';
import { NETWORK_NAME, NETWORK_PASSPHRASE, CONTRACT_ID } from './constants'
import { Server } from './server'


export type Tx = Transaction<Memo<MemoType>, Operation[]>

export type Simulation = NonNullable<SorobanClient.SorobanRpc.SimulateTransactionResponse['results']>[0]

export type TxResponse = SorobanClient.SorobanRpc.GetTransactionResponse;

export type InvokeArgs = {
  method: string
  args?: any[]
  sign?: boolean
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
 * Invoke a method on the INSERT_CONTRACT_NAME_HERE contract.
 *
 * Uses Freighter to determine the current user and sign the transaction.
 *
 * @param {string} obj.method - The method to invoke.
 * @param {any[]} obj.args - The arguments to pass to the method.
 * @param {boolean} obj.sign - Whether to sign the transaction with Freighter.
 * @returns The transaction response, or the simulation result if `sign` is false.
 */
export async function invoke({ method, args = [], fee = 100 }: InvokeArgs): Promise<(TxResponse & { xdr: string }) | Simulation> {
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
  if (auths?.length > 0) {
    if (auths.length > 1) {
      throw new NotImplementedError("Multiple auths not yet supported")
    }

    const auth = SorobanClient.xdr.ContractAuth.fromXDR(auths[0], 'base64')

    if (auth.addressWithNonce() !== undefined) {
      throw new NotImplementedError(
        `This transaction needs to be signed by ${auth.addressWithNonce()
        }; how do we do that?`
      )
    }

    const raw = await invokeRpc(tx, simulated);
    return {
      ...raw,
      xdr: raw.resultXdr,
    };
  }

  const { results } = await Server.simulateTransaction(tx)
  if (!results || results[0] === undefined) {
    throw new Error("Invalid response from simulateTransaction")
  }
  return results[0]
}


async function invokeRpc(tx: Tx, simulation: SorobanClient.SorobanRpc.SimulateTransactionResponse): Promise<TxResponse> {
  // Simulate the tx to discover the storage footprint, and update the
  // tx to include it. If you already know the storage footprint you
  // can use `addFootprint` to add it yourself, skipping this step.
  tx = SorobanClient.assembleTransaction(tx, NETWORK_PASSPHRASE, simulation) as Tx

  // sign with Freighter
  const signed = await signTransaction(tx.toXDR(), {
    network: NETWORK_NAME,
    networkPassphrase: NETWORK_PASSPHRASE,
  })

  // re-assemble with signed tx
  tx = SorobanClient.TransactionBuilder.fromXDR(
    signed,
    NETWORK_PASSPHRASE
  ) as Tx

  const sendTransactionResponse = await Server.sendTransaction(tx);
  let getTransactionResponse = await Server.getTransaction(sendTransactionResponse.hash);

  const secondsToWait = 10
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
