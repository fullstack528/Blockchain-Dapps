casper-client put-deploy     --node-address http://3.208.91.63:7777   \
--chain-name casper-test   \
  --secret-key ed25519-keys/secret_key.pem  \
 --payment-amount 100000000000   \
 --session-path CasperWork/samplework/newtest/target/wasm32-unknown-unknown/release/contract.wasm

casper-client put-deploy     --node-address http://3.208.91.63:7777   \
   --chain-name casper-test   \
   --secret-key ed25519-keys/secret_key.pem  \
   --payment-amount 60000000000   \
   --session-hash hash-f48937440423aaba4a75ffc69d5e1742fe3ce3cb4fbd5580c40b610ed9e7a3b2 \
   --session-entry-point "init" \
   --session-arg "scontract-hash:string='contract-package-wasm7f731f44f8429dacb8d81c52c72127a894347d85d94d79a9cfa51864da8d2b7f'" 
   
   --session-arg "token-hash:string='contract-b02ec9fe439a945bcc0cc4a786f22fab7ae41829e10ea029e6f82af1b3833b60'" \
    

   --session-arg "token-hash:account_hash='account-hash-4b05191dadb56bf3b7988167e771023298f609b6596a65fe784cfecd1f262000'" 
  --session-arg "scontract-hash:contract_hash='contract-3b05191dadb56bf3b7988167e771023298f609b6596a65fe784cfecd1f262000'" 


   casper-client put-deploy     --node-address http://3.208.91.63:7777   \
   --chain-name casper-test   \
   --secret-key ed25519-keys/secret_key.pem  \
   --payment-amount 60000000000   \
   --session-hash hash-f48937440423aaba4a75ffc69d5e1742fe3ce3cb4fbd5580c40b610ed9e7a3b2 \
   --session-entry-point "lock" \
   --session-arg "cliff_durtime:u64='2000000000'"\
   --session-arg "cliff_amount:U256='2000000000'"\
   --session-arg "unit_time:u64='10000000'"\
   --session-arg "recipient:string='account-hash-0256f840a7b330d8164779c51e1af3959d94d1dd4b9d0bb2e4acf85f094a4bf4'" \
   --session-arg "token-hash:string='contract-de93171e1867e787f771a8ad04dd33b1167fbf9cdde1468443dcb640fccca1a0'" 

   casper-client put-deploy     --node-address http://3.208.91.63:7777   \
   --chain-name casper-test   \
   --secret-key ed25519-keys/secret_key.pem  \
   --payment-amount 60000000000   \
   --session-hash hash-f48937440423aaba4a75ffc69d5e1742fe3ce3cb4fbd5580c40b610ed9e7a3b2 \
   --session-entry-point "claimable_amount"\
   --session-arg "recipient:string='account-hash-0256f840a7b330d8164779c51e1af3959d94d1dd4b9d0bb2e4acf85f094a4bf4'" \
   --session-arg "token-hash:string='contract-de93171e1867e787f771a8ad04dd33b1167fbf9cdde1468443dcb640fccca1a0'" 
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
