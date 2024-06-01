#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::{boxed::Box, collections::BTreeMap, string::String, vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    ApiError, CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key, Parameter,
};

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
pub extern "C" fn ep1() {
    let meta: BTreeMap<String, String> = runtime::get_named_arg("meta");
    runtime::put_key("meta", storage::new_uref(meta).into());
}
#[no_mangle]
pub extern "C" fn call() {
    let mut entry_points = EntryPoints::new();
    let ep1 = EntryPoint::new(
        String::from("ep1"),
        vec![Parameter::new(
            "meta",
            CLType::Map {
                key: Box::new(CLType::String),
                value: Box::new(CLType::String),
            },
        )],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    entry_points.add_entry_point(ep1);

    let (contract_hash, contract_version) = storage::new_contract(
        entry_points,
        None,
        Some(String::from("passing_map").clone()),
        None,
    );
}
