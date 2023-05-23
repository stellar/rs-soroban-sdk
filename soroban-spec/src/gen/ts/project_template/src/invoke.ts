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
window.Buffer = window.Buffer || Buffer;

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
export async function invoke(args: InvokeArgs & { sign: false }): Promise<Simulation>;
export async function invoke(args: InvokeArgs & { sign: true }): Promise<TxResponse>;
export async function invoke(args: InvokeArgs & { sign?: undefined }): Promise<TxResponse>;
export async function invoke({ method, args = [], sign = false, fee = 100 }: InvokeArgs): Promise<TxResponse | Simulation> {
  const account = await getAccount()

  const contract = new SorobanClient.Contract(CONTRACT_ID)

  let tx = new SorobanClient.TransactionBuilder(account, {
    fee: fee.toString(10),
    networkPassphrase: NETWORK_PASSPHRASE,
  })
    .addOperation(contract.call(method, ...args))
    .setTimeout(SorobanClient.TimeoutInfinite)
    .build()

  if (sign) {
    let res = await invokeRpc(tx);
    console.log(res);
    return res;
  }

  const { results } = await Server.simulateTransaction(tx)
  if (!results || results[0] === undefined) {
    throw new Error("Invalid response from simulateTransaction")
  }
  return results[0]
}


async function invokeRpc(tx: Tx): Promise<TxResponse> {
  // Simulate the tx to discover the storage footprint, and update the
  // tx to include it. If you already know the storage footprint you
  // can use `addFootprint` to add it yourself, skipping this step.
  tx = await Server.prepareTransaction(tx, NETWORK_PASSPHRASE) as Tx

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
    // Wait a second
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
