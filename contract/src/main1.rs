#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::{collections::BTreeMap, string::String};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{runtime_args, ApiError, ContractPackageHash, Key, RuntimeArgs};

const KEY_NAME: &str = "my-key-name";
const RUNTIME_ARG_NAME: &str = "message";

/// An error enum which can be converted to a `u16` so it can be returned as an `ApiError::User`.
#[repr(u16)]
enum Error {
    KeyAlreadyExists = 0,
    KeyMismatch = 1,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        ApiError::User(error as u16)
    }
}

#[no_mangle]
pub extern "C" fn call() {
    let passing_map_contract_hash_key: Key =
        runtime::get_named_arg("passing_map_contract_hash_key");
    let passing_map_contract_hash =
        ContractPackageHash::new(passing_map_contract_hash_key.into_hash().unwrap());

    let mut meta: BTreeMap<String, String> = BTreeMap::new();
    meta.insert(String::from("key1"), String::from("value1"));
    runtime::call_versioned_contract(
        passing_map_contract_hash,
        None,
        "ep1",
        runtime_args! {"meta" => meta},
    )
}
