casper-client put-deploy     --node-address http://3.208.91.63:7777   \
--chain-name casper-test   \
  --secret-key ed25519-keys/secret_key.pem  \
 --payment-amount 100000000000   \
 --session-path CasperWork/samplework/newtest/target/wasm32-unknown-unknown/release/contract.wasm

casper-client put-deploy     --node-address http://3.208.91.63:7777   \
   --chain-name casper-test   \
   --secret-key ed25519-keys/secret_key.pem  \
   --payment-amount 60000000000   \
   --session-hash hash-d8757c139cf8a321d79dc36fe17bea029288d9faf8fd33b593b9f1c402481e22 \
   --session-entry-point "init" \
   --session-arg "scontract-hash:string='contract-package-wasm285448f572fa7f3801a649ef177939895eb4da303216ab0a5ee64a9fbe692b5c'" \
   --session-arg "token-hash:string='contract-b02ec9fe439a945bcc0cc4a786f22fab7ae41829e10ea029e6f82af1b3833b60'" \
    

   --session-arg "token-hash:account_hash='account-hash-4b05191dadb56bf3b7988167e771023298f609b6596a65fe784cfecd1f262000'" 
  --session-arg "scontract-hash:contract_hash='contract-3b05191dadb56bf3b7988167e771023298f609b6596a65fe784cfecd1f262000'" 


   casper-client put-deploy     --node-address http://3.208.91.63:7777   \
   --chain-name casper-test   \
   --secret-key ed25519-keys/secret_key.pem  \
   --payment-amount 60000000000   \
   --session-hash hash-d8757c139cf8a321d79dc36fe17bea029288d9faf8fd33b593b9f1c402481e22 \
   --session-entry-point "lock" \
   --session-arg "cliff_durtime:u64='2000000'"\
   --session-arg "cliff_amount:U256='200000000'"\
   --session-arg "recipient:string='account-hash-0256f840a7b330d8164779c51e1af3959d94d1dd4b9d0bb2e4acf85f094a4bf4'" 

   casper-client put-deploy     --node-address http://3.208.91.63:7777   \
   --chain-name casper-test   \
   --secret-key ed25519-keys/secret_key.pem  \
   --payment-amount 60000000000   \
   --session-hash hash-d8757c139cf8a321d79dc36fe17bea029288d9faf8fd33b593b9f1c402481e22 \
   --session-entry-point "claimable_amount"\
   --session-arg "recipient:string='account-hash-0256f840a7b330d8164779c51e1af3959d94d1dd4b9d0bb2e4acf85f094a4bf4'" \
   --session-arg "cliff_amount:u64='1'" 

   casper-client put-deploy     --node-address http://3.208.91.63:7777   \
   --chain-name casper-test   \
   --secret-key ed25519-keys/secret_key.pem  \
   --payment-amount 60000000000   \
   --session-hash hash-d8757c139cf8a321d79dc36fe17bea029288d9faf8fd33b593b9f1c402481e22 \
   --session-entry-point "claim"\
   --session-arg "recipient:string='account-hash-0256f840a7b330d8164779c51e1af3959d94d1dd4b9d0bb2e4acf85f094a4bf4'" 



   casper-client put-deploy     --node-address http://3.208.91.63:7777   \
   --chain-name casper-test   \
   --secret-key ed25519-keys/secret_key.pem  \
   --payment-amount 60000000000   \
   --session-hash hash-d8757c139cf8a321d79dc36fe17bea029288d9faf8fd33b593b9f1c402481e22 \
   --session-entry-point "total_lock_amount"

   casper-client put-deploy     --node-address http://3.208.91.63:7777   \
   --chain-name casper-test   \
   --secret-key ed25519-keys/secret_key.pem  \
   --payment-amount 60000000000   \
   --session-hash hash-d8757c139cf8a321d79dc36fe17bea029288d9faf8fd33b593b9f1c402481e22 \
   --session-entry-point "vested_amount"\
   --session-arg "recipient:string='account-hash-4b05191dadb56bf3b7988167e771023298f609b6596a65fe784cfecd1f262000'" 

  casper-client put-deploy     --node-address http://3.208.91.63:7777   \
   --chain-name casper-test   \
   --secret-key ed25519-keys/secret_key.pem  \
   --payment-amount 60000000000   \
   --session-hash hash-d8757c139cf8a321d79dc36fe17bea029288d9faf8fd33b593b9f1c402481e22 \
   --session-entry-point "get_amount_per_hourly" \
   --session-arg "recipient:string='account-hash-4b05191dadb56bf3b7988167e771023298f609b6596a65fe784cfecd1f262000'" 
