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
export type Option<T> = T | undefined;

// export type Error_ = { error: string };
export type Result<T, E> = Ok<T, E> | Err<T, E>;

export class Ok<T, E> {
    readonly kind: 'ok' = 'ok';
    constructor(readonly value: T) { }

    unwrap(): T {
        return this.value;
    }

    map<U>(f: (value: T) => U): Result<U, E> {
        return new Ok(f(this.value));
    }

    mapErr<U>(_: (error: E) => U): Result<T, U> {
        return this as unknown as Result<T, U>;
    }
}

export class Err<T, E> {
    readonly kind: 'err' = 'err';
    constructor(readonly error: E) { }

    unwrap(): never {
        throw new Error(this.error as unknown as string);
    }

    map<U>(_: (value: T) => U): Result<U, E> {
        return this as unknown as Result<U, E>;
    }

    mapErr<U>(f: (error: E) => U): Result<T, U> {
        return new Err(f(this.error));
    }
}

window.Buffer = window.Buffer || Buffer;

