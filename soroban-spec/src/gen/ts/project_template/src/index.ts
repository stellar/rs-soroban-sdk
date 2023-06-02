import * as SorobanClient from 'soroban-client';
import { xdr } from 'soroban-client';
import { Buffer } from "buffer";
import { scValStrToJs } from './convert';
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
export type Option<T> = T | undefined;

/// Error interface containing the error message
export interface Error_ { message: string };

export interface Result<T, E = Error_> {
    unwrap(): T,
    map<U>(f: (value: T) => U): Result<U, E>,
};

export class Ok<T> {
    constructor(readonly value: T) { }
    unwrap(): T {
        return this.value;
    }

    isOk(): boolean {
        return true;
    }

    isErr(): boolean {
        return !this.isOk
    }

    map<U>(f: (value: T) => U): Result<U> {
        return new Ok(f(this.value));
    }
}

export class Err<T> {
    constructor(readonly message: Error_) { }
    unwrap(): never {
        throw new Error(this.message as unknown as string);
    }

    isOk(): boolean {
        return false;
    }

    isErr(): boolean {
        return !this.isOk
    }

    map<U>(_: (value: T) => U): Result<U> {
        return this as unknown as Result<U>;
    }
}

if (typeof window !== 'undefined') {
    //@ts-ignore Buffer exists
    window.Buffer = window.Buffer || Buffer;
}

