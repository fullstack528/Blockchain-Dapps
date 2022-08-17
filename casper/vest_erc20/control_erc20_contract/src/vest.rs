extern crate alloc;

use alloc::{
    string::{String, ToString},
};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

use casper_types::{
    account::AccountHash, U256, ApiError, ContractPackageHash
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
    lock_timestamp : u64,
    lock_amount : U256,    
    vested_amount : U256,
    hourly_vest_amount : U256,
}

fn make_dictionary_item_key(owner: AccountHash, ich: char) -> String {
    let newstr_ = owner.to_string();

    let (strhash, _) = newstr_.as_str().split_at(30);    
    let mut newstr = String::from(strhash);

    newstr.push(ich);
    
    newstr
}

#[derive(Default)]
pub struct VestContract;

impl VestContract
{
    pub fn init(&mut self, hash_token: String, hash_self_contract: String)
    {
        set_admin_account(runtime::get_caller()); 
        set_self_contract_hash(hash_self_contract); 
        interact_erc20::default().init(hash_token.clone());

        storage::new_dictionary(KEY_NAME_DIC_LOCK_INFOS).unwrap_or_revert();
    }

    pub fn set_recipient_info(&self, recac: AccountHash, rec_info: RecipientInfo)
    {    
        let key = runtime::get_key(KEY_NAME_DIC_LOCK_INFOS).unwrap_or_revert();
        let uref_dic = *key.as_uref().unwrap_or_revert();

        let dictionary_item_key = make_dictionary_item_key(recac, '1');  
        storage::dictionary_put(uref_dic, dictionary_item_key.as_str(), rec_info.lock_timestamp);

        let dictionary_item_key = make_dictionary_item_key(recac, '2');    
        storage::dictionary_put(uref_dic, dictionary_item_key.as_str(), rec_info.lock_amount);

        let dictionary_item_key = make_dictionary_item_key(recac, '3');    
        storage::dictionary_put(uref_dic, dictionary_item_key.as_str(), rec_info.vested_amount);

        let dictionary_item_key = make_dictionary_item_key(recac, '4');    
        storage::dictionary_put(uref_dic, dictionary_item_key.as_str(), rec_info.hourly_vest_amount);
    }
    
    pub fn set_recipient_claimable(&self, recac: AccountHash, claimable: U256)
    {    
        let key = runtime::get_key(KEY_NAME_DIC_LOCK_INFOS).unwrap_or_revert();
        let uref_dic = *key.as_uref().unwrap_or_revert();

        let dictionary_item_key = make_dictionary_item_key(recac, '5');  
        storage::dictionary_put(uref_dic, dictionary_item_key.as_str(), claimable);
    }

    pub fn get_recipient_infos(&mut self, reca: AccountHash) -> RecipientInfo
    {
        let key = runtime::get_key(KEY_NAME_DIC_LOCK_INFOS).unwrap_or_revert();
        let uref_dic = *key.as_uref().unwrap_or_revert();

        let dictionary_item_key = make_dictionary_item_key(reca, '1');

        let lock_timestamp = storage::dictionary_get(uref_dic, dictionary_item_key.as_str())
            .unwrap_or_revert()
            .unwrap_or_default();

        let dictionary_item_key = make_dictionary_item_key(reca, '2');
        let lock_amount = storage::dictionary_get(uref_dic, dictionary_item_key.as_str())
            .unwrap_or_revert()
            .unwrap_or_default();

        let dictionary_item_key = make_dictionary_item_key(reca, '3');
        let vested_amount: U256 = storage::dictionary_get(uref_dic, dictionary_item_key.as_str())
            .unwrap_or_revert()
            .unwrap_or_default();

        let dictionary_item_key = make_dictionary_item_key(reca, '4');
        let hourly_vest_amount: U256 = storage::dictionary_get(uref_dic, dictionary_item_key.as_str())
            .unwrap_or_revert()
            .unwrap_or_default();

            RecipientInfo{
            lock_timestamp:lock_timestamp,
            lock_amount:lock_amount,
            vested_amount:vested_amount,
            hourly_vest_amount:hourly_vest_amount,
        }
    }

    pub fn lock(&mut self, reciep: AccountHash, cliff_durtime: u64, cliff_amount: U256)
    {
        // self.verify_admin_account();
        {           
            interact_erc20::default().transfer_from(reciep, self_contract_hash(), cliff_amount);
            
            self.set_recipient_info(reciep, 
                RecipientInfo{
                    lock_timestamp: runtime::get_blocktime().into(),
                    lock_amount: cliff_amount,
                    vested_amount: U256::zero(),
                    hourly_vest_amount: cliff_amount.checked_div(U256::from(cliff_durtime)).unwrap()}
                );
            
            set_total_lock_amount(true, cliff_amount);            
        }
    }

    pub fn claim(&mut self, acc_recip: AccountHash)
    {
        let mut reci = self.get_recipient_infos(acc_recip);

        let stamp_now : u64 = runtime::get_blocktime().into();

        let past_hours : u64 = (stamp_now  - reci.lock_timestamp) / 3600000;

        let mut vestable_until_now: U256 = reci.hourly_vest_amount.checked_mul(U256::from(past_hours)).unwrap();

        if vestable_until_now > reci.lock_amount {
            vestable_until_now = reci.lock_amount;
        }

        let claimamount = vestable_until_now - reci.vested_amount;
        utils::set_key(RET_VAL_U256, claimamount);

        if !claimamount.is_zero() {
            interact_erc20::default().transfer(acc_recip, claimamount);

            reci.vested_amount += claimamount;    
            
            self.set_recipient_info(acc_recip, reci);
            

            set_total_lock_amount(false, claimamount);        
        }
        
    }
    
    pub fn claimable_amount(&mut self, acc_recip: AccountHash) -> U256
    {
        let reci = self.get_recipient_infos(acc_recip);

        let stamp_now : u64 = runtime::get_blocktime().into();

        let past_hours : u64 = (stamp_now  - reci.lock_timestamp) / 3600000;

        let mut vestable_until_now: U256 = reci.hourly_vest_amount.checked_mul(U256::from(past_hours)).unwrap();

        if vestable_until_now > reci.lock_amount {
            vestable_until_now = reci.lock_amount;
        }

        let claimamount = vestable_until_now - reci.vested_amount;
        utils::set_key(RET_VAL_U256, claimamount);
        self.set_recipient_claimable(acc_recip, claimamount);

        claimamount        
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