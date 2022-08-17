//#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(unused_attributes)]

// #[cfg(not(target_arch = "wasm32"))]
// compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;
use core::str::FromStr;

use casper_contract::{contract_api::{runtime, storage}, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{runtime_args, ContractHash, Key, RuntimeArgs, account::AccountHash, U256, ContractPackageHash};

use alloc::{
    collections::{BTreeMap}, //BTreeSet},
    string::String, borrow::ToOwned
};

use crate::utils;
use crate::{Error, Address};

use crate::{
    constants::{
        ENTRY_POINT_NAME_BALANCE_OF,
        ENTRY_POINT_NAME_TRANSFER_FROM,
        ENTRY_POINT_NAME_TRANSFER,
        ARG_NAME_OWNER,
        ARG_NAME_AMOUNT,
        KEY_NAME_TOKEN_HASH,
        ARG_NAME_SPENDER,
        ARG_NAME_RECIPIENT,
    },
};

#[derive(Default)]
pub struct interact_erc20;

impl interact_erc20
{
    pub fn init(&mut self, tconthash : String)
    {   
        let contHash = ContractHash::from_formatted_str(tconthash.as_str()).unwrap();
        utils::set_key(KEY_NAME_TOKEN_HASH, contHash);
    }

    pub fn get_token_hash(&self) -> ContractHash
    {
        utils::get_key(KEY_NAME_TOKEN_HASH)
    }

    pub fn transfer_from(&mut self
        , owner: AccountHash
        , spender: ContractPackageHash
        , amount: U256
    ) 
    {
        let tokenhash = self.get_token_hash();

        runtime::call_contract(
            tokenhash,                            //contracthash
            ENTRY_POINT_NAME_TRANSFER_FROM,
            runtime_args! {
                ARG_NAME_OWNER => Address::from(owner),         //owner : AccountHash
                ARG_NAME_RECIPIENT => Address::from(spender),   //spender: AccountHash
                ARG_NAME_AMOUNT => amount
            },
        )
    }

    pub fn transfer(&mut self
        , recipient: AccountHash
        , amount: U256
    ) 
    {
        runtime::call_contract(
            self.get_token_hash(), 
            ENTRY_POINT_NAME_TRANSFER,
            runtime_args! {
                ARG_NAME_RECIPIENT => Address::from(recipient),
                ARG_NAME_AMOUNT => amount
            },
        )
    }
}
