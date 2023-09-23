#![cfg(test)]

mod address;
mod auth;
mod budget;
mod contract_add_i32;
mod contract_assert;
mod contract_docs;
mod contract_invoke;
mod contract_overlapping_type_fn_names;
mod contract_snapshot;
mod contract_store;
mod contract_udt_enum;
mod contract_udt_struct;
mod contract_udt_struct_tuple;
mod contractimport;
mod contractimport_with_error;
mod crypto_ed25519;
mod crypto_keccak256;
mod crypto_secp256k1;
mod crypto_sha256;
mod env;
mod prng;
mod proptest_scval_cmp;
mod proptest_val_cmp;
mod token_client;
mod token_spec;
