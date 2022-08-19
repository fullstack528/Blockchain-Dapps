#![no_std]
#![no_main]
#[macro_use]

// #[cfg(not(target_arch = "wasm32"))]
// compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

// use alloc::string::String;
use alloc::{collections::BTreeMap, string::{String, ToString}, };

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{ Key, account::AccountHash, ContractHash,
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints}, CLType, CLTyped, Parameter
    , U256, CLValue
};


use contract::{VestContract, constants::ENTRY_POINT_NAME_CLAIMABLE_AMOUNT};

//use VestContract;
use contract::{
    constants::{
        CONTRACT_NAME, CONTRACT_VERSION, CONTRACT_HASH, VESTOR_PACKAGE_NAME, VESTOR_UREF_NAME,
        ARG_NAME_ERC20_TOKEN_HASH, ARG_NAME_ERC20_SELFCONTRACT_HASH,
        ARG_NAME_CLIFF_DURTIME, ARG_NAME_CLIFF_AMOUNT, ARG_NAME_RECIPIENT, ARG_NAME_UNIT_TIME, 
        KEY_NAME_TOTAL_LOCK_AMOUNT,
        ENTRY_POINT_NAME_INIT,
        ENTRY_POINT_NAME_LOCK,
        ENTRY_POINT_NAME_CLAIM,
        RET_VAL_U256,
    },
};

#[no_mangle]
pub extern "C" fn call() {
    // The key shouldn't already exist in the named keys.
    let counter_local_key = storage::new_uref(0_i32);
    
    // Create initial named keys of the contract.
    let mut vestor_named_keys: BTreeMap<String, Key> = BTreeMap::new();
    let key_name = String::from(CONTRACT_NAME);
    vestor_named_keys.insert(key_name, counter_local_key.into());

    let total_vest_key = storage::new_uref(U256::zero());
    vestor_named_keys.insert(String::from(KEY_NAME_TOTAL_LOCK_AMOUNT), total_vest_key.into());

    let ret_val_u256_key = storage::new_uref(U256::zero());
    vestor_named_keys.insert(String::from(RET_VAL_U256), ret_val_u256_key.into());


    let (stored_contract_hash, contract_version) = storage::new_contract(
        get_entry_points(),
        Some(vestor_named_keys),
        Some(VESTOR_PACKAGE_NAME.to_string()),
        Some(VESTOR_UREF_NAME.to_string()),
    );

    // The current version of the contract will be reachable through named keys
    let version_uref = storage::new_uref(contract_version);
    runtime::put_key(CONTRACT_VERSION, version_uref.into());

    runtime::put_key(CONTRACT_HASH, stored_contract_hash.into());
}


#[no_mangle]
pub extern "C" fn init() {
    // let str_hash_token = runtime::get_named_arg::<String>(ARG_NAME_ERC20_TOKEN_HASH);
    let str_hash_selfcontract: String = runtime::get_named_arg(ARG_NAME_ERC20_SELFCONTRACT_HASH);

    VestContract::default().init(str_hash_selfcontract);        
}

#[no_mangle]
pub extern "C" fn lock(){
    let cliff_durtime: u64 = runtime::get_named_arg(ARG_NAME_CLIFF_DURTIME);
    let cliff_amount: U256 = runtime::get_named_arg(ARG_NAME_CLIFF_AMOUNT);
    let unit_time: u64 = runtime::get_named_arg(ARG_NAME_UNIT_TIME);
    let str_account_hash_reciepient: String = runtime::get_named_arg(ARG_NAME_RECIPIENT);
    let str_hash_token: String = runtime::get_named_arg(ARG_NAME_ERC20_TOKEN_HASH);
    
    let hash_reciep = AccountHash::from_formatted_str(str_account_hash_reciepient.as_str()).expect("lock hash string format is error");
    let hash_token = ContractHash::from_formatted_str(str_hash_token.as_str()).expect("lock token hash string format is error");
    VestContract::default().lock_vest(hash_reciep, hash_token, cliff_durtime, unit_time, cliff_amount); 
}

#[no_mangle]
pub extern "C" fn claim()
{
    let str_account_hash_reciepient: String = runtime::get_named_arg(ARG_NAME_RECIPIENT);
    let str_hash_token: String = runtime::get_named_arg(ARG_NAME_ERC20_TOKEN_HASH);
    let uparse: u64 = runtime::get_named_arg("uparse");

    let acc_reciep = AccountHash::from_formatted_str(str_account_hash_reciepient.as_str()).expect("claim hash string format is error");
    let hash_token = ContractHash::from_formatted_str(str_hash_token.as_str()).expect("claim token hash string format is error");
    VestContract::default().claim(acc_reciep, hash_token, uparse);    
}

#[no_mangle]
pub extern "C" fn claimable_amount() 
{
    let str_account_hash_reciepient: String = runtime::get_named_arg(ARG_NAME_RECIPIENT);
    let str_hash_token: String = runtime::get_named_arg(ARG_NAME_ERC20_TOKEN_HASH);
    let uparse: u64 = runtime::get_named_arg("uparse");

    let acc_reciep = AccountHash::from_formatted_str(str_account_hash_reciepient.as_str()).expect("claimable hash string format is error");
    let hash_token = ContractHash::from_formatted_str(str_hash_token.as_str()).expect("claim token hash string format is error");
    let retval = VestContract::default().claimable_amount(acc_reciep, hash_token, uparse);  

    runtime::ret(CLValue::from_t(retval).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn claimable() -> U256
{
    U256::from(0)
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_NAME_INIT,
        vec![
            // Parameter::new(ARG_NAME_ERC20_TOKEN_HASH, String::cl_type()),
            Parameter::new(ARG_NAME_ERC20_SELFCONTRACT_HASH, String::cl_type()),
        ],
        CLType::I32,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_NAME_LOCK,
        vec![
            Parameter::new(ARG_NAME_CLIFF_DURTIME, u64::cl_type()),
            Parameter::new(ARG_NAME_CLIFF_AMOUNT, U256::cl_type()),
            Parameter::new(ARG_NAME_UNIT_TIME, u64::cl_type()),
            Parameter::new(ARG_NAME_RECIPIENT, String::cl_type()),
            Parameter::new(ARG_NAME_ERC20_TOKEN_HASH, String::cl_type()),
            
        ],
        CLType::I32,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));


    entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_NAME_CLAIM,
        vec![
            Parameter::new(ARG_NAME_RECIPIENT, String::cl_type()),
            Parameter::new(ARG_NAME_ERC20_TOKEN_HASH, String::cl_type()),
        ],
        CLType::I32,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_NAME_CLAIMABLE_AMOUNT,
        vec![
            Parameter::new(ARG_NAME_RECIPIENT, String::cl_type()),
            Parameter::new(ARG_NAME_ERC20_TOKEN_HASH, String::cl_type()),
            Parameter::new("uparse", u64::cl_type()),
        ],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    
    entry_points.add_entry_point(EntryPoint::new(
        "claimable",
        vec![
        ],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points
}