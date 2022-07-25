#![cfg(all(feature = "testutils", not(target_family = "wasm")))]

use std::panic::panic_any;
use stellar_contract_sdk::{Env, Status};
use stellar_xdr::ScHostStorageErrorCode;

#[test]
fn test_assert_panic_with_status() {
    let e = Env::with_empty_recording_storage();
    let status: Status = ScHostStorageErrorCode::AccessToUnknownEntry.into();
    e.assert_panic_with_status(status, |_env| panic_any(status));
}

#[test]
fn test_assert_panic_with_string() {
    let e = Env::with_empty_recording_storage();
    e.assert_panic_with_string("oh no", |_env| panic!("oh no"));
}

#[test]
fn test_assert_panic_with_fmt_string() {
    let e = Env::with_empty_recording_storage();
    e.assert_panic_with_string("oh no 123", |_env| panic!("oh no {}", 123));
}

#[test]
#[should_panic]
fn test_assert_panic_wrong_string() {
    let e = Env::with_empty_recording_storage();
    e.assert_panic_with_string("oh no", |_env| panic!("and yet"));
}

#[test]
#[should_panic]
fn test_assert_panic_with_wrong_status() {
    let e = Env::with_empty_recording_storage();
    let expected: Status = ScHostStorageErrorCode::AccessToUnknownEntry.into();
    let got: Status = ScHostStorageErrorCode::GetOnDeletedKey.into();
    e.assert_panic_with_status(expected, |_env| panic_any(got));
}

#[test]
#[should_panic]
fn test_assert_panic_with_wrong_type() {
    let e = Env::with_empty_recording_storage();
    let expected: Status = ScHostStorageErrorCode::AccessToUnknownEntry.into();
    e.assert_panic_with_status(expected, |_env| panic_any(10));
}
