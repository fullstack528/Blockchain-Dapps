extern crate alloc;

 use core::convert::TryInto;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

use casper_types::{
    api_error::ApiError,
    bytesrepr::{FromBytes, ToBytes},
    CLTyped, URef, account::AccountHash
};
use alloc::{string::{String,}, };

pub fn get_key<T: FromBytes + CLTyped>(name: &str) -> T {
    let key: URef = runtime::get_key(name)
        .unwrap_or_revert_with(ApiError::MissingKey)
        .into_uref()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);
    storage::read(key)
        .unwrap_or_revert_with(ApiError::Read)
        .unwrap_or_revert_with(ApiError::ValueNotFound)  
}

pub fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}

pub fn get_account_hash_from_string(str_hash: String)->AccountHash
{
    let ac_hashex = AccountHash::from_bytes(str_hash.as_bytes());
        
    let (ac_hash, _) = match ac_hashex{
        Ok(ac_hash) => ac_hash,
        Err(erro) => panic!("Problem opening {:?}", erro),
    }; 

    ac_hash
}

