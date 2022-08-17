-	Erc20 token
In erc20-master folder, type command “make build-contracts”
In deployContractByJS folder, edit depolyBySigner.js according to requirements.
And then type node depolyBySigner.js to deploy erc20 token
-	Control contract
In contract folder, type command “cargo build --release --target wasm32-unknown-unknown”
And then type following command 
casper-client put-deploy     --node-address http://3.208.91.63:7777   \
--chain-name casper-test   \
  --secret-key ed25519-keys/secret_key.pem  \
 --payment-amount 100000000000   \
 --session-path CasperWork/samplework/newtest/target/wasm32-unknown-unknown/release/contract.wasm


After deploy, call init function by command line as following
casper-client put-deploy     --node-address http://3.208.91.63:7777   \
   --chain-name casper-test   \
   --secret-key ed25519-keys/secret_key.pem  \
   --payment-amount 60000000000   \
   --session-hash hash-837c78745a78246c9486147c1d22e887b012cdb6c46c1c15da8e86867bec525b \
   --session-entry-point "init" \
   --session-arg "scontract-hash:string='contract-package-wasm4f8050bd4df07a467bdacc006f1c3b0049aea042373d217957eb03e3c6a31612'" \
   --session-arg "token-hash:string='contract-b02ec9fe439a945bcc0cc4a786f22fab7ae41829e10ea029e6f82af1b3833b60'" \
   
Parameter “token-hash”: erc20 token contract hash
Parameter “scontract-hash”: control contract package hash


