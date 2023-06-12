import * as SorobanClient from 'soroban-client';
import { xdr } from 'soroban-client';
import { Buffer } from "buffer";
import { scValStrToJs, scValToJs, addressToScVal } from './convert.js';
import { invoke, InvokeArgs } from './invoke.js';

declare const Errors: { message: string }[]

export * from './constants.js'
export * from './server.js'
export * from './invoke.js'

export type u32 = number;
export type i32 = number;
export type u64 = bigint;
export type i64 = bigint;
export type u128 = bigint;
export type i128 = bigint;
export type u256 = bigint;
export type i256 = bigint;
export type Address = string;
export type Option<T> = T | undefined;

/// Error interface containing the error message
export interface Error_ { message: string };

export interface Result<T, E = Error_> {
    unwrap(): T,
    unwrapErr(): E,
    isOk(): boolean,
    isErr(): boolean,
};

export class Ok<T> implements Result<T> {
    constructor(readonly value: T) { }
    unwrapErr(): Error_ {
        throw new Error('No error');
    }
    unwrap(): T {
        return this.value;
    }

    isOk(): boolean {
        return true;
    }

    isErr(): boolean {
        return !this.isOk()
    }
}

export class Err<T> implements Result<T> {
    constructor(readonly error: Error_) { }
    unwrapErr(): Error_ {
        return this.error;
    }
    unwrap(): never {
        throw new Error(this.error.message);
    }

    isOk(): boolean {
        return false;
    }

    isErr(): boolean {
        return !this.isOk()
    }
}

if (typeof window !== 'undefined') {
    //@ts-ignore Buffer exists
    window.Buffer = window.Buffer || Buffer;
}

const regex = /ContractError\((\d+)\)/;

function getError(err: string): Err<Error_> | undefined {
    const match = err.match(regex);
    if (!match) {
        return undefined;
    }
    if (Errors == undefined) {
        return undefined;
    }
    // @ts-ignore
    let i = parseInt(match[1], 10);
    if (i < Errors.length) {
        return new Err(Errors[i]!);
    }
    return undefined;
}

export async function put({key, val}: {key: string, val: string}, {signAndSend, fee}?: {signAndSend: boolean, fee: number} = {signAndSend: false, fee: 100}): Promise<void> {
    let invokeArgs: InvokeArgs = {
        signAndSend,
        fee,
        method: 'put', 
        args: [((i) => xdr.ScVal.scvSymbol(i))(key),
        ((i) => xdr.ScVal.scvSymbol(i))(val)], 
    };
    
    // @ts-ignore Type does exist
    const response = await invoke(invokeArgs);
    return ;
}

export async function get({key}: {key: string}, {signAndSend, fee}?: {signAndSend: boolean, fee: number} = {signAndSend: false, fee: 100}): Promise<Option<string>> {
    let invokeArgs: InvokeArgs = {
        signAndSend,
        fee,
        method: 'get', 
        args: [((i) => xdr.ScVal.scvSymbol(i))(key)], 
    };
    
    // @ts-ignore Type does exist
    const response = await invoke(invokeArgs);
    return scValStrToJs(response.xdr) as Option<string>;
}

export async function del({key}: {key: string}, {signAndSend, fee}?: {signAndSend: boolean, fee: number} = {signAndSend: false, fee: 100}): Promise<void> {
    let invokeArgs: InvokeArgs = {
        signAndSend,
        fee,
        method: 'del', 
        args: [((i) => xdr.ScVal.scvSymbol(i))(key)], 
    };
    
    // @ts-ignore Type does exist
    const response = await invoke(invokeArgs);
    return ;
}
