use dotenv;
use hex_literal::hex;
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_primitives::transaction::{Action, FunctionCallAction, Transaction};
use near_primitives::types::{BlockReference, Finality, FunctionArgs};
use near_primitives::views::QueryRequest;
use serde_json::json;
use std::env;
use std::str::FromStr;
use web3::ethabi::ethereum_types::BloomInput;
use web3::signing;
use web3::types::{BlockId, BlockNumber, H160, U64};

mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let client = JsonRpcClient::connect(env::var("RPC_ENDPOINT").unwrap());

    let _ = insert_filter(client).await;

    Ok(())
}

async fn insert_filter(client: JsonRpcClient) -> Result<(), Box<dyn std::error::Error>> {
    let logs = get_logs().await;

    let signer_account_id = env::var("ACCOUNT_ID").unwrap().parse().unwrap();
    let signer_secret_key = env::var("SECRET_KEY").unwrap().parse().unwrap();

    let signer = near_crypto::InMemorySigner::from_secret_key(signer_account_id, signer_secret_key);
    let access_key_query_response = client
        .call(methods::query::RpcQueryRequest {
            block_reference: BlockReference::latest(),
            request: near_primitives::views::QueryRequest::ViewAccessKey {
                account_id: signer.account_id.clone(),
                public_key: signer.public_key.clone(),
            },
        })
        .await?;

    let current_nonce = match access_key_query_response.kind {
        QueryResponseKind::AccessKey(access_key) => access_key.nonce,
        _ => Err("failed to extract current nonce")?,
    };

    let transaction = Transaction {
        signer_id: signer.account_id.clone(),
        public_key: signer.public_key.clone(),
        nonce: current_nonce + 1,
        receiver_id: env::var("CONTRACT_ACCOUNT_ID").unwrap().parse()?,
        block_hash: access_key_query_response.block_hash,
        actions: vec![Action::FunctionCall(FunctionCallAction {
            method_name: "rate".to_string(),
            args: json!({
                // "block_number":123.0,
                // "bloom": logs.1
            })
            .to_string()
            .into_bytes(),
            gas: 100_000_000_000_000, // 100 TeraGas
            deposit: 0,
        })],
    };

    // let response = client.call(request).await;
    // println!("{:?}", response);

    Ok(())
}

async fn test(client: JsonRpcClient) -> Result<(), Box<dyn std::error::Error>> {
    let request = methods::query::RpcQueryRequest {
        block_reference: BlockReference::Finality(Finality::Final),
        request: QueryRequest::CallFunction {
            account_id: "dev-1669570531644-39751926390237".parse()?,
            method_name: "test".to_string(),
            args: FunctionArgs::from(vec![]),
        },
    };

    let response = client.call(request).await?;
    match response.kind {
        QueryResponseKind::CallResult(result) => {
            println!("{:?}", std::str::from_utf8(&result.result).unwrap());
        }
        _ => {}
    }

    Ok(())
}

async fn get_logs() -> (u64, [u8; 256]) {
    let transport = web3::transports::WebSocket::new(
        // ws for testnet
        "wss://eth-goerli.g.alchemy.com/v2/fq5FsW3IggL1giodxIhWRqW-er0MpDbi",
    )
    .await
    .unwrap();
    let web3 = web3::Web3::new(transport);
    let block_number = web3.eth().block_number().await.unwrap();

    let block = web3
        .eth()
        .block(BlockId::Number(BlockNumber::Latest))
        .await
        .unwrap();
    let logs_bloom = block.unwrap().logs_bloom.unwrap();
    (block_number.0[0], logs_bloom.0)
}

async fn validate() -> web3::Result<()> {
    let args: Vec<String> = env::args().collect();
    // mock topic
    let topic_1 = hex!("3b874d464775b5082b95c98fb5f815494cc129e32c4e8a07a0bb98e710f8c25c");

    let block_number = &args[1];
    let block_number = BlockId::Number(BlockNumber::Number(
        U64::from_dec_str(&block_number).unwrap(),
    ));

    let contract_address = &args[2];
    let normalized_address = H160::from_str(contract_address).unwrap();

    let transport = web3::transports::WebSocket::new(
        // ws for testnet
        "wss://eth-goerli.g.alchemy.com/v2/fq5FsW3IggL1giodxIhWRqW-er0MpDbi",
    )
    .await?;
    let web3 = web3::Web3::new(transport);

    let block = web3.eth().block(block_number).await?;
    let logs_bloom = block.unwrap().logs_bloom.unwrap();

    let is_valid = logs_bloom.contains_input(BloomInput::Hash(&signing::keccak256(
        &signing::keccak256("Locked(bytes32)".as_bytes()),
    ))) & logs_bloom.contains_input(BloomInput::Hash(&signing::keccak256(
        normalized_address.as_bytes(),
    ))) & logs_bloom.contains_input(BloomInput::Raw(&topic_1));

    println!("Validated: {}", is_valid);

    Ok(())
}
