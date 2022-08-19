extern crate alloc;

use alloc::{
    string::{String, ToString}, vec::Vec
};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

use casper_types::{
    account::AccountHash, U256, ApiError, ContractPackageHash, ContractHash, bytesrepr::ToBytes
};

use crate::interact_token::interact_erc20;
use crate::utils;

use crate::{
    constants::{
        RET_VAL_U256,
        KEY_NAME_ADMIN, KEY_NAME_TOTAL_LOCK_AMOUNT, KEY_NAME_SELF_CONTRACT_HASH, KEY_NAME_DIC_LOCK_INFOS
    },
};

//#[derive(Serialize, Debug)]
pub struct RecipientInfo
{
    lock_timestamp: u64,
    lock_amount: U256,    
    release_time_unit: u64,
    release_total_time: u64,
    vested_amount: U256,
    release_amount_per_unitime : U256,
}

// fn make_dictionary_item_key(owner: AccountHash, ich: char) -> String {
//     let newstr_ = owner.to_string();

//     let (strhash, _) = newstr_.as_str().split_at(30);    
//     let mut newstr = String::from(strhash);

//     newstr.push(ich);
    
//     newstr
// }

fn make_dictionary_item_key(owner: AccountHash, spender: ContractHash) -> String {
    let mut preimage = Vec::new();
    preimage.append(&mut owner.to_bytes().unwrap_or_revert());
    preimage.append(&mut spender.to_bytes().unwrap_or_revert());

    let key_bytes = runtime::blake2b(&preimage);
    let mut prehex: String = hex::encode(&key_bytes);
    prehex.pop(); //slice last letter to add index
    prehex
}

#[derive(Default)]
pub struct VestContract;

impl VestContract
{
    pub fn init(&mut self, hash_self_contract: String)
    {
        set_admin_account(runtime::get_caller()); 
        set_self_contract_hash(hash_self_contract); 
        //interact_erc20::default().init(hash_token.clone());

        storage::new_dictionary(KEY_NAME_DIC_LOCK_INFOS).unwrap_or_revert();
    }

    pub fn set_recipient_info(&self, recac: AccountHash, tokh: ContractHash, rec_info: RecipientInfo)
    {    
        let key = runtime::get_key(KEY_NAME_DIC_LOCK_INFOS).unwrap_or_revert();
        let uref_dic = *key.as_uref().unwrap_or_revert();

        let pre_dickey = make_dictionary_item_key(recac, tokh);        

        let mut dictionary_item_key = pre_dickey.clone();  dictionary_item_key.push('1');

        utils::set_key("getrecipeinfo", "getrecipeinfo");
        utils::set_key("getrecipeinfo1", dictionary_item_key.clone());

        storage::dictionary_put(uref_dic, dictionary_item_key.as_str(), rec_info.lock_timestamp);

        let mut dictionary_item_key = pre_dickey.clone();  dictionary_item_key.push('2');
        storage::dictionary_put(uref_dic, dictionary_item_key.as_str(), rec_info.lock_amount);

        let mut dictionary_item_key = pre_dickey.clone();  dictionary_item_key.push('3');
        storage::dictionary_put(uref_dic, dictionary_item_key.as_str(), rec_info.vested_amount);

        let mut dictionary_item_key = pre_dickey.clone();  dictionary_item_key.push('4');
        storage::dictionary_put(uref_dic, dictionary_item_key.as_str(), rec_info.release_time_unit);

        let mut dictionary_item_key = pre_dickey.clone();  dictionary_item_key.push('5');
        storage::dictionary_put(uref_dic, dictionary_item_key.as_str(), rec_info.release_amount_per_unitime);

        let mut dictionary_item_key = pre_dickey.clone();  dictionary_item_key.push('6');
        storage::dictionary_put(uref_dic, dictionary_item_key.as_str(), rec_info.release_total_time);
    }
    
    pub fn set_recipient_claimable(&self, reca: AccountHash, tokh: ContractHash, claimable: U256)
    {    
        let key = runtime::get_key(KEY_NAME_DIC_LOCK_INFOS).unwrap_or_revert();
        let uref_dic = *key.as_uref().unwrap_or_revert();

        let pre_dickey = make_dictionary_item_key(reca, tokh);        

        let mut dictionary_item_key = pre_dickey.clone();  dictionary_item_key.push('7');
        storage::dictionary_put(uref_dic, dictionary_item_key.as_str(), claimable);
        //utils::set_key(dictionary_item_key, claimable)
    }

    pub fn set_recipient_vested_amount(&self, reca: AccountHash, tokh: ContractHash, vested_amount: U256)
    {    
        let key = runtime::get_key(KEY_NAME_DIC_LOCK_INFOS).unwrap_or_revert();
        let uref_dic = *key.as_uref().unwrap_or_revert();

        let pre_dickey = make_dictionary_item_key(reca, tokh);        

        let mut dictionary_item_key = pre_dickey.clone();  dictionary_item_key.push('3');
        storage::dictionary_put(uref_dic, dictionary_item_key.as_str(), vested_amount);
        //utils::set_key(dictionary_item_key, claimable)
    }

    pub fn get_recipient_infos(&mut self, reca: AccountHash, tokh: ContractHash, uparse: u64) -> RecipientInfo
    {
        utils::set_key("getrecipeinfo", "getrecipeinfo");
        
        let key = runtime::get_key(KEY_NAME_DIC_LOCK_INFOS).unwrap_or_revert();

        if uparse < 2 {
            return RecipientInfo{
                lock_timestamp:0,
                lock_amount:U256::from(0),
                vested_amount: U256::from(0),
                release_time_unit: 0,
                release_amount_per_unitime:U256::from(0),
                release_total_time: 0};
        }

        let uref_dic = *key.as_uref().unwrap_or_revert();

        if uparse < 3 {
            return RecipientInfo{
                lock_timestamp:0,
                lock_amount:U256::from(0),
                vested_amount: U256::from(0),
                release_time_unit: 0,
                release_amount_per_unitime:U256::from(0),
                release_total_time: 0};
        }

        let pre_dickey = make_dictionary_item_key(reca, tokh);  
        utils::set_key("getrecipeinfo3", pre_dickey.clone());

        if uparse < 4 {
            return RecipientInfo{
                lock_timestamp:0,
                lock_amount:U256::from(0),
                vested_amount: U256::from(0),
                release_time_unit: 0,
                release_amount_per_unitime:U256::from(0),
                release_total_time: 0};
        }

        let mut dictionary_item_key = pre_dickey.clone();  

        utils::set_key("getrecipeinfo4", pre_dickey.clone());
        
        if uparse < 5 {
            return RecipientInfo{
                lock_timestamp:0,
                lock_amount:U256::from(0),
                vested_amount: U256::from(0),
                release_time_unit: 0,
                release_amount_per_unitime:U256::from(0),
                release_total_time: 0};
        }

        dictionary_item_key.push('1');
        let lock_timestamp: u64 = storage::dictionary_get(uref_dic, dictionary_item_key.as_str())
            .unwrap_or_revert()
            .unwrap_or_default();

        let mut dictionary_item_key = pre_dickey.clone();  dictionary_item_key.push('2');
        let lock_amount: U256 = storage::dictionary_get(uref_dic, dictionary_item_key.as_str())
            .unwrap_or_revert()
            .unwrap_or_default();

        let mut dictionary_item_key = pre_dickey.clone();  dictionary_item_key.push('3');
        let vested_amount: U256 = storage::dictionary_get(uref_dic, dictionary_item_key.as_str())
            .unwrap_or_revert()
            .unwrap_or_default();

        let mut dictionary_item_key = pre_dickey.clone();  dictionary_item_key.push('4');
        let release_time_unit: u64 = storage::dictionary_get(uref_dic, dictionary_item_key.as_str())
            .unwrap_or_revert()
            .unwrap_or_default();

        let mut dictionary_item_key = pre_dickey.clone();  dictionary_item_key.push('5');
        let release_amount_per_unitime: U256 = storage::dictionary_get(uref_dic, dictionary_item_key.as_str())
            .unwrap_or_revert()
            .unwrap_or_default();

        let mut dictionary_item_key = pre_dickey.clone();  dictionary_item_key.push('6');
        let release_total_time: u64 = storage::dictionary_get(uref_dic, dictionary_item_key.as_str())
            .unwrap_or_revert()
            .unwrap_or_default();

        RecipientInfo{
            lock_timestamp:lock_timestamp,
            lock_amount:lock_amount,
            vested_amount: vested_amount,
            release_time_unit: release_time_unit,
            release_amount_per_unitime:release_amount_per_unitime,
            release_total_time: release_total_time
        }
    }
    
    pub fn lock_vest(&mut self, reciep: AccountHash, hash_token: ContractHash, cliff_durtime: u64, release_time_unit: u64, cliff_amount: U256)
    {
        // self.verify_admin_account();
        {           
            utils::set_key("lockvest1", "lockvest1");
            if cliff_durtime > 2000000000{
                interact_erc20::default().transfer_from(hash_token, reciep, self_contract_hash(), cliff_amount);
            utils::set_key("lockvest2", "lockvest2");
        }
            if cliff_durtime > 2000000001{
                utils::set_key("lockvest3", "lockvest3");

                self.set_recipient_info(reciep, hash_token, 
                    RecipientInfo{
                        lock_timestamp: runtime::get_blocktime().into(),
                        lock_amount: cliff_amount,
                        vested_amount: U256::zero(),
                        release_time_unit: release_time_unit,
                        release_total_time: cliff_durtime,
                        release_amount_per_unitime: cliff_amount.checked_div(U256::from(cliff_durtime / release_time_unit)).unwrap()
                    }
                );
            }
            set_total_lock_amount(true, cliff_amount);            
        }
    }

    pub fn claim(&mut self, acc_recip: AccountHash, hash_token: ContractHash, uparse: u64)
    {
        if uparse < 1   { return ; }
        let mut reci = self.get_recipient_infos(acc_recip, hash_token, uparse);
        
        if uparse < 6   { return ; }
        let stamp_now : u64 = runtime::get_blocktime().into();

        let past_hours : u64 = (stamp_now  - reci.lock_timestamp) / (reci.release_time_unit); // * 3600000

        if uparse < 7   { return ; }
        let mut vestable_until_now: U256 = reci.release_amount_per_unitime.checked_mul(U256::from(past_hours)).unwrap();

        if vestable_until_now > reci.lock_amount {
            vestable_until_now = reci.lock_amount;
        }
        if uparse < 8   { return ; }

        let claimamount = vestable_until_now - reci.vested_amount;
        utils::set_key(RET_VAL_U256, claimamount);

        if  !claimamount.is_zero() {
            if uparse > 7 {
             interact_erc20::default().transfer(hash_token, acc_recip, claimamount); }

            reci.vested_amount += claimamount;    
            
            if uparse > 8
            {self.set_recipient_vested_amount(acc_recip, hash_token, reci.vested_amount);}

            set_total_lock_amount(false, claimamount);        
        }
        
    }
    
    pub fn claimable_amount(&mut self, acc_recip: AccountHash, hash_token: ContractHash, uparse: u64) -> U256
    {
        if uparse < 4
        { return U256::from(0); }

        let reci = self.get_recipient_infos(acc_recip, hash_token, uparse);

        if uparse < 8 {
        return U256::from(0); }

        let stamp_now : u64 = runtime::get_blocktime().into();

        let past_units : u64 = (stamp_now  - reci.lock_timestamp) / (reci.release_time_unit); // * 3600000

        utils::set_key("claimableamount", "claimableamount");
        utils::set_key("claimableamount1", stamp_now);
        utils::set_key("claimableamount2", reci.lock_timestamp);
        utils::set_key("claimableamount3", reci.release_time_unit);
        utils::set_key("claimableamount4", past_units);

        if past_units > 0
        {
            let mut vestable_until_now: U256 = reci.release_amount_per_unitime.checked_mul(U256::from(past_units)).unwrap();

            if vestable_until_now > reci.lock_amount {
                vestable_until_now = reci.lock_amount;
            }

            let claimamount = vestable_until_now - reci.vested_amount;
            utils::set_key(RET_VAL_U256, claimamount);
            self.set_recipient_claimable(acc_recip, hash_token, claimamount);
            claimamount
        }
        else
        {
            U256::from(0) 
        }
    }

    pub fn verify_admin_account(&mut self) 
    {
        if admin_account() != runtime::get_caller() 
        {
            runtime::revert(ApiError::InvalidPurse);
        }
    }
} 

fn admin_account() -> AccountHash {
    utils::get_key(KEY_NAME_ADMIN)
}

fn set_admin_account(admin: AccountHash)
{
    utils::set_key(KEY_NAME_ADMIN, admin);
}

fn set_total_lock_amount( b_increase: bool, tamount: U256)
{
    let tvm: U256 = utils::get_key(KEY_NAME_TOTAL_LOCK_AMOUNT);

    if b_increase{
        utils::set_key(KEY_NAME_TOTAL_LOCK_AMOUNT, tvm + tamount);
    }
    else{
        utils::set_key(KEY_NAME_TOTAL_LOCK_AMOUNT, tvm - tamount);
    }
}

fn self_contract_hash() -> ContractPackageHash {
    utils::get_key(KEY_NAME_SELF_CONTRACT_HASH)
}

fn set_self_contract_hash(conthash: String)
{
    let self_acc_hash = ContractPackageHash::from_formatted_str(conthash.as_str()).expect("self contract string format is error");
    utils::set_key(KEY_NAME_SELF_CONTRACT_HASH, self_acc_hash);
}

