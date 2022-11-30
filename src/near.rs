use std::env;

use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_primitives::{
    borsh::BorshSerialize,
    transaction::{Action, FunctionCallAction, Transaction},
    types::{BlockReference, Finality, FunctionArgs},
    views::QueryRequest,
};
use serde_json::json;

use crate::{
    eth,
    utils::{Bloom, BloomRequest},
};

pub fn setup_client(connection_string: &str) -> JsonRpcClient {
    JsonRpcClient::connect(connection_string)
}

pub async fn insert_filter(
    client: &JsonRpcClient,
    block_number: u64,
    filter: Bloom,
) -> Result<(), Box<dyn std::error::Error>> {
    let request = BloomRequest {
        block_number,
        logs: near_primitives::serialize::to_base64(&filter.logs),
    };

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
            method_name: "insert_filter".to_string(),
            args: json!({ "request": request }).to_string().into_bytes(),
            gas: 100_000_000_000_000, // 100 TeraGas
            deposit: 0,
        })],
    };

    let request = methods::broadcast_tx_async::RpcBroadcastTxAsyncRequest {
        signed_transaction: transaction.sign(&signer),
    };

    let tx_hash = client.call(request).await?;
    println!("tx hash: {}", tx_hash);

    Ok(())
}

pub async fn test(client: &JsonRpcClient) -> Result<(), Box<dyn std::error::Error>> {
    let request = methods::query::RpcQueryRequest {
        block_reference: BlockReference::Finality(Finality::Final),
        request: QueryRequest::CallFunction {
            account_id: "dev-1669574080300-19372117431608".parse()?,
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
