import * as SorobanClient from 'soroban-client';
import { xdr } from 'soroban-client';
import { Buffer } from "buffer";
import { scValToJs } from './convert';
import { invoke, InvokeArgs } from './invoke';

export * from './constants'
export * from './server'
export * from './invoke'

export type u32 = number;
export type i32 = number;
export type u64 = bigint;
export type i64 = bigint;
export type u128 = bigint;
export type i128 = bigint;
export type u256 = bigint;
export type i256 = bigint;
export type Address = string;

window.Buffer = window.Buffer || Buffer;

export async function initialize({admin, decimal, name, symbol}: {admin: Address, decimal: u32, name: Buffer, symbol: Buffer}) {
    let invokeArgs: InvokeArgs = {method: 'initialize', args: [((i) =>SorobanClient.Address.fromString(i).toScVal())(admin), ((i) =>xdr.ScVal.scvU32(i))(decimal), ((i) =>xdr.ScVal.scvBytes(i))(name), ((i) =>xdr.ScVal.scvBytes(i))(symbol)]};
    // @ts-ignore Type does exist
    const response = await invoke(invokeArgs);
    
}

export async function allowance({from, spender}: {from: Address, spender: Address}): Promise<i128> {
    let invokeArgs: InvokeArgs = {method: 'allowance', args: [((i) =>SorobanClient.Address.fromString(i).toScVal())(from), ((i) =>SorobanClient.Address.fromString(i).toScVal())(spender)]};
    // @ts-ignore Type does exist
    const response = await invoke(invokeArgs);
    return scValToJs(SorobanClient.xdr.ScVal.fromXDR(Buffer.from(response.xdr, 'base64'))) as i128

    
}

export async function incr_allow({from, spender, amount}: {from: Address, spender: Address, amount: i128}) {
    let invokeArgs: InvokeArgs = {method: 'incr_allow', args: [((i) =>SorobanClient.Address.fromString(i).toScVal())(from), ((i) =>SorobanClient.Address.fromString(i).toScVal())(spender), ((i) =>xdr.ScVal.scvI128(xdr.Int128Parts.fromXDR(i.toString(16), 'hex')))(amount)]};
    // @ts-ignore Type does exist
    const response = await invoke(invokeArgs);
    
}

export async function decr_allow({from, spender, amount}: {from: Address, spender: Address, amount: i128}) {
    let invokeArgs: InvokeArgs = {method: 'decr_allow', args: [((i) =>SorobanClient.Address.fromString(i).toScVal())(from), ((i) =>SorobanClient.Address.fromString(i).toScVal())(spender), ((i) =>xdr.ScVal.scvI128(xdr.Int128Parts.fromXDR(i.toString(16), 'hex')))(amount)]};
    // @ts-ignore Type does exist
    const response = await invoke(invokeArgs);
    
}

/**
 * @signme
 */
export async function balance({id}: {id: Address}): Promise<i128> {
    let invokeArgs: InvokeArgs = {"sign": true, method: 'balance', args: [((i) =>SorobanClient.Address.fromString(i).toScVal())(id)]};
    // @ts-ignore Type does exist
    const response = await invoke(invokeArgs);
    return scValToJs(SorobanClient.xdr.ScVal.fromXDR(Buffer.from(response.xdr, 'base64'))) as i128

    
}

export async function spendable({id}: {id: Address}): Promise<i128> {
    let invokeArgs: InvokeArgs = {method: 'spendable', args: [((i) =>SorobanClient.Address.fromString(i).toScVal())(id)]};
    // @ts-ignore Type does exist
    const response = await invoke(invokeArgs);
    return scValToJs(SorobanClient.xdr.ScVal.fromXDR(Buffer.from(response.xdr, 'base64'))) as i128

    
}

export async function authorized({id}: {id: Address}): Promise<boolean> {
    let invokeArgs: InvokeArgs = {method: 'authorized', args: [((i) =>SorobanClient.Address.fromString(i).toScVal())(id)]};
    // @ts-ignore Type does exist
    const response = await invoke(invokeArgs);
    return scValToJs(SorobanClient.xdr.ScVal.fromXDR(Buffer.from(response.xdr, 'base64'))) as boolean

    
}

export async function xfer({from, to, amount}: {from: Address, to: Address, amount: i128}) {
    let invokeArgs: InvokeArgs = {method: 'xfer', args: [((i) =>SorobanClient.Address.fromString(i).toScVal())(from), ((i) =>SorobanClient.Address.fromString(i).toScVal())(to), ((i) =>xdr.ScVal.scvI128(xdr.Int128Parts.fromXDR(i.toString(16), 'hex')))(amount)]};
    // @ts-ignore Type does exist
    const response = await invoke(invokeArgs);
    
}

export async function xfer_from({spender, from, to, amount}: {spender: Address, from: Address, to: Address, amount: i128}) {
    let invokeArgs: InvokeArgs = {method: 'xfer_from', args: [((i) =>SorobanClient.Address.fromString(i).toScVal())(spender), ((i) =>SorobanClient.Address.fromString(i).toScVal())(from), ((i) =>SorobanClient.Address.fromString(i).toScVal())(to), ((i) =>xdr.ScVal.scvI128(xdr.Int128Parts.fromXDR(i.toString(16), 'hex')))(amount)]};
    // @ts-ignore Type does exist
    const response = await invoke(invokeArgs);
    
}

export async function burn({from, amount}: {from: Address, amount: i128}) {
    let invokeArgs: InvokeArgs = {method: 'burn', args: [((i) =>SorobanClient.Address.fromString(i).toScVal())(from), ((i) =>xdr.ScVal.scvI128(xdr.Int128Parts.fromXDR(i.toString(16), 'hex')))(amount)]};
    // @ts-ignore Type does exist
    const response = await invoke(invokeArgs);
    
}

export async function burn_from({spender, from, amount}: {spender: Address, from: Address, amount: i128}) {
    let invokeArgs: InvokeArgs = {method: 'burn_from', args: [((i) =>SorobanClient.Address.fromString(i).toScVal())(spender), ((i) =>SorobanClient.Address.fromString(i).toScVal())(from), ((i) =>xdr.ScVal.scvI128(xdr.Int128Parts.fromXDR(i.toString(16), 'hex')))(amount)]};
    // @ts-ignore Type does exist
    const response = await invoke(invokeArgs);
    
}

export async function clawback({admin, from, amount}: {admin: Address, from: Address, amount: i128}) {
    let invokeArgs: InvokeArgs = {method: 'clawback', args: [((i) =>SorobanClient.Address.fromString(i).toScVal())(admin), ((i) =>SorobanClient.Address.fromString(i).toScVal())(from), ((i) =>xdr.ScVal.scvI128(xdr.Int128Parts.fromXDR(i.toString(16), 'hex')))(amount)]};
    // @ts-ignore Type does exist
    const response = await invoke(invokeArgs);
    
}

export async function set_auth({admin, id, authorize}: {admin: Address, id: Address, authorize: boolean}) {
    let invokeArgs: InvokeArgs = {method: 'set_auth', args: [((i) =>SorobanClient.Address.fromString(i).toScVal())(admin), ((i) =>SorobanClient.Address.fromString(i).toScVal())(id), ((i) =>xdr.ScVal.scvBool(i))(authorize)]};
    // @ts-ignore Type does exist
    const response = await invoke(invokeArgs);
    
}

/**
 * Mint tokens to the `to` Address
 * This function is only callable by the contract administrator.
 * 
 * # Arguments
 * * `admin` - The Address of the contract administrator
 * * `to` - The Address to mint tokens to
 * * `amount` - The amount of smallest possible token fractions to mint (remember to multiply
 * by `decimals` to get the actual amount of tokens to mint)
 */
export async function mint({admin, to, amount}: {admin: Address, to: Address, amount: i128}) {
    let invokeArgs: InvokeArgs = {method: 'mint', args: [((i) =>SorobanClient.Address.fromString(i).toScVal())(admin), ((i) =>SorobanClient.Address.fromString(i).toScVal())(to), ((i) =>xdr.ScVal.scvI128(xdr.Int128Parts.fromXDR(i.toString(16), 'hex')))(amount)]};
    // @ts-ignore Type does exist
    const response = await invoke(invokeArgs);
    
}

/**
 * Mint one token to yourself.
 * 
 * # Arguments
 * * `to` - The Address to mint a token to. Must also be the signer/caller.
 */
export async function token_plz({to}: {to: Address}) {
    let invokeArgs: InvokeArgs = {method: 'token_plz', args: [((i) =>SorobanClient.Address.fromString(i).toScVal())(to)]};
    // @ts-ignore Type does exist
    const response = await invoke(invokeArgs);
    
}

export async function set_admin({admin, new_admin}: {admin: Address, new_admin: Address}) {
    let invokeArgs: InvokeArgs = {method: 'set_admin', args: [((i) =>SorobanClient.Address.fromString(i).toScVal())(admin), ((i) =>SorobanClient.Address.fromString(i).toScVal())(new_admin)]};
    // @ts-ignore Type does exist
    const response = await invoke(invokeArgs);
    
}

export async function decimals(): Promise<u32> {
    let invokeArgs: InvokeArgs = {method: 'decimals', args: []};
    // @ts-ignore Type does exist
    const response = await invoke(invokeArgs);
    return scValToJs(SorobanClient.xdr.ScVal.fromXDR(Buffer.from(response.xdr, 'base64'))) as u32

    
}

export async function name(): Promise<Buffer> {
    let invokeArgs: InvokeArgs = {method: 'name', args: []};
    // @ts-ignore Type does exist
    const response = await invoke(invokeArgs);
    return scValToJs(SorobanClient.xdr.ScVal.fromXDR(Buffer.from(response.xdr, 'base64'))) as Buffer

    
}

export async function symbol(): Promise<Buffer> {
    let invokeArgs: InvokeArgs = {method: 'symbol', args: []};
    // @ts-ignore Type does exist
    const response = await invoke(invokeArgs);
    return scValToJs(SorobanClient.xdr.ScVal.fromXDR(Buffer.from(response.xdr, 'base64'))) as Buffer

    
}

export interface AllowanceDataKey {
  from: Address;
  spender: Address;
}

