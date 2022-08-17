import * as utils from './utils.js';
import sdk  from 'casper-js-sdk';

const CasperClient = sdk.CasperClient;
const CLValueBuilder = sdk.CLValueBuilder;
const DeployUtil = sdk.DeployUtil;
const RuntimeArgs = sdk.RuntimeArgs;


// Path to contract to be installed.
const PATH_TO_CONTRACT = "../CasperWork/vestgit/erc20/target/wasm32-unknown-unknown/release/erc20_token.wasm";
const DEPLOY_NODE_ADDRESS = "http://3.208.91.63:7777/rpc";
const PATH_TO_CONTRACT_KEYS = "../ed25519-keys";
const DEPLOY_GAS_PAYMENT = 100000000000;
const DEPLOY_GAS_PRICE = 1;
const DEPLOY_CHAIN_NAME = "casper-test";

// Token parameters.
const TOKEN_NAME = "Acme Token";
const TOKEN_SYMBOL = "ACME";
const TOKEN_DECIMALS = 6;
const TOKEN_SUPPLY = 3470000000000;

/**
 * Demonstration entry point.
 */
const main = async () => {
    console.log("1")
    // Step 1: Set casper node client.
    const client = new CasperClient(DEPLOY_NODE_ADDRESS);
    console.log("2")

    // Step 2: Set contract operator key pair.
    const keyPairOfContract = utils.getKeyPairOfContract(PATH_TO_CONTRACT_KEYS);
    console.log("3")

    // Step 3: Set contract installation deploy (unsigned).
    let deploy = DeployUtil.makeDeploy(
        new DeployUtil.DeployParams(
            keyPairOfContract.publicKey,
            DEPLOY_CHAIN_NAME,
            DEPLOY_GAS_PRICE,
        ),
        DeployUtil.ExecutableDeployItem.newModuleBytes(
            utils.getBinary(PATH_TO_CONTRACT),
            RuntimeArgs.fromMap({
                decimals: CLValueBuilder.u8(TOKEN_DECIMALS),
                name: CLValueBuilder.string(TOKEN_NAME),
                symbol: CLValueBuilder.string(TOKEN_SYMBOL),
                total_supply: CLValueBuilder.u256(TOKEN_SUPPLY),
            })
        ),
        DeployUtil.standardPayment(DEPLOY_GAS_PAYMENT)
    );

    console.log("4")
    // Step 4: Sign deploy.
    deploy = client.signDeploy(deploy, keyPairOfContract); 
    console.log("5 deploy = ", deploy)

    // Step 5: Dispatch deploy to node.
    const deployHash = await client.putDeploy(deploy);
    console.log("6")

    // Step 6: Render deploy details.
    logDetails(deployHash)
    console.log("7")
};

/**
 * Emits to stdout deploy details.
 * @param {String} deployHash - Identifer of dispatched deploy.
 */
const logDetails = (deployHash) => {
    console.log(`
---------------------------------------------------------------------
installed contract -> ERC20
... account = ${PATH_TO_CONTRACT_KEYS}
... deploy chain = ${DEPLOY_CHAIN_NAME}
... deploy dispatch node = ${DEPLOY_NODE_ADDRESS}
... deploy gas payment = ${DEPLOY_GAS_PAYMENT}
... deploy gas price = ${DEPLOY_GAS_PRICE}
contract constructor args:
... token symbol = ${TOKEN_SYMBOL}
... token name = ${TOKEN_NAME}
... token supply = ${TOKEN_SUPPLY}
... token decimals = ${TOKEN_DECIMALS}
contract installation details:
... path = ${PATH_TO_CONTRACT}
... deploy hash = ${deployHash}
---------------------------------------------------------------------
    `);    
};
  
main();